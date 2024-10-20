use super::EraId;
use super::ValidatorWeight;

pub struct EraValidatorWeights {
    /// Consensus era id.
    era_id: EraId,

    /// Set of weights scoped by era and validator ID.
    weights: Vec<ValidatorWeight>,
}
