pub(super) mod config;

use super::fetcher::FetcherBackend;

pub use super::{cache::Cache, fetcher::Fetcher, prover::Prover};
pub use config::Config;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

pub struct Kernel {
    config: Config,
    cache: Cache,
    fetcher: Fetcher,
    prover: Prover,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Kernel {
    pub fn new(path_to_config_toml: String) -> Self {
        let config = Config::new(path_to_config_toml);
        let cache = Cache::new(config.clone());
        let fetcher = Fetcher::new(config.clone());
        let prover = Prover::new(config.clone());

        Self {
            cache,
            config,
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

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn fetcher(&self) -> &Fetcher {
        &self.fetcher
    }

    pub fn prover(&self) -> &Prover {
        &self.prover
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl Kernel {
    /// Kernel initializer.
    pub fn init(&self) {
        self.fetcher.init().unwrap();
    }
}
