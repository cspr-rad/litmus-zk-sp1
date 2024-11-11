use std::time::SystemTime;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Timestamp(u128);

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Timestamp {
    fn new(ms_since_epoch: u128) -> Self {
        Self(ms_since_epoch)
    }

    // Returns instance hydrated from current system time.
    pub fn now() -> Self {
        Timestamp(SystemTime::UNIX_EPOCH.elapsed().unwrap().as_millis())
    }
}
