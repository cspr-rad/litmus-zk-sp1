use crate::fixtures::chain::WrappedBlockWithProofs;
use crate::fixtures::Fixtures;
use std::fs;

pub fn get_fixtures() -> Fixtures {
    fn get_set_of_blocks_with_proofs() -> Vec<WrappedBlockWithProofs> {
        [
            get_wrapped_block_with_proofs(469),
            get_wrapped_block_with_proofs(470),
        ]
        .to_vec()
    }

    fn get_wrapped_block_with_proofs(block_id: u32) -> WrappedBlockWithProofs {
        let content = get_content(format!("block-{block_id}"));
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
        set_of_blocks_with_proofs: get_set_of_blocks_with_proofs(),
        crypto: serde_json::from_str(&get_content(String::from("crypto"))).unwrap(),
    }
}
