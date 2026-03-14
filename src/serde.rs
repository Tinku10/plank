pub trait Serialize {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait Deserialize<'a>: Sized {
    type Schema;
    fn from_bytes(bytes: &[u8], schema: &'a Self::Schema) -> std::io::Result<Self>;
}
