# demo4_multi_sync.py — synchronized multi-axis move (random lower position and return to top)
import sys, time, random
from cable_robot import CABLE_ROBOT, list_com_ports

# -------- User-editable constants --------
PORT               = "/dev/ttyUSB0"
ADDRESSES_TO_MOVE   = [1, 2, 3, 4]
TOP_POSITION_MM   = [1000, 1000, 1000, 1000]  # per-motor top positions (mm)
MAX_RANDOM_DIST_MM       = [4800, 4800, 5300, 5300]  # per-motor targets (mm)
MIN_RANDOM_DIST_MM       = [2800, 2800, 3300, 3300]  # per-motor targets (mm),
MAX_SPEED_RPM  = 800.0     # Absolute Max Speed is 1500 RPM. Recommended speed is 1000 RPM.
MIN_SPEED_RPM  = 400.0     # Absolute Max Speed is 1500 RPM. Recommended speed is 1000 RPM.
ACC_RPMS   = 100.0      # accel/decel for the longest axis. Max Accel is 150 RPM/s. Recommended accel is 100 RPM/s.
NUM_OF_CYCLES      = 50          # number of random moves to perform
# -----------------------------------------

GEARBOX_RATIO      = 70
DRUM_CIRCUMFERENCE = 769.5    # mm

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

def sync_move(robot: CABLE_ROBOT, addrs, target_mm_list, vmax_fast_rpm, acc_fast_rpms):
    # read current absolute positions (deg)
    cur_deg = [robot.read_motor_position(a) for a in addrs]
    # build absolute targets (deg)
    tgt_deg = [CABLE_ROBOT.mm_to_motor_deg(mm, DRUM_CIRCUMFERENCE, GEARBOX_RATIO) for mm in target_mm_list]
    # distances to travel (deg)
    dist_deg = [abs(td - cd) for td, cd in zip(tgt_deg, cur_deg)]
    # compute per-axis (vmax, acc) to finish together
    T_star, params = CABLE_ROBOT.solve_sync_params(dist_deg, vmax_fast_rpm, acc_fast_rpms)
    # issue commands
    for a, td, (v, acc) in zip(addrs, tgt_deg, params):
        robot.set_motor_target(a, td, speed_rpm=v, acc_rpm_s=acc)
    return T_star

def main():
    print("Available COM ports:", list_com_ports())
    robot = CABLE_ROBOT(PORT)
    try:
        addrs = [a for a in ADDRESSES_TO_MOVE if robot.probe_motor(a)]

        # All motors must be present
        if addrs != ADDRESSES_TO_MOVE:
            print("Not all motors online. Abort.")
            return
        print("Online:", addrs)

        
        for cycle_i in range(NUM_OF_CYCLES):
            print(f"=== Cycle {cycle_i + 1} of {NUM_OF_CYCLES} ===")

            # Random targets within specified ranges
            dists = []
            for i, a in enumerate(addrs):
                mm = random.uniform(MIN_RANDOM_DIST_MM[i], MAX_RANDOM_DIST_MM[i])
                dists.append(mm)
            print("Random target distances (mm):", dists)
            # Random speed between min and max
            speed_rpm = random.uniform(MIN_SPEED_RPM, MAX_SPEED_RPM)
            print(f"Random speed: {speed_rpm:.1f} RPM")
     
            # Go Down to Random Position
            T1 = sync_move(robot, addrs, dists, speed_rpm, ACC_RPMS)
            print(f"Synchronized movement duration T* ≈ {T1:.2f}s")
            poll_until(robot, addrs)

            time.sleep(2.0)

            # Go Back Up to Top Position
            T2 = sync_move(robot, addrs, TOP_POSITION_MM, speed_rpm, ACC_RPMS)
            print(f"Synchronized movement duration T* ≈ {T2:.2f}s")
            poll_until(robot, addrs)

        print(f"Random sync moves completed for {NUM_OF_CYCLES} cycles.")

    finally:
        robot.close()

if __name__ == "__main__":
    main()
