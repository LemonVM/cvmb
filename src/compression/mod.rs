use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CompressionInfo {}
