// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// Monotonically increasing chain height.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BlockHeight(u64);

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl BlockHeight {
    /// Factory: new [`EraId`] instance.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl BlockHeight {
    pub fn inner(&self) -> u64 {
        self.0
    }
}
