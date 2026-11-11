pub enum CanID {
    RightDrive1,
    RightDrive2,
    LeftDrive1,
    LeftDrive2,
}

impl CanID {
    pub fn id(&self) -> i32 {
        match *self {
            CanID::RightDrive1 => 1,
            CanID::RightDrive2 => 2,
            CanID::LeftDrive1 => 3,
            CanID::LeftDrive2 => 4,
        }
    }
    pub fn bus(&self) -> &'static str  {
        match *self {
            CanID::RightDrive1 => "rio",
            CanID::RightDrive2 => "rio",
            CanID::LeftDrive1 => "rio",
            CanID::LeftDrive2 => "rio",
        }
    }
}
