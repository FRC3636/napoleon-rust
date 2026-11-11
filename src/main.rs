use frc::prelude::*;

mod can;
mod drivetrain;

fn main() {
    let mut state: State = State::new().unwrap();

    drivetrain::setup(&mut state);
    state.insert_resource(XboxController::new(1));
    state.add_systems(TeleopPeriodic, drivetrain::drive_with_controller);

    state.run();
}
