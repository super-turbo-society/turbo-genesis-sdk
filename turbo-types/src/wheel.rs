use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, BorshDeserialize, BorshSerialize)]
pub struct Wheel {
    /// The x scroll delta
    pub delta_x: i32,
    /// The y scroll delta
    pub delta_y: i32,
    /// The z scroll delta
    pub delta_z: i32,
}
impl Wheel {
    pub fn xy(&self) -> (i32, i32) {
        (self.delta_x, self.delta_y)
    }
    pub fn xyz(&self) -> (i32, i32, i32) {
        (self.delta_x, self.delta_y, self.delta_z)
    }
}
