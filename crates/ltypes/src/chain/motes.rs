use core::ops::Add;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// Base unit of system economic security mechanism.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Motes(u64);

/// Constants.
impl Motes {
    /// Maximum possible value.
    pub const MIN: Motes = Motes(u64::MIN);
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Motes {
    /// Factory: new [`Motes`] instance.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl Motes {
    pub fn inner(&self) -> u64 {
        self.0
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl Add for Motes {
    type Output = Motes;

    fn add(self, rhs: Self) -> Self::Output {
        Motes::new(self.inner() + rhs.inner())
    }
}

// ------------------------------------------------------------------------
// Traits -> serde.
// ------------------------------------------------------------------------

impl<'de> Deserialize<'de> for Motes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: &str = Deserialize::deserialize(deserializer).unwrap();
        let parsed: u64 = raw.parse().unwrap();
        Ok(Motes::new(parsed))
    }
}

impl Serialize for Motes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(serializer.serialize_u64(self.inner()).unwrap())
    }
}
