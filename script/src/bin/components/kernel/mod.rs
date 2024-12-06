pub(super) mod config;

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
        println!("{:?}", config);
        let cache = Cache::new(&config);
        let fetcher = Fetcher::new(&config);
        let prover = Prover::new(&config);

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
