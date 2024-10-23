# sykin

A library kinematic calculations using the [syunit-unit-system](https://github.com/SamuelNoesslboeck/syunit).

## Quick introduction

```rust
use syunit::prelude::*;
use sykin::prelude::*;

let distance = Millimeters(100.0);
let velocity = MMPerSecond(10.0); 

// Kin1 - Only velocity and distance
assert_eq!(kin1::time_for_distance::<MetricMM>(distance, velocity), Seconds(10.0));

// Kin2 - distance, velocity and acceleration
let distance = Millimeters(15.0);
let velocity = MMPerSecond(2.0); 
let acceleration = MMPerSecond2(2.0);

assert_eq!(kin2::time_for_distance::<MetricMM>(distance, velocity, acceleration), (Seconds(3.0), Seconds(-5.0)));

// ...
```