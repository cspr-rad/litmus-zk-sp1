use crate::fixtures::Fixtures;
use serde_json::Value;
use std::fs;

pub fn get_fixtures() -> Fixtures {
    fn get_content(typeof_fixture: String) -> String {
        fs::read_to_string(get_path(typeof_fixture)).unwrap()
    }

    fn get_content_block() -> Value {
        serde_json::from_str(&get_content(String::from("block-469"))).unwrap()
    }

    fn get_content_crypto() -> Value {
        serde_json::from_str(&get_content(String::from("crypto"))).unwrap()
    }

    fn get_path(typeof_fixture: String) -> String {
        format!("fixtures/{typeof_fixture}.json")
    }

    Fixtures {
        block: get_content_block(),
        crypto: serde_json::from_str(&get_content(String::from("crypto"))).unwrap(),
    }
}
