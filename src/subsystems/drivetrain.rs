use can;
use frc::prelude::*;

pub fn setup(state: &mut State) {
    state.insert_resource(Drivetrain::new());
    state.insert_resource(DrivetrainTargets::new());

    //state.add_systems(TeleopPeriodic, periodic);
    //state.add_systems(AutonomousPeriodic, periodic);
}

//fn periodic(mut drivetrain: ResourceMut<Drivetrain>, targets: Resource<DrivetrainTargets>) {}

struct Drivetrain {
    right1: TalonFX,
    right2: TalonFX,
    left1: TalonFX,
    left2: TalonFX,
}

impl Drivetrain {
    fn new() -> Self {
        let right1 = TalonFX::new(CanID::RightDrive1::id(), CanID::RightDrive1::bus());

        let right2 = TalonFX::new(CanID::RightDrive2::id(), CanID::RightDrive2::bus());

        let left1 = TalonFX::new(CanID::LeftDrive1::id(), CanID::LeftDrive1::bus());

        let left2 = TalonFX::new(CanID::LeftDrive2::id(), CanID::LeftDrive2::bus());

        Self {
            right1: right1,
            right2: right2,
            left1: left1,
            left2: left2,
        }
    }
}

pub struct DrivetrainTargets {
    pub speed: f64,
    pub turn: f64,
}

impl DrivetrainTargets {
    fn new() -> Self {
        Self {
            speed: 0.0,
            turn: 0.0,
        }
    }
}
