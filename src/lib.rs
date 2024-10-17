#![doc = include_str!("../README.md")]
#![crate_name = "sykin"]
#![no_std]

// Rules
#![deny(missing_docs)]

// ####################
// #    SUBMODULES    #
// ####################
    /// Kinematic formulas for only distance and velocity
    pub mod kin1;

    /// Kinematic formulas for distance, velocity and acceleration
    pub mod kin2;

    /// Kinematic formulas for distance, velocity, acceleration and jolt
    pub mod kin3;
// 