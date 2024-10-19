pub struct ValidatorConsensusWeight {
    validator_id: [u8; 32],
    weight: u64,
}

impl ValidatorConsensusWeight {
    pub fn new(validator_id: [u8; 32], weight: u64) -> Self {
        Self {
            validator_id,
            weight,
        }
    }
}
