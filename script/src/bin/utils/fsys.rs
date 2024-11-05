use super::fixtures::Fixtures;
use serde_json;
use std::fs;

pub fn get_fixtures() -> Fixtures {
    let fpath = "fixtures/crypto.json";
    let fcontents = fs::read_to_string(fpath).unwrap();

    serde_json::from_str(&fcontents).unwrap()
}
