use frc::prelude::*;

mod bindings;
mod can;
mod subsystems;

fn main() {
    let mut state: State = State::new().unwrap();

    drivetrain::setup(&mut state);

    state.run();
}
