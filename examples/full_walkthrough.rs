use syunit::*;

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
    let jolt : Jolt = prompt("Enter jolt: ", Some(Jolt(1.0)));
    let accel : Acceleration = prompt("Enter acceleration: ", Some(Acceleration(1.0)));
    let vel : Velocity = prompt("Enter velocity: ", Some(Velocity(1.0)));
    let dist : RelDist = prompt("Enter relative distance: ", Some(RelDist(1.0)));

    println!();
    println!("Result: {}", sykin::kin3::time_for_distance(dist, vel, accel, jolt));
}