use super::ValidatorID;
use super::Weight;

/// Weight of an identity's vote within the context of some form of governance process.
pub struct ValidatorWeight {
    /// Identity of a voting entity.
    validator_id: ValidatorID,

    /// Relative weight of vote.
    weight: Weight,
}

impl ValidatorWeight {
    pub fn validator_id(&self) -> ValidatorID {
        self.validator_id
    }
    pub fn weight(&self) -> Weight {
        self.weight
    }
}

impl ValidatorWeight {
    pub fn new(validator_id: ValidatorID, weight: Weight) -> Self {
        Self {
            validator_id,
            weight,
        }
    }
}
