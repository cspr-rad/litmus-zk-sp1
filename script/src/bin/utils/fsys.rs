#![allow(dead_code)]
use serde_json;
use std::fs;
use super::fixtures::Fixtures;

pub fn get_fixtures() -> Fixtures {
    let fpath = "fixtures/crypto.json";

    serde_json::from_str(&fs::read_to_string(fpath).unwrap()).unwrap()
}
