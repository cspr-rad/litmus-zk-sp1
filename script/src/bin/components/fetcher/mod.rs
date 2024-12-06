use super::kernel::Config;
pub use chain::{Fetcher as ChainFetcher, FetcherConfig as ChainFetcherConfig};
pub use fsys::{Fetcher as FileSystemFetcher, FetcherConfig as FileSystemFetcherConfig};
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
    pub fn new(config: &Config) -> Self {
        match config.fetcher().kind().as_str() {
            "chain" => Self::new_chain(&config),
            "fsys" => Self::new_fsys(&config),
            _ => panic!("Invalid config option"),
        }
    }

    pub fn new_chain(config: &Config) -> Self {
        unimplemented!()
    }

    pub fn new_fsys(config: &Config) -> Self {
        // Scan file system.
        unimplemented!()
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

impl FetcherBackend for Fetcher {
    fn get_block(&self, block_id: BlockID) -> Result<Option<Block>, Error> {
        match self {
            Self::Chain(inner) => inner.get_block(block_id),
            Self::FileSystem(inner) => inner.get_block(block_id),
        }
    }
}
