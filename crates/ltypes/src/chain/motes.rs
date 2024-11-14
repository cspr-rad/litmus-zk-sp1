use core::ops::Add;
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// Base unit of system economic security mechanism.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
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
