extern crate alloc;

use super::{BlockHash, EraEndV2, EraId, ProtocolVersion, Timestamp, TransactionV2Hash};
use crate::crypto::{Digest, PublicKey};
use alloc::collections::BTreeMap;
use lutils::bites::Byte;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

// Version 2 block.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Block {
    /// Digest over block body + header.
    pub(super) hash: BlockHash,

    /// Information pertaining to vm + consensus.
    pub(super) header: BlockBody,

    /// Block meta data.
    pub(super) body: BlockHeader,
}

// Version 2 block body.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BlockBody {
    /// Map of transactions mapping categories to a list of transaction hashes.
    pub(super) transactions: BTreeMap<Byte, Vec<TransactionV2Hash>>,

    /// List of identifiers for finality signatures for a particular past block.
    pub(super) rewarded_signatures: Vec<Byte>,
}

// Version 2 block header.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BlockHeader {
    /// The parent block's hash.
    pub(super) parent_hash: BlockHash,

    /// The root hash of global state after the deploys in this block have been executed.
    pub(super) state_root_hash: Digest,

    /// The hash of the block's body.
    pub(super) body_hash: Digest,

    /// A random bit needed for initializing a future era.
    pub(super) random_bit: bool,

    /// A seed needed for initializing a future era.
    pub(super) accumulated_seed: Digest,

    /// The `EraEnd` of a block if it is a switch block.
    pub(super) era_end: Option<EraEndV2>,

    /// The timestamp from when the block was proposed.
    pub(super) timestamp: Timestamp,

    /// The era ID in which this block was created.
    pub(super) era_id: EraId,

    /// The height of this block, i.e. the number of ancestors.
    pub(super) height: u64,

    /// The protocol version of the network from when this block was created.
    pub(super) protocol_version: ProtocolVersion,

    /// The public key of the validator which proposed the block.
    pub(super) proposer: PublicKey,

    /// The gas price of the era
    pub(super) current_gas_price: u8,

    /// The most recent switch block hash.
    pub(super) last_switch_block_hash: Option<BlockHash>,
}
