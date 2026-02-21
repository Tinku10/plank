pub trait Serialize {
    fn to_bytes(&self) -> Vec<u8>;
}

trait Deserialize {
}
