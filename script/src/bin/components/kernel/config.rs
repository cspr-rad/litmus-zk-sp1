use super::super::fetcher::{ChainFetcherConfig, FileSystemFetcherConfig};
use ltypes::chain::BlockHash;
use serde::Deserialize;
use std::fs;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    name_of_chain: String,
    trusted_hash_hex: BlockHash,
    fetcher: FetcherConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FetcherConfig {
    kind: String,
    chain: ChainFetcherConfig,
    fsys: FileSystemFetcherConfig,
}

impl Config {
    pub fn new(path_to_toml: String) -> Self {
        let f = fs::read_to_string(path_to_toml).unwrap();

        toml::from_str(&f).unwrap()
    }
}

impl Config {
    pub fn fetcher(&self) -> &FetcherConfig {
        &self.fetcher
    }

    pub fn name_of_chain(&self) -> &String {
        &self.name_of_chain
    }

    pub fn trusted_hash_hex(&self) -> &BlockHash {
        &self.trusted_hash_hex
    }
}

impl FetcherConfig {
    pub fn kind(&self) -> &String {
        &self.kind
    }

    pub fn chain(&self) -> &ChainFetcherConfig {
        &self.chain
    }

    pub fn fsys(&self) -> &FileSystemFetcherConfig {
        &self.fsys
    }
}
