mod config;

pub use super::{
    cache::Cache,
    fetcher::{Fetcher, FileSystemFetcher},
    prover::Prover,
};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

pub struct Kernel {
    config: config::Config,
    cache: Cache,
    fetcher: Fetcher,
    prover: Prover,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Kernel {
    pub fn new(path_to_toml: String) -> Self {
        let config = config::Config::new(path_to_toml);

        Self {
            config,
            cache: Cache::new(),
            fetcher: Fetcher::new(),
            prover: Prover::new(),
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
