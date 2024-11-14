extern crate alloc;

use super::{Motes, ValidatorWeight};
use crate::crypto::PublicKey;
use alloc::collections::BTreeMap;
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct EraEndV1 {}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct EraEndV2 {
    /// The set of equivocators.
    pub(super) equivocators: Vec<PublicKey>,

    /// Validators that haven't produced any unit during the era.
    pub(super) inactive_validators: Vec<PublicKey>,

    /// The validators for the upcoming era and their respective weights.
    pub(super) next_era_validator_weights: Vec<ValidatorWeight>,

    /// The rewards distributed to the validators.
    pub(super) rewards: BTreeMap<PublicKey, Vec<Motes>>,

    pub(super) next_era_gas_price: u8,
}
