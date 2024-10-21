use syunit::*;

/// Calculates the time required to travel a certain relative distance (`rel_dist`) with the given starting velocity (`velocity_0`), starting acceleration (`acceleration_0`) and `jolt`
/// 
/// ### TFD - Equations
/// 
/// This is the 3-dimensional variant of the time for distance equations
pub fn time_for_distance<U : UnitSet>(rel_dist : U::Distance, velocity_0 : U::Velocity, acceleration_0 : U::Acceleration, jolt : U::Jolt) -> U::Time {
    // Values of full cubic formula
    let a = jolt.into() / 6.0;
    let b = acceleration_0.into() / 2.0;
    let c = velocity_0.into(); 
    let d = -rel_dist.into();

    // Creating depressed cubic
    let shift = b / (3.0*a);
    let p = (3.0*a*c - b*b) / (3.0*a*a);
    let q = (2.0*b*b*b - 9.0*a*b*c + 27.0*a*a*d) / (27.0*a*a*a);

    // Calculate the helper root 
    let root = (q*q / 4.0 + p*p*p / 27.0).sqrt();

    // Helper numbers
    let u_1 = -q/2.0 + root;
    let u_2 = -q/2.0 - root;

    // Return the result
    U::Time::from(u_1.cbrt() + u_2.cbrt() - shift)
}

/// The time it takes a motor to move the distance `rel_dist` with the given maximum `jolt` without any starting acceleration or velocity
pub fn time_for_distance_only_jolt<U : UnitSet>(rel_dist : U::Distance, jolt : U::Jolt) -> U::Time {
    U::Time::from((6.0 * rel_dist.into() / jolt.into()).cbrt())
}

/// The exit acceleration of an object after moving the distance `rel_dist` with the maximum `jolt` without any starting acceleration or velocity
pub fn acceleration_for_distance_only_jolt<U : UnitSet>(rel_dist : U::Distance, jolt : U::Jolt) -> U::Acceleration {
    U::Acceleration::from((6.0 * rel_dist.into() * jolt.into() * jolt.into()).cbrt())
} 