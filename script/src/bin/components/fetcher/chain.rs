use super::FetcherBackend;
use ltypes::chain::{Block, BlockHash, BlockHeight, BlockID};
use serde::Deserialize;
use std::io::Error;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

pub struct Fetcher {
    config: FetcherConfig,
}

#[derive(Deserialize)]
pub struct FetcherConfig {
    ip_address: String,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Fetcher {
    pub fn new(config: FetcherConfig) -> Self {
        Self { config }
    }
}

impl FetcherConfig {
    pub fn new(ip_address: String) -> Self {
        Self { ip_address }
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl Fetcher {
    fn get_block_by_hash(&self, block_hash: BlockHash) -> Result<Option<Block>, Error> {
        todo!()
    }

    fn get_block_by_height(&self, block_height: BlockHeight) -> Result<Option<Block>, Error> {
        todo!()
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl Default for Fetcher {
    fn default() -> Self {
        todo!()
    }
}

impl Default for FetcherConfig {
    fn default() -> Self {
        todo!()
    }
}

impl FetcherBackend for Fetcher {
    fn get_block(&self, block_id: BlockID) -> Result<Option<Block>, Error> {
        match block_id {
            BlockID::BlockHash(inner) => self.get_block_by_hash(inner),
            BlockID::BlockHeight(inner) => self.get_block_by_height(inner),
        }
    }
}
