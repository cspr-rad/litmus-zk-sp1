extern crate alloc;

use super::{BlockHash, BlockHeight, EraEndV2, EraId, ProtocolVersion, TransactionV2Hash};
use crate::crypto::{Digest, PublicKey};
use crate::misc::Timestamp;
use alloc::collections::BTreeMap;
use lutils::bites::Byte;
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

// Block (v2).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct Block {
    /// Information pertaining to vm + consensus.
    header: BlockHeader,

    /// Digest over block body + header.
    hash: BlockHash,

    /// Block meta data.
    body: BlockBody,
}

// Block (v2) body.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct BlockBody {
    /// List of identifiers for finality signatures for a particular past block.
    rewarded_signatures: Vec<Vec<Byte>>,

    /// Map of transactions mapping categories to a list of transaction hashes.
    transactions: BTreeMap<Byte, Vec<TransactionV2Hash>>,
}

// Block (v2) header.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct BlockHeader {
    /// A seed needed for initializing a future era.
    accumulated_seed: Digest,

    /// The hash of the block's body.
    body_hash: Digest,

    /// The gas price of the era
    current_gas_price: u8,

    /// The `EraEnd` of a block if it is a switch block.
    era_end: Option<EraEndV2>,

    /// The era ID in which this block was created.
    era_id: EraId,

    /// The height of this block, i.e. the number of ancestors.
    height: BlockHeight,

    /// The most recent switch block hash.
    last_switch_block_hash: Option<BlockHash>,

    /// The parent block's hash.
    parent_hash: BlockHash,

    /// The public key of the validator which proposed the block.
    proposer: PublicKey,

    /// The protocol version of the network from when this block was created.
    protocol_version: ProtocolVersion,

    /// A random bit needed for initializing a future era.
    random_bit: bool,

    /// The root hash of global state after the deploys in this block have been executed.
    state_root_hash: Digest,
}

// The timestamp from when the block was proposed.
// timestamp: Timestamp,

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Block {
    pub fn new(body: BlockBody, hash: BlockHash, header: BlockHeader) -> Self {
        // TODO: validate inputs.
        Self { body, hash, header }
    }
}

impl BlockBody {
    pub fn new(
        rewarded_signatures: Vec<Vec<Byte>>,
        transactions: BTreeMap<Byte, Vec<TransactionV2Hash>>,
    ) -> Self {
        // TODO: validate inputs.
        Self {
            rewarded_signatures,
            transactions,
        }
    }
}

impl BlockHeader {
    pub fn new(
        accumulated_seed: Digest,
        body_hash: Digest,
        current_gas_price: u8,
        era_end: Option<EraEndV2>,
        era_id: EraId,
        height: BlockHeight,
        last_switch_block_hash: Option<BlockHash>,
        parent_hash: BlockHash,
        proposer: PublicKey,
        protocol_version: ProtocolVersion,
        random_bit: bool,
        state_root_hash: Digest,
        // timestamp: Timestamp,
    ) -> Self {
        // TODO: validate inputs.
        Self {
            accumulated_seed,
            body_hash,
            current_gas_price,
            era_end,
            era_id,
            height,
            last_switch_block_hash,
            parent_hash,
            proposer,
            protocol_version,
            random_bit,
            state_root_hash,
            // timestamp,
        }
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl Block {
    pub fn body(&self) -> &BlockBody {
        unimplemented!()
    }

    pub fn hash(&self) -> &BlockHash {
        &self.hash
    }

    pub fn header(&self) -> &BlockHeader {
        &self.header
    }
}

impl BlockBody {
    pub fn rewarded_signatures(&self) -> &Vec<Vec<Byte>> {
        &self.rewarded_signatures
    }

    pub fn transactions(&self) -> &BTreeMap<Byte, Vec<TransactionV2Hash>> {
        &self.transactions
    }
}

impl BlockHeader {
    pub fn accumulated_seed(&self) -> &Digest {
        &self.accumulated_seed
    }

    pub fn body_hash(&self) -> &Digest {
        &self.body_hash
    }

    pub fn current_gas_price(&self) -> &u8 {
        &self.current_gas_price
    }

    pub fn era_end(&self) -> &Option<EraEndV2> {
        &self.era_end
    }

    pub fn height(&self) -> &BlockHeight {
        &self.height
    }

    pub fn last_switch_block_hash(&self) -> &Option<BlockHash> {
        &self.last_switch_block_hash
    }

    pub fn parent_hash(&self) -> &BlockHash {
        &self.parent_hash
    }

    pub fn proposer(&self) -> &PublicKey {
        &self.proposer
    }

    pub fn protocol_version(&self) -> &ProtocolVersion {
        &self.protocol_version
    }

    pub fn random_bit(&self) -> &bool {
        &self.random_bit
    }

    pub fn state_root_hash(&self) -> &Digest {
        &self.state_root_hash
    }

    // pub fn timestamp(&self) -> &Timestamp {
    //     &self.timestamp
    // }
}
