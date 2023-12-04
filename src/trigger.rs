#[derive(Debug)]
pub struct Trigger {
    pub note: u8,
    pub velocity: u8,
}

impl Trigger {
    pub fn new(note: u8, velocity: u8) -> Self {
        Self {
            note,
            velocity,
        }
    }
}