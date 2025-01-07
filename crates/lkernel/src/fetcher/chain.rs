use super::FetcherBackend;
use ltypes::chain::{Block, BlockHash, BlockHeight, BlockID};
use std::io::Error;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

pub struct Fetcher {
    ip_address_set: Vec<String>,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Fetcher {
    pub fn new(ip_address_set: Vec<String>) -> Self {
        Self { ip_address_set }
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl Fetcher {
    fn get_block_by_hash(&self, block_hash: BlockHash) -> Option<Block> {
        todo!()
    }

    fn get_block_by_height(&self, block_height: BlockHeight) -> Option<Block> {
        todo!()
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl FetcherBackend for Fetcher {
    fn get_block(&self, block_id: BlockID) -> Option<Block> {
        match block_id {
            BlockID::BlockHash(inner) => self.get_block_by_hash(inner),
            BlockID::BlockHeight(inner) => self.get_block_by_height(inner),
        }
    }

    fn init(&self) -> Result<(), Error> {
        unimplemented!()
    }
}
