use std::convert::TryInto;

pub trait Unpackable {
    fn get_bytes() -> usize;
    fn from_le_bytes(bytes: &[u8]) -> Self;
}

impl Unpackable for u64 {
    fn get_bytes() -> usize {
        8
    }

    fn from_le_bytes(bytes: &[u8]) -> Self{
        return u64::from_le_bytes(bytes.try_into().unwrap());
    }
}

impl Unpackable for f64 {
    fn get_bytes() -> usize {
        8
    }

    fn from_le_bytes(bytes: &[u8]) -> Self{
        return f64::from_le_bytes(bytes.try_into().unwrap());
    }
}

impl Unpackable for u128 {
    fn get_bytes() -> usize {
        16
    }

    fn from_le_bytes(bytes: &[u8]) -> Self{
        return u128::from_le_bytes(bytes.try_into().unwrap());
    }
}