mod cache;
mod fetcher;
mod prover;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

pub struct Components {
    cache: cache::Cache,
    fetcher: fetcher::Fetcher,
    prover: prover::Prover,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Components {
    pub fn new(cache: cache::Cache, fetcher: fetcher::Fetcher, prover: prover::Prover) -> Self {
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

impl Components {
    pub fn cache(&self) -> &cache::Cache {
        &self.cache
    }

    pub fn fetcher(&self) -> &fetcher::Fetcher {
        &self.fetcher
    }

    pub fn prover(&self) -> &prover::Prover {
        &self.prover
    }
}
