extern crate alloc;

use super::{BlockHash, BlockHeight, EraEndV2, EraId, ProtocolVersion, TransactionV2Hash};
use crate::crypto::{Digest, PublicKey};
use crate::misc::Timestamp;
use alloc::collections::BTreeMap;
use lutils::bites::Byte;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

// Version 2 block.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Block {
    /// Information pertaining to vm + consensus.
    pub(super) header: BlockHeader,

    /// Digest over block body + header.
    pub(super) hash: BlockHash,

    /// Block meta data.
    pub(super) body: BlockBody,
}

// Version 2 block body.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BlockBody {
    /// List of identifiers for finality signatures for a particular past block.
    pub(super) rewarded_signatures: Vec<Byte>,

    /// Map of transactions mapping categories to a list of transaction hashes.
    pub(super) transactions: BTreeMap<Byte, Vec<TransactionV2Hash>>,
}

// Version 2 block header.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BlockHeader {
    /// A seed needed for initializing a future era.
    pub(super) accumulated_seed: Digest,

    /// The hash of the block's body.
    pub(super) body_hash: Digest,

    /// The gas price of the era
    pub(super) current_gas_price: u8,

    /// The `EraEnd` of a block if it is a switch block.
    pub(super) era_end: Option<EraEndV2>,

    /// The era ID in which this block was created.
    pub(super) era_id: EraId,

    /// The height of this block, i.e. the number of ancestors.
    pub(super) height: BlockHeight,

    /// The most recent switch block hash.
    pub(super) last_switch_block_hash: Option<BlockHash>,

    /// The parent block's hash.
    pub(super) parent_hash: BlockHash,

    /// The public key of the validator which proposed the block.
    pub(super) proposer: PublicKey,

    /// The protocol version of the network from when this block was created.
    pub(super) protocol_version: ProtocolVersion,

    /// A random bit needed for initializing a future era.
    pub(super) random_bit: bool,

    /// The root hash of global state after the deploys in this block have been executed.
    pub(super) state_root_hash: Digest,

    /// The timestamp from when the block was proposed.
    pub(super) timestamp: Timestamp,
}

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
        rewarded_signatures: Vec<Byte>,
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
        timestamp: Timestamp,
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
            timestamp,
        }
    }
}
