use ltypes::chain::BlockHash;
use serde::Deserialize;
use std::fs;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    fetcher: FetcherConfig,
    name_of_chain: String,
    trusted_block_hash: BlockHash,
}

#[derive(Clone, Debug, Deserialize)]
pub enum FetcherConfig {
    Chain { ip_address_set: Vec<String> },
    FileSystem { path_to_root: String },
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Config {
    pub fn new(path_to_toml: String) -> Self {
        println!("222 {:?}", path_to_toml);
        let f = fs::read_to_string(path_to_toml).unwrap();

        toml::from_str(&f).unwrap()
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl Config {
    pub fn fetcher(&self) -> &FetcherConfig {
        &self.fetcher
    }

    pub fn name_of_chain(&self) -> &String {
        &self.name_of_chain
    }

    pub fn trusted_block_hash(&self) -> &BlockHash {
        &self.trusted_block_hash
    }
}
