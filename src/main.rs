use frc::prelude::*;

mod can;
mod drivetrain;

fn main() {
    let mut state: State = State::new().unwrap();
    state.add_systems(Init, hello_robot);
    state.run();
}

fn hello_robot() {
    println!("Hello, Robot!");
}
