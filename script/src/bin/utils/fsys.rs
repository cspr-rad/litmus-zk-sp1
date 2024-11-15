use crate::fixtures::chain::WrappedBlockWithProofs;
use crate::fixtures::Fixtures;
use std::fs;

pub fn get_fixtures() -> Fixtures {
    fn get_block_with_proofs() -> WrappedBlockWithProofs {
        let content = get_content(String::from("block-469"));
        println!("{:?}", content);

        serde_json::from_str(&content).unwrap()
    }

    fn get_content(typeof_fixture: String) -> String {
        fs::read_to_string(get_path(typeof_fixture)).unwrap()
    }

    fn get_path(typeof_fixture: String) -> String {
        format!("fixtures/{typeof_fixture}.json")
    }

    Fixtures {
        set_of_blocks_with_proofs: [get_block_with_proofs()].to_vec(),
        crypto: serde_json::from_str(&get_content(String::from("crypto"))).unwrap(),
    }
}
