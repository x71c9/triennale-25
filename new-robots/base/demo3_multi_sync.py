# demo3_multi_sync.py — synchronized multi-axis move (different distances, same end time)
import sys, time
from cable_robot import CABLE_ROBOT, list_com_ports

# -------- User-editable constants --------
PORT               = "/dev/ttyUSB0"
ADDRESSES_TO_TRY   = (1, 2, 3, 4)
# TEST_DIST_MM       = [4800, 4800, 5300, 5300]  # per-motor targets (mm), max distance is 
TEST_DIST_MM       = [1000, 1000, 1000, 1000]  # per-motor targets (mm), max distance is 
FASTEST_SPEED_RPM  = 800.0     # limits for the longest axis. Absolute Max Speed is 1500 RPM. Recommended speed is 1000 RPM.
FASTEST_ACC_RPMS   = 100.0      # accel/decel for the longest axis. Max Accel is 150 RPM/s. Recommended accel is 100 RPM/s.
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
        addrs = [a for a in ADDRESSES_TO_TRY if robot.probe_motor(a)]
        if not addrs:
            print("No motors online.")
            return
        print("Online:", addrs)

        # Trim or cycle TEST_DIST_MM to match number of online motors
        k = len(addrs)
        dists = (TEST_DIST_MM * ((k + len(TEST_DIST_MM) - 1) // len(TEST_DIST_MM)))[:k]
        print("Sync targets (mm):", dists)

        T = sync_move(robot, addrs, dists, FASTEST_SPEED_RPM, FASTEST_ACC_RPMS)
        print(f"Nominal synchronized duration T* ≈ {T:.2f}s")
        poll_until(robot, addrs)

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
