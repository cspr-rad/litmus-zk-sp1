/// Monotonically increasing chain height.
#[derive(Clone, Debug)]
pub struct BlockHeight(u64);

/// Constructors.
impl BlockHeight {
    /// Factory: new [`EraId`] instance.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

/// Accessors.
impl BlockHeight {
    pub fn inner(&self) -> u64 {
        self.0
    }
}
