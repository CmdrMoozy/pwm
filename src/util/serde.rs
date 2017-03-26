use bincode::{self, deserialize, serialize};
use error::Result;
use serde::{Deserialize, Serialize};

pub fn serialize_binary<T: Serialize>(data: &T) -> Result<Vec<u8>> {
    match serialize(data, bincode::Infinite) {
        Err(e) => bail!("Binary serialization failed: {}", e),
        Ok(s) => Ok(s),
    }
}

pub fn deserialize_binary<T: Deserialize>(data: &[u8]) -> Result<T> {
    match deserialize(data) {
        Err(e) => bail!("Binary deserialization failed: {}", e),
        Ok(d) => Ok(d),
    }
}
