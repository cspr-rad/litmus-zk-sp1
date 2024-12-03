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
    ip_address: String,
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
    pub fn new(ip_address: String) -> Self {
        Self { ip_address }
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl Service {
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

impl FetcherService for Service {
    fn get_block(&self, block_id: BlockID) -> Result<Option<Block>, Error> {
        match block_id {
            BlockID::BlockHash(inner) => self.get_block_by_hash(inner),
            BlockID::BlockHeight(inner) => self.get_block_by_height(inner),
        }
    }
}
