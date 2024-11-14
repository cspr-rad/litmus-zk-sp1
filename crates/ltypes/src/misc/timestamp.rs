use serde::{Deserialize, Serialize};
use std::time::SystemTime;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct Timestamp(u128);

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Timestamp {
    pub fn new(ms_since_epoch: u128) -> Self {
        Self(ms_since_epoch)
    }

    // Returns instance hydrated from current system time.
    pub fn now() -> Self {
        Timestamp(SystemTime::UNIX_EPOCH.elapsed().unwrap().as_millis())
    }
}
