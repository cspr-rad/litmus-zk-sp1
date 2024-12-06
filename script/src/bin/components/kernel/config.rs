use super::super::fetcher::{ChainFetcherConfig, FileSystemFetcherConfig};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    name_of_chain: String,
    trusted_hash: String,
    fetcher: FetcherConfig,
}

#[derive(Deserialize)]
pub struct FetcherConfig {
    kind: &'static str,
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

    pub fn trusted_hash(&self) -> &String {
        &self.trusted_hash
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
