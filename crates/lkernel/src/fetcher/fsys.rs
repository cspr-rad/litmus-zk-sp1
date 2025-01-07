use super::FetcherBackend;
use camino::Utf8PathBuf;
use ltypes::chain::{Block, BlockHash, BlockHeight, BlockID};
use std::{
    fs::{self, DirEntry},
    io::Error,
};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Debug)]
struct BlockFileInfo {
    hash: BlockHash,
    height: BlockHeight,
    path: Utf8PathBuf,
}

pub struct Fetcher {
    block_files: Vec<BlockFileInfo>,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl BlockFileInfo {
    fn new(hash: BlockHash, height: BlockHeight, path_to_file: Utf8PathBuf) -> Self {
        Self {
            hash,
            height,
            path: path_to_file,
        }
    }
}

impl Fetcher {
    pub fn new(path_to_root: String) -> Self {
        let mut block_files: Vec<BlockFileInfo> = Vec::new();
        for file in fs::read_dir(Utf8PathBuf::from(path_to_root).as_path()).unwrap() {
            block_files.push(BlockFileInfo::try_from(&file.unwrap()).unwrap());
        }

        Self { block_files }
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl BlockFileInfo {
    fn hash(&self) -> &BlockHash {
        &self.hash
    }

    fn height(&self) -> BlockHeight {
        self.height
    }

    fn path(&self) -> &Utf8PathBuf {
        &self.path
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl Fetcher {
    fn get_block_by_hash(&self, block_id: BlockHash) -> Result<Option<Block>, Error> {
        for file_info in &self.block_files {
            if file_info.hash == block_id {
                return Ok(Option::Some(Block::from(file_info)));
            }
        }

        Ok(None)
    }

    fn get_block_by_height(&self, block_id: BlockHeight) -> Result<Option<Block>, Error> {
        for file_info in &self.block_files {
            if file_info.height() == block_id {
                return Ok(Option::Some(Block::from(file_info)));
            }
        }

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
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl From<&BlockFileInfo> for Block {
    fn from(value: &BlockFileInfo) -> Self {
        unimplemented!()
    }
}

impl From<&DirEntry> for BlockFileInfo {
    fn from(value: &DirEntry) -> Self {
        let fpath = Utf8PathBuf::from_path_buf(value.path()).unwrap();
        let fname = value.file_name().into_string().unwrap();
        let fparts: Vec<&str> = fname.split("-").collect();

        BlockFileInfo::new(
            BlockHash::from(&fparts[2][..64]),
            BlockHeight::from(fparts[1]),
            fpath,
        )
    }
}

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::Fetcher;
    use std::env;

    fn get_path_to_root() -> String {
        format!(
            "{}/fixtures/blocks",
            env::var("CARGO_MANIFEST_DIR").unwrap()
        )
    }

    #[test]
    fn test_that_instance_can_be_instantiated() {
        Fetcher::new(get_path_to_root());
    }
}
