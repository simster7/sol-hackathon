use std::convert::TryInto;

pub trait Unpackable {
    fn get_bytes() -> usize;
}

impl Unpackable for u64 {
    fn get_bytes() -> usize {
        8
    }
}

impl Unpackable for f64 {
    fn get_bytes() -> usize {
        8
    }
}

impl Unpackable for u128 {
    fn get_bytes() -> usize {
        16
    }
}


pub trait ByteDeserialize {
    fn from_le_bytes(bytes: &[u8]) -> Self;
}

impl ByteDeserialize for u64 {
    fn from_le_bytes(bytes: &[u8]) -> Self{
        return u64::from_le_bytes(bytes.try_into().unwrap());
    }
}

impl ByteDeserialize for f64 {
    fn from_le_bytes(bytes: &[u8]) -> Self{
        return f64::from_le_bytes(bytes.try_into().unwrap());
    }
}

impl ByteDeserialize for u128 {
    fn from_le_bytes(bytes: &[u8]) -> Self{
        return u128::from_le_bytes(bytes.try_into().unwrap());
    }
}