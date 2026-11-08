use can;
use frc::prelude::*;

use crate::can::CanivoreID;

struct Drivetrain {
    right_1: TalonFX,
    right_2: TalonFX,
    left_1: TalonFX,
    left_2: TalonFX,
}

impl Drivetrain {
    fn new() -> Self {
        Self {
            right_1: TalonFX::new(CanivoreID::RightDrive1 as i32, "*"),
            right_2: TalonFX::new(CanivoreID::RightDrive2 as i32, "*"),
            left_1: TalonFX::new(CanivoreID::LeftDrive1 as i32, "*"),
            left_2: TalonFX::new(CanivoreID::LeftDrive2 as i32, "*"),
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

fn periodic(mut drivetrain: ResourceMut<Drivetrain>, targets: Resource<DrivetrainTargets>) {}
