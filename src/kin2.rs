use syunit::*;

/// Helper inaccuracy
const ROOT_EPSILON : f32 = -1.0e-4;

/// Calculates the time required to travel a certain relative distance (`rel_dist`) with the given starting velocity (`velocity_0`) and an `acceleration`
/// 
/// ### TFD - Equations
/// 
/// This is the 2-dimensional variant of the time for distance equations. It is a lot cheeper to calculate than [crate::kin3::time_for_distance]
pub fn time_for_distance<U : UnitSet>(rel_dist : U::Distance, velocity_0 : U::Velocity, acceleration : U::Acceleration) -> (U::Time, U::Time) {
    let p = velocity_0 / acceleration;
    let q : f32 = 2.0 * rel_dist.into() / acceleration.into(); 

    let inner = p.into().powi(2) + q;
    let mut root = U::Time::new(inner.sqrt());

    // Make root `Time::ZERO`, if `inner` is just slightly negative (root returns `NaN` even for very small negative numbers)
    if inner.is_sign_negative() & (inner > ROOT_EPSILON) {
        root = U::Time::ZERO;
    }

    ( -p + root, -p - root )
}

// No starting velocity
    /// A cheeper version of [time_for_distance] when no starting velocity `velocity_0` is present
    pub fn time_for_distance_no_vel0<U : UnitSet>(rel_dist : U::Distance, acceleration : U::Acceleration) -> U::Time {
        U::Time::new((2.0 * rel_dist.into() / acceleration.into()).sqrt())
    }

    /// Exit velocity when moving a distance `rel_dist` from a zero-velocity state with the acceleration `acceleration`
    pub fn velocity_for_distance_no_vel0<U : UnitSet>(rel_dist : U::Distance, acceleration : U::Acceleration) -> U::Velocity {
        U::Velocity::new((2.0 * rel_dist.into() * acceleration.into()).sqrt())
    }

    /// The acceleration required to exit a certain distance `rel_dist` with the given velocity `velocity_exit`, starting wiithout any velocity
    pub fn acceleration_required_no_vel0<U : UnitSet>(rel_dist : U::Distance, velocity_exit : U::Velocity) -> U::Acceleration {
        U::Acceleration::new(velocity_exit.into() * velocity_exit.into() / 2.0 / rel_dist.into())
    }
// 

// With starting velocity
    /// Returns the time it takes an object to move a distance `rel_dist` with the starting velocity (`velocity_0`) and exit velocity (`velocity_exit`)
    pub fn time_for_distance_no_accel<U : UnitSet>(rel_dist : U::Distance, velocity_0 : U::Velocity, velocity_exit : U::Velocity) -> U::Time {
        rel_dist * 2.0 / (velocity_0 + velocity_exit)
    }
// 