use super::FetcherBackend;
use ltypes::chain::{Block, BlockHash, BlockHeight, BlockID};
use std::{fs, io::Error};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

pub struct Fetcher {
    path_to_root: &String,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Fetcher {
    pub fn new(path_to_root: &String) -> Self {
        Self { path_to_root }
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl Fetcher {
    fn get_block_by_hash(&self, block_hash: BlockHash) -> Result<Option<Block>, Error> {
        let fpattern = format!("block-*-{:?}.json", block_hash);
        println!("{:?}", fpattern);

        Ok(None)
    }

    fn get_block_by_height(&self, block_height: BlockHeight) -> Result<Option<Block>, Error> {
        let fpattern = format!("block-*{:?}-*.json", block_height);
        println!("{:?}", fpattern);

        Ok(None)
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl FetcherBackend for Fetcher {
    fn get_block(&self, block_id: BlockID) -> Result<Option<Block>, Error> {
        match block_id {
            BlockID::BlockHash(inner) => self.get_block_by_hash(inner),
            BlockID::BlockHeight(inner) => self.get_block_by_height(inner),
        }
    }

    fn init(&self) -> Result<(), Error> {
        let dir = fs::read_dir(format!("fixtures/chain/blocks")).unwrap();
        for file in dir {
            let file = file.unwrap();
            let parts = file
                .file_name()
                .into_string()
                .unwrap()
                .split("-")
                .collect::<Vec<_>>();

            println!("{:?}", file.file_name().into_string().unwrap().split("-"));
        }
        unimplemented!()
    }
}
