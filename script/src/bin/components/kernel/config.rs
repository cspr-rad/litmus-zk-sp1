use super::super::fetcher::{ChainFetcherConfig, FileSystemFetcherConfig};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub(super) struct Config {
    fetcher: FetcherConfig,
}

#[derive(Deserialize)]
pub(super) struct FetcherConfig {
    kind: String,
    chain: ChainFetcherConfig,
    fsys: FileSystemFetcherConfig,
}

impl Config {
    pub(super) fn new(path_to_toml: String) -> Self {
        let f = fs::read_to_string(path_to_toml).unwrap();

        toml::from_str(&f).unwrap()
    }
}
