#![doc = include_str!("../README.md")]
#![crate_name = "sykin"]
#![no_std]
// Rules
#![deny(missing_docs)]

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

    /// Lazy import of the library (mainly for testing/examples)
    pub mod prelude {
        pub use crate::kin1;
        pub use crate::kin2;
        pub use crate::kin3;
    }
// 