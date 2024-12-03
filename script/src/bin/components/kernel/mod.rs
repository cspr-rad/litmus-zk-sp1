pub use super::{
    cache::Cache,
    fetcher::{Fetcher, FileSystemFetcher},
    prover::Prover,
};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

pub struct Kernel {
    cache: Cache,
    fetcher: Fetcher,
    prover: Prover,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Kernel {
    pub fn new(cache: Cache, fetcher: Fetcher, prover: Prover) -> Self {
        Self {
            cache,
            fetcher,
            prover,
        }
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl Kernel {
    pub fn cache(&self) -> &Cache {
        &self.cache
    }

    pub fn fetcher(&self) -> &Fetcher {
        &self.fetcher
    }

    pub fn prover(&self) -> &Prover {
        &self.prover
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl Default for Kernel {
    fn default() -> Self {
        unimplemented!()
    }
}
