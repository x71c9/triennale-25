# demo3_multi_sync.py — synchronized multi-axis move (different distances, same end time)
import sys, time
from cable_robot import CABLE_ROBOT, list_com_ports

# -------- User-editable constants --------
PORT               = "/dev/ttyUSB0"
ADDRESSES_TO_TRY   = [2]
# -----------------------------------------

GEARBOX_RATIO      = 70
DRUM_CIRCUMFERENCE = 769.5    # mm

def read_print_positions(robot : CABLE_ROBOT, addr):
    pos = robot.read_motor_position(addr)
    arrived = robot.read_motor_arrived_flag(addr)
    status = "stopped" if arrived else "moving"
    print(f"Motor {addr} position: {pos:.1f} deg ({CABLE_ROBOT.motor_deg_to_mm(pos, DRUM_CIRCUMFERENCE, GEARBOX_RATIO):.1f} mm), status: {status}")

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
            read_print_positions(robot, a)
            print("Resetting positions to zero...")
            robot.clear_motor_position(a)
            time.sleep(0.5)  # wait for motors to process the command
            read_print_positions(robot, a)
            print("---------")

    finally:
        robot.close()

if __name__ == "__main__":
    main()
