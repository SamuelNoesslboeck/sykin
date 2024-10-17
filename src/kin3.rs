use syunit::*;

pub fn time_for_distance(rel_dist : RelDist, velocity_0 : Velocity, acceleration_0 : Acceleration, jolt : Jolt) -> Time {
    Time::ZERO
}

/// The time it takes a motor to move the distance `rel_dist` with the given maximum `jolt`
pub fn time_from_distance_only_jolt(rel_dist : RelDist, jolt : Jolt) -> Time {
    Time((6.0 * rel_dist.0 / jolt.0).cbrt())
}