/// An era represents a set of consensus rounds.
#[derive(Clone, Debug)]
pub struct EraId(u64);

/// Constructors.
impl EraId {
    /// Factory: new [`EraId`] instance.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

/// Accessors.
impl EraId {
    pub fn inner(&self) -> u64 {
        self.0
    }
}
