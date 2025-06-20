use borsh::{BorshDeserialize, BorshSerialize};

pub trait Borsh: BorshDeserialize + BorshSerialize {
    fn try_from_slice(bytes: &[u8]) -> borsh::io::Result<Self> {
        borsh::BorshDeserialize::try_from_slice(bytes)
    }
    fn to_vec(&self) -> Vec<u8> {
        borsh::to_vec(self).expect("Borsh serialization failed")
    }
}

// Blanket impl for any compatible type
impl<T: BorshDeserialize + BorshSerialize> Borsh for T {}
