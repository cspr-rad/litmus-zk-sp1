use super::FetcherService;
use ltypes::chain::{Block, BlockHash, BlockHeight, BlockID};
use std::io::Error;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

pub struct Service {
    config: ServiceConfig,
}

pub struct ServiceConfig {
    path_to_root: String,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Service {
    pub fn new(config: ServiceConfig) -> Self {
        Self { config }
    }
}

impl ServiceConfig {
    pub fn new(path_to_root: String) -> Self {
        Self { path_to_root }
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl Service {
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

impl FetcherService for Service {
    fn get_block(&self, block_id: BlockID) -> Result<Option<Block>, Error> {
        match block_id {
            BlockID::BlockHash(inner) => self.get_block_by_hash(inner),
            BlockID::BlockHeight(inner) => self.get_block_by_height(inner),
        }
    }
}
