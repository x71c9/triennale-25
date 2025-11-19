# demo3_multi_sync.py — synchronized multi-axis move (different distances, same end time)
import sys, time
from cable_robot import CABLE_ROBOT, list_com_ports

# -------- User-editable constants --------
PORT               = "/dev/ttyUSB0"
ADDRESSES_TO_TRY   = (1, 2, 3, 4)
# -----------------------------------------

GEARBOX_RATIO      = 70
DRUM_CIRCUMFERENCE = 769.5    # mm

def print_status_flags(flags: dict):
    """
    Pretty-print motor status flags returned by read_motor_status().
    """
    descriptions = {
        "Ens_TF": "Motor enabled (power ON)",
        "Prf_TF": "Target position reached",
        "Cgi_TF": "Rotation detection active",
        "Cgp_TF": "Rotation protection active",
        "Esi_LF": "Left limit switch triggered",
        "Esi_RF": "Right limit switch triggered",
        "Oac_TF": "Power-loss flag (set after reset)"
    }

    print(f"\nMotor Status (raw=0x{flags['raw_byte']:02X}):")
    for key, desc in descriptions.items():
        state = "✅ ON " if flags.get(key, False) else "❌ OFF"
        print(f"  {state:<7} {key:<7} — {desc}")


def main():
    print("Available COM ports:", list_com_ports())
    robot = CABLE_ROBOT(PORT)
    try:
        addrs = [a for a in ADDRESSES_TO_TRY if robot.probe_motor(a)]
        if not addrs:
            print("No motors online.")
            return
        print("Online:", addrs)

        for a in addrs:
            pos = robot.read_motor_position(a)
            arrived = robot.read_motor_arrived_flag(a)
            status = "stopped" if arrived else "moving"
            bus_voltage = robot.read_bus_voltage(a)
            print(f"Motor {a} position: {pos:.1f} deg ({CABLE_ROBOT.motor_deg_to_mm(pos, DRUM_CIRCUMFERENCE, GEARBOX_RATIO):.1f} mm), status: {status}, bus voltage: {bus_voltage:.1f} V")
            flags = robot.read_motor_status(a)
            print_status_flags(flags)

        # # Trim or cycle TEST_DIST_MM to match number of online motors
        # k = len(addrs)
        # dists = (TEST_DIST_MM * ((k + len(TEST_DIST_MM) - 1) // len(TEST_DIST_MM)))[:k]
        # print("Sync targets (mm):", dists)

        # T = sync_move(robot, addrs, dists, FASTEST_SPEED_RPM, FASTEST_ACC_RPMS)
        # print(f"Nominal synchronized duration T* ≈ {T:.2f}s")
        # poll_until(robot, addrs)

        # time.sleep(2.0)

        # zero_mm = [0.0] * k
        # T2 = sync_move(robot, addrs, zero_mm, FASTEST_SPEED_RPM, FASTEST_ACC_RPMS)
        # print(f"Return sync duration T* ≈ {T2:.2f}s")
        # poll_until(robot, addrs)

        # print("Synchronized cycle complete.")
    finally:
        robot.close()

if __name__ == "__main__":
    main()
