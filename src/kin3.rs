use syunit::*;

/// Calculates the time required to travel a certain relative distance (`rel_dist`) with the given starting velocity (`velocity_0`), starting acceleration (`acceleration_0`) and `jolt`
/// 
/// ### TFD - Equations
/// 
/// This is the 3-dimensional variant of the time for distance equations
pub fn time_for_distance(rel_dist : RelDist, velocity_0 : Velocity, acceleration_0 : Acceleration, jolt : Jolt) -> Time {
    Time::ZERO
}

/// The time it takes a motor to move the distance `rel_dist` with the given maximum `jolt` without any starting acceleration or velocity
pub fn time_for_distance_only_jolt(rel_dist : RelDist, jolt : Jolt) -> Time {
    Time((6.0 * rel_dist.0 / jolt.0).cbrt())
}

/// The exit acceleration of an object after moving the distance `rel_dist` with the maximum `jolt` without any starting acceleration or velocity
pub fn acceleration_for_distance_only_jolt(rel_dist : RelDist, jolt : Jolt) -> Acceleration {
    Acceleration((6.0 * rel_dist.0 * jolt.0 * jolt.0).cbrt())
} 