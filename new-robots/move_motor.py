#!/usr/bin/env python3
# move_motor.py — parameterized motor movement script
import sys
import time
import argparse
import os

# Add the current directory to Python path to import base module
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from base.cable_robot import CABLE_ROBOT, list_com_ports

GEARBOX_RATIO = 70
DRUM_CIRCUMFERENCE = 769.5  # mm
DEFAULT_PORT = "/dev/ttyUSB0"
DEFAULT_SPEED_RPM = 1000.0
DEFAULT_ACC_RPMS = 100.0

def _move_single_motor_with_wait(robot: CABLE_ROBOT, addr: int, target_mm: float, speed_rpm: float, acc_rpms: float):
    target_deg = CABLE_ROBOT.mm_to_motor_deg(target_mm, DRUM_CIRCUMFERENCE, GEARBOX_RATIO)
    robot.set_motor_target(addr, target_deg, speed_rpm, acc_rpms)
    
    last = 0
    while True:
        pos_deg = robot.read_motor_position(addr)
        pos_mm = CABLE_ROBOT.motor_deg_to_mm(pos_deg, DRUM_CIRCUMFERENCE, GEARBOX_RATIO)
        arrived = robot.read_motor_arrived_flag(addr)
        
        line = f"addr {addr}: {pos_deg:10.1f} deg {pos_mm:10.1f} mm  ({'stopped' if arrived else 'moving'})"
        pad = " " * max(0, last - len(line))
        sys.stdout.write("\r" + line + pad)
        sys.stdout.flush()
        last = len(line)
        
        if arrived:
            break
        time.sleep(0.05)
    
    print(f"\nMovement to {target_mm} mm complete.")

def main():
    parser = argparse.ArgumentParser(description="Move a single motor to specified position")
    parser.add_argument("--motor-id", type=int, required=True, help="Motor address (e.g., 1, 2, 3, 4)")
    parser.add_argument("--position", type=float, required=True, help="Target position in mm (0-5496)")
    parser.add_argument("--speed", type=float, default=DEFAULT_SPEED_RPM, help=f"Speed in RPM (max 1500, default: {DEFAULT_SPEED_RPM})")
    parser.add_argument("--port", type=str, default=DEFAULT_PORT, help=f"COM port (default: {DEFAULT_PORT})")
    parser.add_argument("--acceleration", type=float, default=DEFAULT_ACC_RPMS, help=f"Acceleration in RPM/s (default: {DEFAULT_ACC_RPMS})")
    
    args = parser.parse_args()
    
    # Validate inputs
    if args.position < 0 or args.position > 5496:
        print("Error: Position must be between 0 and 5496 mm")
        sys.exit(1)
    
    if args.speed <= 0 or args.speed > 1500:
        print("Error: Speed must be between 0 and 1500 RPM")
        sys.exit(1)
    
    if args.acceleration <= 0 or args.acceleration > 150:
        print("Error: Acceleration must be between 0 and 150 RPM/s")
        sys.exit(1)
    
    print("Available COM ports:", list_com_ports())
    robot = CABLE_ROBOT(args.port)
    
    try:
        if not robot.probe_motor(args.motor_id):
            print(f"Motor {args.motor_id} not responding. Abort.")
            return
        
        print(f"Moving motor {args.motor_id} to {args.position} mm at {args.speed} RPM...")
        _move_single_motor_with_wait(robot, args.motor_id, args.position, args.speed, args.acceleration)
        
    finally:
        robot.close()

if __name__ == "__main__":
    main()