use crate::can::*;
use frc::math::prelude::*;
use frc::prelude::*;

const MAX_SPEED: f64 = 3.0;
const TURN_SPEED: f64 = 1.0;
const KV: f64 = 1.0;

pub fn setup(state: &mut State) {
    state.insert_resource(Drivetrain::new());
}

pub fn drive_with_controller(controller: Resource<XboxController>, mut drivetrain: ResourceMut<Drivetrain>) {
    let speeds = ChassisSpeeds::new(
        controller.left_x() * MAX_SPEED.meters_per_second(),
        0.0.meters_per_second(),
        controller.right_y() * TURN_SPEED.rotations_per_second(),
    );
    let wheel_speeds = drivetrain.kinematics.to_wheel_speeds(&speeds);
    drivetrain.right1.set_voltage(wheel_speeds.right.value * KV.volts());
    drivetrain.right2.set_voltage(wheel_speeds.right.value * KV.volts());
    drivetrain.left1.set_voltage(wheel_speeds.left.value * KV.volts());
    drivetrain.left2.set_voltage(wheel_speeds.left.value * KV.volts());
}

pub struct Drivetrain {
    right1: TalonFX,
    right2: TalonFX,
    left1: TalonFX,
    left2: TalonFX,
    kinematics: DifferentialDriveKinematics,
}

impl Drivetrain {
    fn new() -> Self {
        let right1 = TalonFX::new(CanID::RightDrive1.id(), CanID::RightDrive1.bus());
        let right2 = TalonFX::new(CanID::RightDrive2.id(), CanID::RightDrive2.bus());
        let left1 = TalonFX::new(CanID::LeftDrive1.id(), CanID::LeftDrive1.bus());
        let left2 = TalonFX::new(CanID::LeftDrive2.id(), CanID::LeftDrive2.bus());

        Self {
            right1: right1,
            right2: right2,
            left1: left1,
            left2: left2,
            kinematics: DifferentialDriveKinematics::new(24.0.inches()),
        }
    }
}
