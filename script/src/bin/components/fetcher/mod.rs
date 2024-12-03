pub use chain::Fetcher as ChainFetcher;
pub use fsys::Fetcher as FileSystemFetcher;
use ltypes::chain::{Block, BlockID};
use std::io::Error;

mod chain;
mod fsys;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

pub enum Fetcher {
    Chain(ChainFetcher),
    FileSystem(FileSystemFetcher),
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Fetcher {
    pub fn new_chain(inner: ChainFetcher) -> Self {
        Self::Chain(inner)
    }

    pub fn new_fsys(inner: FileSystemFetcher) -> Self {
        Self::FileSystem(inner)
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

/// Set of backend functions that each block fetcher must implement.
trait FetcherBackend {
    /// Retrieves a block by an identifier.
    ///
    /// # Arguments
    ///
    /// * `block_id` - Identifier of a block for which to issue a query.
    ///
    fn get_block(&self, block_id: BlockID) -> Result<Option<Block>, Error>;
}

impl Default for Fetcher {
    fn default() -> Self {
        Self::new_fsys(FileSystemFetcher::default())
    }
}

impl FetcherBackend for Fetcher {
    fn get_block(&self, block_id: BlockID) -> Result<Option<Block>, Error> {
        match self {
            Self::Chain(inner) => inner.get_block(block_id),
            Self::FileSystem(inner) => inner.get_block(block_id),
        }
    }
}
