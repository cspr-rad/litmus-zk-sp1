pub struct EraId(u64);

impl EraId {
    /// Constraint: maximum possible value an [`EraId`] can hold.
    pub const MAX: EraId = EraId(u64::MAX);

    /// Factory: new [`EraId`] instance.
    pub const fn new(value: u64) -> EraId {
        EraId(value)
    }
}
