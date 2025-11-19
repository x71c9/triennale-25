# demo1_single_move.py — move one motor to a fixed target
import sys, time
from cable_robot import CABLE_ROBOT, list_com_ports

# -------- User-editable constants --------
PORT            = "/dev/ttyUSB0"    # COM port
ADDR            = 3          # motor address
TARGET_MM       = 2000       # linear target distance (mm), payout direction is positive, distance value is clamped between 0mm and +5496mm.
SPEED_RPM       = 1000.0     # target speed for all axes. Max Speed is 1500 RPM. Recommended speed is 1000 RPM.
ACC_RPMS        = 100.0      # accel/decel for all axes. Max Accel is 150 RPM/s. Recommended accel is 100 RPM/s.
# -----------------------------------------

# 2025-11-10 We tested the lowest position for M1 and M2 to be 4800mm, and for M3 and M4 to be 5300mm. We also tested at 1000 RPM

GEARBOX_RATIO     = 70
DRUM_CIRCUMFERENCE = 769.5  # mm

def demo_move_single_motor_with_wait(robot:CABLE_ROBOT, addr:int, target_mm:float, speed_rpm:float, acc_rpms:float):
        # Convert linear distance to motor degrees
        target_deg = CABLE_ROBOT.mm_to_motor_deg(target_mm, DRUM_CIRCUMFERENCE, GEARBOX_RATIO)

        # Send absolute target to motor
        robot.set_motor_target(addr, target_deg, speed_rpm, acc_rpms)

        # Poll position/arrived flag until move complete
        last = 0
        while True:
            # Read current position
            pos_deg = robot.read_motor_position(addr)
            pos_mm = CABLE_ROBOT.motor_deg_to_mm(pos_deg, DRUM_CIRCUMFERENCE, GEARBOX_RATIO)
            # Read 'arrived' flag
            arrived = robot.read_motor_arrived_flag(addr)
            # Print status on same console line
            line = f"addr {addr}: {pos_deg:10.1f} deg {pos_mm:10.1f} mm  ({'stopped' if arrived else 'moving'})"
            pad = " " * max(0, last - len(line))
            sys.stdout.write("\r" + line + pad)
            sys.stdout.flush()
            last = len(line)
            if arrived:
                break
            time.sleep(0.05)

        print("\nMovement to {target_mm} mm complete.")

def main():

    print("Available COM ports:", list_com_ports())
    robot = CABLE_ROBOT(PORT)

    try:
        if not robot.probe_motor(ADDR):
            print(f"Motor {ADDR} not responding. Abort.")
            return
        # Move to target position and wait until done
        demo_move_single_motor_with_wait(robot, ADDR, TARGET_MM, SPEED_RPM, ACC_RPMS)

        # # Move back to zero and wait until done
        # demo_move_single_motor_with_wait(robot, ADDR, 0.0, SPEED_RPM, ACC_RPMS)

    finally:
        robot.close()

if __name__ == "__main__":
    main()
