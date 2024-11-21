use serde::{Deserialize, Serialize};
use std::fmt;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// An era represents a set of consensus rounds.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct EraId(u64);

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl EraId {
    /// Factory: new [`EraId`] instance.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl EraId {
    // Maximum possible value an [`EraId`] can hold.
    pub const MAX: EraId = EraId(u64::MAX);

    // Minimum possible value an [`EraId`] can hold.
    pub const MIN: EraId = EraId(u64::MIN);

    // Wrapped value.
    pub fn inner(&self) -> u64 {
        self.0
    }

    /// Returns whether this is era 0.
    pub fn is_genesis(&self) -> bool {
        self.0 == 0
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl fmt::Display for EraId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERA-ID::{}", self.inner())
    }
}

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
use proptest::prelude::*;

#[cfg(test)]
use rand::Rng;

#[cfg(test)]
impl EraId {
    // Returns a random `EraId`.
    #[cfg(any(feature = "testing", test))]
    pub fn new_from_arb() -> impl Strategy<Value = Self> {
        any::<u64>().prop_map(Self::new)
    }
}

#[cfg(test)]
impl EraId {
    /// Returns a random `EraId`.
    #[cfg(any(feature = "testing", test))]
    pub fn random() -> Self {
        Self::new(rand::thread_rng().gen())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_genesis() {
        for (entity, expected) in [(EraId::MIN, true), (EraId::MAX, false)] {
            assert_eq!(entity.is_genesis(), expected);
        }
    }
}

#[cfg(test)]
mod proptests {
    use super::*;

    proptest! {
        #[test]
        fn codec_roundtrip(era_id in EraId::new_from_arb()) {
            // bytesrepr::test_serialization_roundtrip(&era_id);
            println!("era_id: {:?}", era_id);
        }
    }
}
