use chain::Service as ChainFetcherService;
use fsys::Service as FileSystemFetcherService;
use ltypes::chain::{Block, BlockHash, BlockHeight, BlockID};
use std::io::Error;

mod chain;
mod fsys;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

pub enum Fetcher {
    Chain(ChainFetcherService),
    FileSystem(FileSystemFetcherService),
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Fetcher {
    pub fn new_chain(inner: ChainFetcherService) -> Self {
        Self::Chain(inner)
    }

    pub fn new_fsys(inner: FileSystemFetcherService) -> Self {
        Self::FileSystem(inner)
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

/// A component that encapsulates block fetching.
trait FetcherService {
    /// Retrieves a block by an identifier.
    ///
    /// # Arguments
    ///
    /// * `block_id` - Identifier of a block for which to issue a query.
    ///
    fn get_block(&self, block_id: BlockID) -> Result<Option<Block>, Error>;
}

impl FetcherService for Fetcher {
    fn get_block(&self, block_id: BlockID) -> Result<Option<Block>, Error> {
        match self {
            Self::Chain(inner) => inner.get_block(block_id),
            Self::FileSystem(inner) => inner.get_block(block_id),
        }
    }
}
