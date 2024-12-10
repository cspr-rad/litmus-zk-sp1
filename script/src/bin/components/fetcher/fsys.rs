use super::super::kernel::Config as KernelConfig;
use super::FetcherBackend;
use ltypes::chain::{Block, BlockHash, BlockHeight, BlockID};
use serde::Deserialize;
use std::io::Error;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

pub struct Fetcher {
    config: KernelConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FetcherConfig {
    path_to_root: String,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Fetcher {
    pub fn new(config: KernelConfig) -> Self {
        Self { config }
    }
}

impl FetcherConfig {
    pub fn new(path_to_root: String) -> Self {
        Self { path_to_root }
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl Fetcher {
    fn get_block_by_hash(&self, block_hash: BlockHash) -> Result<Option<Block>, Error> {
        let fpattern = format!("block-*-{:?}.json", block_hash);
        println!("{:?}", fpattern);

        Ok(None)
    }

    fn get_block_by_height(&self, block_height: BlockHeight) -> Result<Option<Block>, Error> {
        let fpattern = format!("block-*{:?}-*.json", block_height);
        println!("{:?}", fpattern);

        Ok(None)
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl FetcherBackend for Fetcher {
    fn get_block(&self, block_id: BlockID) -> Result<Option<Block>, Error> {
        match block_id {
            BlockID::BlockHash(inner) => self.get_block_by_hash(inner),
            BlockID::BlockHeight(inner) => self.get_block_by_height(inner),
        }
    }
}
