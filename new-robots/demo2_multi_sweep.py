# demo2_multi_sweep.py — auto-detect motors, go to 1000 mm, pause, return to 0
import sys, time
from cable_robot import CABLE_ROBOT, list_com_ports

# -------- User-editable constants --------
PORT               = "/dev/ttyUSB0"
ADDRESSES_TO_TRY   = (1, 2, 3, 4)
TARGET_MM          = 3800       # linear target distance (mm), payout direction is positive, distance value is clamped between 0mm and +5496mm.
SPEED_RPM          = 1000.0     # target speed for all axes. Max Speed is 1500 RPM. Recommended speed is 1000 RPM.
ACC_RPMS           = 100.0      # accel/decel for all axes. Max Accel is 150 RPM/s. Recommended accel is 100 RPM/s.
# -----------------------------------------

GEARBOX_RATIO      = 70
DRUM_CIRCUMFERENCE = 769.5  # mm

def poll_until(robot: CABLE_ROBOT, addrs):
    last = 0
    while True:
        all_done = True
        parts = []
        for a in addrs:
            pos = robot.read_motor_position(a)
            arrived = robot.read_motor_arrived_flag(a)
            parts.append(f"{a}={pos:8.1f} {'stopped' if arrived else 'moving'}")
            all_done &= arrived
        line = " | ".join(parts)
        sys.stdout.write("\r" + line + " " * max(0, last - len(line)))
        sys.stdout.flush()
        last = len(line)
        if all_done:
            break
        time.sleep(0.05)
    print()

def main():
    print("Available COM ports:", list_com_ports())
    robot = CABLE_ROBOT(PORT)
    try:
        # Auto-detect which motors are online
        addrs = [a for a in ADDRESSES_TO_TRY if robot.probe_motor(a)]
        if not addrs:
            print("No motors online.")
            return
        print("Online:", addrs)

        # Move all connected motors to target 
        tgt_deg = CABLE_ROBOT.mm_to_motor_deg(TARGET_MM, DRUM_CIRCUMFERENCE, GEARBOX_RATIO)
        for a in addrs:
            robot.set_motor_target(a, tgt_deg, SPEED_RPM, ACC_RPMS)
        poll_until(robot, addrs)

        # Pause 
        # time.sleep(4.0)

        # # Move all connected motors back to zero
        # for a in addrs:
        #     robot.set_motor_target(a, 0.0, SPEED_RPM, ACC_RPMS)
        # poll_until(robot, addrs)

        print("Sweep complete.")
    finally:
        robot.close()

if __name__ == "__main__":
    main()
