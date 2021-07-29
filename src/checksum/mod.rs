use bytes::Bytes;
pub use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CheckSumMethod {
    CRC32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CheckSum {
    method: CheckSumMethod,
    data: [u8; 32],
}

impl CheckSum {
    pub fn validate(&self, _data: &Bytes) {}
}
