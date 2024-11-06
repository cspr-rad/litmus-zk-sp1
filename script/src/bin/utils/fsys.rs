use crate::fixtures::Fixtures;
use serde_json;
use std::fs;

pub fn get_fixtures() -> Fixtures {
    pub fn get_content(typeof_fixture: String) -> String {
        fs::read_to_string(get_path(typeof_fixture)).unwrap()
    }

    fn get_path(typeof_fixture: String) -> String {
        format!("fixtures/{typeof_fixture}.json")
    }

    Fixtures {
        crypto: serde_json::from_str(&get_content(String::from("crypto"))).unwrap(),
    }
}
