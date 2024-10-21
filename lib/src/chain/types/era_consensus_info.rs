use std::u64;

use super::EraId;
use super::ValidatorWeight;

/// Information scoped by era pertinent to consensus.
pub struct EraConsensusInfo {
    /// Consensus era identifier.
    era_id: EraId,

    /// Set of validator voting weights scoped by era.
    validator_weights: Vec<ValidatorWeight>,

    /// Total weight of votes
    total_weight: u64,
}

/// Constructors.
impl EraConsensusInfo {
    pub fn new(era_id: EraId, validator_weights: Vec<ValidatorWeight>) -> Self {
        let total_weight: &u64 = &validator_weights
            .iter()
            .fold(u64::MIN, |acc, x| acc + x.weight());

        Self {
            era_id,
            validator_weights,
            total_weight: total_weight.clone(),
        }
    }
}
