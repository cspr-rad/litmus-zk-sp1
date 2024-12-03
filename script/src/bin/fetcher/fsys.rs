use super::FetcherService;
use ltypes::chain::{Block, BlockHash, BlockHeight, BlockID};
use std::io::Error;

pub(super) struct Service {
    config: ServiceConfig,
}

pub(super) struct ServiceConfig {
    path_to_root: String,
}

impl Service {
    fn get_block_by_hash(&self, block_hash: BlockHash) -> Result<Option<Block>, Error> {
        todo!()
    }

    fn get_block_by_height(&self, block_height: BlockHeight) -> Result<Option<Block>, Error> {
        todo!()
    }
}

impl FetcherService for Service {
    fn get_block(&self, block_id: BlockID) -> Result<Option<Block>, Error> {
        match block_id {
            BlockID::BlockHash(inner) => self.get_block_by_hash(inner),
            BlockID::BlockHeight(inner) => self.get_block_by_height(inner),
        }
    }
}
