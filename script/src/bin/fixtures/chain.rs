use ltypes::chain::BlockWithProofs;
use serde::{Deserialize, Serialize};

// Wrapped domain type -> foreign trait constraint workaround.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WrappedBlockWithProofs(pub BlockWithProofs);

impl WrappedBlockWithProofs {
    pub(crate) fn inner(&self) -> &BlockWithProofs {
        &self.0
    }
}
