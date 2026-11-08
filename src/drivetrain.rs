use frc::prelude::*;

struct Drivetrain {
    right_1: TalonFX,
    right_2: TalonFX,
    left_1: TalonFX,
    left_2: TalonFX,
}

impl Drivetrain {
    fn new() -> Self {
        Self {
            right_1: TalonFX::new(1, "*"),
            right_2: TalonFX::new(2, "*"),
            left_1: TalonFX::new(3, "*"),
            left_2: TalonFX::new(4, "*"),
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
