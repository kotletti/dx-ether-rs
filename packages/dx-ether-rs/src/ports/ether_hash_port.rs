pub trait EtherHashPort {
    type Error;

    fn encode_to_string(&self, bytes: &Vec<u8>) -> Result<String, Self::Error>;

    fn encode_to_number(&self, hash: &str) -> Result<String, Self::Error>;

    fn decode_to_bytes(&self, hash: &str) -> Result<Vec<u8>, Self::Error>;

    fn decode_to_string(&self, hash: &str) -> Result<String, Self::Error>;

    fn decode_to_number(&self, hash: &str) -> Result<String, Self::Error>;
}
