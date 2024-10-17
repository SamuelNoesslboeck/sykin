use syunit::*;

/// Calculates the time required to travel a certain relative distance (`rel_dist`) with the given `velocity`
/// 
/// ### TFD - Equations
/// 
/// This is the 1-dimensional variant of the time for distance equations, it exists rather for the sake of completeness than for it's complexity
#[inline]
pub fn time_for_distance(rel_dist : RelDist, velocity : Velocity) -> Time {
    rel_dist / velocity
}