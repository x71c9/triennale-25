# zdt_y42.py  — ZDT Y42 (firmware X) minimal RS-485 driver (free protocol)
import time, struct
import serial
from serial.tools import list_ports

CHECKSUM = 0x6B
BAUD = 115200
SER_TIMEOUT = 0.2

# Device constraints (soft caps)
MAX_SPEED_RPM = 1500.0
MAX_ACC_RPMS = 150.0
POS_MIN_DEG = 0 # Negative value will allow moving in the other direction. -180000.0
POS_MAX_DEG = 180000.0

def list_com_ports():
    return [p.device for p in list_ports.comports()]

def clamp(x, lo, hi):
    return max(lo, min(hi, x))

class CABLE_ROBOT:
    """
    ZDT Y42 (firmware X) 'free protocol' (NOT Modbus).
    Frames: [addr, code, ..., 0x6B]. Big-endian for multi-byte fields.
    """
    def __init__(self, port:str, baud:int=BAUD, timeout:float=SER_TIMEOUT):
        self.ser = serial.Serial(port=port, baudrate=baud, parity=serial.PARITY_NONE,
                                 stopbits=serial.STOPBITS_ONE, bytesize=serial.EIGHTBITS,
                                 timeout=timeout, write_timeout=timeout)

    def close(self):
        if self.ser and self.ser.is_open:
            self.ser.close()

    # ---------- low-level ----------
    def _write(self, payload:bytes):
        self.ser.write(payload)
        self.ser.flush()
        time.sleep(0.003)  # dongle turn-around margin

    def _read_exact(self, n:int) -> bytes:
        data = self.ser.read(n)
        if len(data) != n:
            raise TimeoutError(f"Read timeout: expected {n} bytes, got {len(data)}")
        return data

    def _try_expect_ack(self, addr:int, code:int) -> bool:
        try:
            resp = self._read_exact(4)
            return (resp[0] == addr and resp[1] == code and resp[-1] == CHECKSUM and resp[2] in (0x02, 0x9F))
        except TimeoutError:
            return False

    # ---------- convenience ----------
    def probe_motor(self, addr:int) -> bool:
        """Quick ping via status (0x3A)."""
        try:
            self._write(bytes([addr & 0xFF, 0x3A, CHECKSUM]))
            resp = self._read_exact(4)
            return (resp[0] == addr and resp[1] == 0x3A and resp[-1] == CHECKSUM)
        except TimeoutError:
            return False

    # ---------- public API ----------
    def set_motor_target(self, motor_addr:int, position_deg:float, speed_rpm:float, acc_rpm_s:float):
        """
        Absolute move (trapezoidal). Soft limits+caps applied.
        Encodings (firmware X):
          accel/decel: RPM/s (uint16)
          vmax: 0.1 RPM (uint16)
          position: 0.1° (uint32)
        """
        # Clamp position to respect motor limits, now allowing negative positions
        position_deg = clamp(position_deg, POS_MIN_DEG, POS_MAX_DEG)
        speed_rpm    = clamp(speed_rpm, 0.0, MAX_SPEED_RPM)
        acc_rpm_s    = clamp(acc_rpm_s, 0.0, MAX_ACC_RPMS)

        direction = 0x01 if position_deg < 0 else 0x00
        accel = int(round(acc_rpm_s))
        decel = int(round(acc_rpm_s))
        vmax_0p1rpm = int(round(speed_rpm * 10.0))
        pos_0p1deg  = int(round(abs(position_deg) * 10.0))
        motion_mode = 0x01  # absolute
        sync_flag   = 0x00

        frame = bytearray()
        frame += bytes([motor_addr & 0xFF, 0xFD, direction])
        frame += struct.pack(">H", accel) + struct.pack(">H", decel)
        frame += struct.pack(">H", vmax_0p1rpm) + struct.pack(">I", pos_0p1deg)
        frame += bytes([motion_mode, sync_flag, CHECKSUM])
        self._write(frame)

        if not self._try_expect_ack(motor_addr, 0xFD):
            # verify device is alive
            if not self.probe_motor(motor_addr):
                raise TimeoutError(f"Motor {motor_addr} did not ACK and is not responding to status; check wiring/power.")

    def set_motor_enable(self, motor_addr: int, enable: bool):
        """Enable or disable a motor (lock/unlock shaft)."""
        enable_status = 0x01 if enable else 0x00
        sync_flag = 0x00
        frame = bytes([motor_addr & 0xFF, 0xF3, 0xAB, enable_status, sync_flag, CHECKSUM])
        self._write(frame)
        if not self._try_expect_ack(motor_addr, 0xF3):
            # This is not critical, so just a warning
            print(f"Warning: Motor {motor_addr} did not ACK enable command.")

    def read_motor_position(self, motor_addr:int) -> float:
        """Return absolute position in degrees (float)."""
        self._write(bytes([motor_addr & 0xFF, 0x36, CHECKSUM]))
        resp = self._read_exact(8)
        if resp[0] != motor_addr or resp[1] != 0x36 or resp[-1] != CHECKSUM:
            raise RuntimeError(f"Bad read-pos resp: {resp.hex()}")
        # sign = -1.0 if resp[2] == 0x00 else 1.0
        sign = 1.0 if resp[2] == 0x00 else -1.0 # This is reversed for the cable robot such that payout is positive
        raw = struct.unpack(">I", resp[3:7])[0]
        return sign * (raw / 10.0)

    def read_motor_arrived_flag(self, motor_addr:int) -> bool:
        """True if 'position reached' flag is set."""
        self._write(bytes([motor_addr & 0xFF, 0x3A, CHECKSUM]))
        resp = self._read_exact(4)
        if resp[0] != motor_addr or resp[1] != 0x3A or resp[-1] != CHECKSUM:
            raise RuntimeError(f"Bad status resp: {resp.hex()}")
        return bool(resp[2] & 0x02)

    def read_bus_voltage(self, motor_addr:int) -> float:
        """
        Read bus voltage (VBus) from motor.
        Returns voltage in volts (float).

        Protocol:
          Host → Motor:    [Addr, 0x24, 0x6B]
          Motor → Host:    [Addr, 0x24, VBus_H, VBus_L, 0x6B]
        VBus: 16-bit unsigned integer, in mV (after reverse-protection diode)
        """
        self._write(bytes([motor_addr & 0xFF, 0x24, CHECKSUM]))
        resp = self._read_exact(5)
        if resp[0] != motor_addr or resp[1] != 0x24 or resp[-1] != CHECKSUM:
            raise RuntimeError(f"Bad bus voltage resp: {resp.hex()}")

        vbus_mV = struct.unpack(">H", resp[2:4])[0]  # big-endian, 2 bytes
        return vbus_mV / 1000.0  # convert to volts

    def read_motor_status(self, motor_addr:int) -> dict:
        """
        Read motor status flags (0x3A command).

        Protocol:
          Host → Motor: [Addr, 0x3A, 0x6B]
          Motor → Host: [Addr, 0x3A, status_byte, 0x6B]

        Bit mapping (bit7 → bit0):
          7: Oac_TF (Power-loss flag; set after reset)
          6: reserved
          5: Esi_RF (Right limit triggered)
          4: Esi_LF (Left limit triggered)
          3: Cgp_TF (Rotation protection)
          2: Cgi_TF (Rotation detection)
          1: Prf_TF (Position reached)
          0: Ens_TF (Motor enabled)
        """
        self._write(bytes([motor_addr & 0xFF, 0x3A, CHECKSUM]))
        resp = self._read_exact(4)
        if resp[0] != motor_addr or resp[1] != 0x3A or resp[-1] != CHECKSUM:
            raise RuntimeError(f"Bad motor status resp: {resp.hex()}")

        status = resp[2]
        flags = {
            "Ens_TF": bool(status & 0x01),
            "Prf_TF": bool(status & 0x02),
            "Cgi_TF": bool(status & 0x04),
            "Cgp_TF": bool(status & 0x08),
            "Esi_LF": bool(status & 0x10),
            "Esi_RF": bool(status & 0x20),
            "Oac_TF": bool(status & 0x80),  # Power-loss flag, not overcurrent
            "raw_byte": status
        }
        return flags

    def clear_motor_position(self, motor_addr:int) -> bool:
        """
        Clear the current position angle (set current position = 0°).

        Protocol:
          Host → Motor: [Addr, 0x0A, 0x6D, 0x6B]
          Motor → Host: [Addr, 0x0A, ret, 0x6B]

        Returns:
          True if success. ret_code = 0x02 for success; 0xE2/0xEE for error/warning.
        Raises:
          RuntimeError if malformed response.
        """
        self._write(bytes([motor_addr & 0xFF, 0x0A, 0x6D, CHECKSUM]))
        resp = self._read_exact(4)
        if resp[0] != motor_addr or resp[1] != 0x0A or resp[-1] != CHECKSUM:
            raise RuntimeError(f"Bad clear-position resp: {resp.hex()}")

        ret_code = resp[2]
        if ret_code != 0x02:
            print(f"⚠️ Clear position returned code 0x{ret_code:02X} (see section 4 for meaning).")
        else:
            return True

    # ---------- helpers: kinematics & sync ----------
    @staticmethod
    def mm_to_motor_deg(distance_mm:float, drum_circ_mm:float, gearbox_ratio:float) -> float:
        """Linear mm at drum → motor shaft degrees (absolute)."""
        return (distance_mm / drum_circ_mm) * 360.0 * gearbox_ratio

    @staticmethod
    def motor_deg_to_mm(deg:float, drum_circ_mm:float, gearbox_ratio:float) -> float:
        return (deg / (360.0 * gearbox_ratio)) * drum_circ_mm

    @staticmethod
    def rpm_to_degps(v_rpm:float) -> float:
        return v_rpm * 360.0 / 60.0

    @staticmethod
    def degps_to_rpm(v_degps:float) -> float:
        return v_degps * 60.0 / 360.0

    @staticmethod
    def rpms_to_degps2(a_rpms:float) -> float:
        return a_rpms * 360.0 / 60.0  # deg/s^2

    @staticmethod
    def time_for_trapezoid(distance_deg:float, vmax_rpm:float, acc_rpms:float) -> float:
        """Total time for symmetric accel/decel trapezoid (deg units)."""
        v = CABLE_ROBOT.rpm_to_degps(vmax_rpm)
        a = CABLE_ROBOT.rpms_to_degps2(acc_rpms)
        t_acc = v / a
        d_accdec = v*v / a
        if distance_deg >= d_accdec:
            t = 2*t_acc + (distance_deg - d_accdec) / v
        else:
            # triangular (peak lower than vmax)
            t = 2.0 * (distance_deg / a) ** 0.5
        return t

    @staticmethod
    def solve_sync_params(distances_deg:list, vmax_fast_rpm:float, acc_fast_rpms:float):
        """
        Compute per-axis (vmax_rpm_i, acc_rpms_i) so that all axes finish together.
        Strategy:
          - Determine T* from the largest distance using (vmax_fast, acc_fast).
          - For each smaller distance, use *triangular* profile with a_i = 4 d_i / T*^2,
            v_i = a_i * T*/2. Caps are applied if needed.
        Returns: (T_star, [(vmax_rpm_i, acc_rpms_i), ...])
        """
        d_max = max(distances_deg)
        T_star = CABLE_ROBOT.time_for_trapezoid(d_max, vmax_fast_rpm, acc_fast_rpms)

        params = []
        for d in distances_deg:
            if d == 0:
                params.append((0.0, 0.0))
                continue
            a_i_degps2 = 4.0 * d / (T_star*T_star)
            v_i_degps  = 0.5 * a_i_degps2 * T_star
            # convert to rpm units and cap
            v_i_rpm = CABLE_ROBOT.degps_to_rpm(v_i_degps)
            a_i_rpms = CABLE_ROBOT.degps_to_rpm(a_i_degps2)  # same unit conversion
            v_i_rpm = clamp(v_i_rpm, 0.0, min(MAX_SPEED_RPM, vmax_fast_rpm))
            a_i_rpms = clamp(a_i_rpms, 0.0, min(MAX_ACC_RPMS, acc_fast_rpms))
            params.append((v_i_rpm, a_i_rpms))
        return T_star, params
