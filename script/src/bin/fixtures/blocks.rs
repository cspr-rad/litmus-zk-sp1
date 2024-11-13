// use alloc::collections::BTreeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// Monotonically increasing chain height.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct BlockHeight(u64);

/// An era represents a set of consensus rounds.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct EraId(u64);

// Block.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Block {
    #[serde(rename = "Version2")]
    V2(BlockV2),
}

// Block proof - i.e. finality signature issued by a validator in scope at point of block creation.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct BlockProof {
    #[serde(with = "hex::serde")]
    public_key: Vec<u8>,
    #[serde(with = "hex::serde")]
    signature: [u8; 65],
}

// Block (v2).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockV2 {
    // pub body: BlockV2Body,
    hash: String,
    header: BlockV2Header,
}

// Block (v2) body.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct BlockV2Body {
    // pub rewarded_signatures: Vec<String>,
    // pub transactions: BTreeMap<u8, Vec<String>>,
}

// Block (v2) header.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct BlockV2Header {
    /// A seed needed for initializing a future era.
    #[serde(with = "hex::serde")]
    accumulated_seed: [u8; 32],

    /// Digest over block body.
    #[serde(with = "hex::serde")]
    body_hash: [u8; 32],

    /// Gas price of era in scope at point of block creation.
    current_gas_price: u8,

    /// The `EraEnd` of a block if it is a switch block.
    // era_end: Option<EraEndV2>,

    /// ID of the era in scope at point of block creation.
    era_id: EraId,

    /// Height of this block, i.e. the number of ancestors.
    height: BlockHeight,

    /// Digest over most recent switch block hash.
    #[serde(with = "hex::serde")]
    last_switch_block_hash: [u8; 32],

    /// Digest over parent block.
    #[serde(with = "hex::serde")]
    parent_hash: [u8; 32],

    /// Identifier of validator which proposed the block.
    #[serde(with = "hex::serde")]
    proposer: Vec<u8>,

    /// Version of protocol in scope at point of block creation.
    protocol_version: ProtocolVersion,

    /// A random bit needed for initializing a future era.
    random_bit: bool,
    #[serde(with = "hex::serde")]

    /// Digest over global state once block transactions have been executed.
    state_root_hash: [u8; 32],

    /// Timestamp at point of block creation.
    timestamp: String,
}

// Block with associated set of proofs.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockWithProofs {
    block: Block,
    proofs: Vec<BlockProof>,
}

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize,
)]
pub struct ProtocolVersion(SemanticVersion);

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticVersion {
    /// Major version.
    pub major: u32,

    /// Minor version.
    pub minor: u32,

    /// Patch version.
    pub patch: u32,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl ProtocolVersion {
    pub fn new(semantic_version: SemanticVersion) -> Self {
        // TODO: validate inputs
        Self(semantic_version)
    }
}

impl SemanticVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        // TODO: validate inputs
        Self {
            major,
            minor,
            patch,
        }
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl BlockV2 {
    pub fn body(&self) -> BlockV2Body {
        unimplemented!()
    }
    pub fn hash(&self) -> &String {
        &self.hash
    }
    pub fn header(&self) -> &BlockV2Header {
        &self.header
    }
}

impl BlockWithProofs {
    pub fn block(&self) -> &Block {
        &self.block
    }
    pub fn proofs(&self) -> &Vec<BlockProof> {
        &self.proofs
    }
}

// ------------------------------------------------------------------------
// Traits -> serde.
// ------------------------------------------------------------------------

impl<'de> Deserialize<'de> for SemanticVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: &str = Deserialize::deserialize(deserializer).unwrap();
        let tokens: Vec<&str> = raw.split('.').collect();
        if tokens.len() != 3 {
            panic!("SemanticVersion deserialization error.")
        }

        Ok(SemanticVersion {
            major: tokens[0].parse().unwrap(),
            minor: tokens[1].parse().unwrap(),
            patch: tokens[2].parse().unwrap(),
        })
    }
}

impl Serialize for SemanticVersion {
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        unimplemented!("Serialize for SemanticVersion");
    }
}
