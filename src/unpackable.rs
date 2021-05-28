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
