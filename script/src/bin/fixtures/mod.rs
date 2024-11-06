use serde::{Deserialize, Serialize};

pub mod blocks;
pub mod crypto;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Fixtures {
    pub crypto: crypto::Fixtures,
}
