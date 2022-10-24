pub trait Proto {
    fn dispatch(&self);
    fn from_bytes(bytes: &[u8]) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
}
