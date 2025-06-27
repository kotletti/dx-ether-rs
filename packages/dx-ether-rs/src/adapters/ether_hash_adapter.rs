use alloy_primitives::{U256, hex};

use crate::{errors::error::Error, ports::ether_hash_port::EtherHashPort};

pub struct EtherHashAdapter;

impl EtherHashAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl EtherHashPort for EtherHashAdapter {
    type Error = Error;

    fn encode_to_string(&self, bytes: &Vec<u8>) -> Result<String, Self::Error> {
        Ok(format!("0x{}", hex::encode(bytes)))
    }

    fn encode_to_number(&self, hash: &str) -> Result<String, Self::Error> {
        let number = hash.parse::<u64>()?;

        Ok(format!("0x{:x}", number))
    }

    fn decode_to_bytes(&self, hash: &str) -> Result<Vec<u8>, Self::Error> {
        let hex_data = hash.trim_start_matches("0x");

        let bytes = hex::decode(hex_data)?;

        Ok(bytes)
    }

    fn decode_to_string(&self, hash: &str) -> Result<String, Self::Error> {
        let bytes = self.decode_to_bytes(hash)?;

        Ok(String::from_utf8(bytes)?)
    }

    fn decode_to_number(&self, hash: &str) -> Result<String, Self::Error> {
        Ok(U256::from_str_radix(&hash.trim_start_matches("0x"), 16)?.to_string())
    }
}
