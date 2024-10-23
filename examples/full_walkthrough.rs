use syunit::*;
use syunit::metric::*;

pub fn prompt<T : core::str::FromStr + Copy>(msg : &str, default_opt : Option<T>) -> T {
    let mut input_string = String::new();

    loop {
        print!("{}", msg);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::io::stdin().read_line(&mut input_string).unwrap();
    
        if let Ok(res) = T::from_str(input_string.trim()) {
            return res;
        } else {
            if let Some(default) = default_opt {
                return default;
            } else {
                println!("Bad input for type!");
            }
        }
    }
}

fn main() {
    // Kin3
    let jolt : MMPerSecond3 = prompt("Enter jolt: ", Some(MMPerSecond3(1.0)));
    let accel : MMPerSecond2 = prompt("Enter acceleration: ", Some(MMPerSecond2(1.0)));
    let vel : MMPerSecond = prompt("Enter velocity: ", Some(MMPerSecond(1.0)));
    let dist : Millimeters = prompt("Enter relative distance: ", Some(Millimeters(1.0)));

    println!();
    println!("Result: {}", sykin::kin3::time_for_distance::<MetricMM>(dist, vel, accel, jolt));

    // TODO: Add more calculations
}