use ltypes::chain::BlockHash;
use serde::{Deserialize, Serialize};
use std::fs;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    fetcher: FetcherConfig,
    name_of_chain: String,
    trusted_block_hash: BlockHash,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", content = "args")]
pub enum FetcherConfig {
    Chain { ip_address_set: Vec<String> },
    FileSystem { path_to_root: String },
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Config {
    pub fn new(path_to_toml: String) -> Self {
        let path_to_toml = fs::read_to_string(path_to_toml).unwrap();

        toml::from_str(&path_to_toml).unwrap()
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

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::Config;
    use std::{env, path::Path};

    fn get_path_to_toml_file() -> String {
        format!(
            "{}/fixtures/config.toml",
            env::var("CARGO_MANIFEST_DIR").unwrap()
        )
    }

    #[test]
    fn test_path_to_toml_file_exists() {
        let path_to_toml_file = get_path_to_toml_file();
        assert!(Path::new(&path_to_toml_file).exists());
    }

    #[test]
    fn test_instance_can_be_instantiated_from_toml_file() {
        let path_to_toml_file = get_path_to_toml_file();
        let _ = Config::new(path_to_toml_file);
    }
}
