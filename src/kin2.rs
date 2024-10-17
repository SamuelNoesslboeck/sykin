use syunit::*;

/// Helper inaccuracy
const ROOT_EPSILON : f32 = -1.0e-4;

/// Calculates the time required to travel a certain relative distance (`rel_dist`) with the given starting velocity (`velocity_0`) and an `acceleration`
/// 
/// ### TFD - Equations
/// 
/// This is the 2-dimensional variant of the time for distance equations. It is a lot cheeper to calculate than [kin3::time_for_distance]
pub fn time_for_distance(rel_dist : RelDist, velocity_0 : Velocity, acceleration : Acceleration) -> (Time, Time) {
    let p = velocity_0 / acceleration;
    let q = 2.0 * rel_dist.0 / acceleration.0; 

    let inner = p.0.powi(2) + q;
    let mut root = Time(inner.sqrt());

    // Make root `Time::ZERO`, if `inner` is just slightly negative (root returns `NaN` even for very small negative numbers)
    if inner.is_sign_negative() & (inner > ROOT_EPSILON) {
        root = Time::ZERO;
    }

    ( -p + root, -p - root )
}

/// A cheeper version of [time_for_distance] when no starting velocity `velocity_0` is present
pub fn time_for_distance_no_vel0(rel_dist : RelDist, acceleration : Acceleration) -> Time {
    Time((2.0 * rel_dist.0 / acceleration.0).sqrt())
}

/// Exit velocity when moving a distance `rel_dist` from a zero-velocity state with the acceleration `acceleration`
pub fn velocity_for_distance_no_vel0(rel_dist : RelDist, acceleration : Acceleration) -> Velocity {
    Velocity((2.0 * rel_dist.0 * acceleration.0).sqrt())
}

/// The acceleration required to exit a certain distance `rel_dist` with the given velocity `velocity_exit`, starting wiithout any velocity
pub fn acceleration_required_no_vel0(rel_dist : RelDist, velocity_exit : Velocity) -> Acceleration {
    Acceleration(velocity_exit.0 * velocity_exit.0 / 2.0 / rel_dist.0)
}