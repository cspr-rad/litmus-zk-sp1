use lcrypto;
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WrappedDigest(pub lcrypto::Digest, pub Vec<u8>);

impl WrappedDigest {
    // Computed digest.
    pub(crate) fn inner(&self) -> &lcrypto::Digest {
        &self.0
    }

    // Data over which digest was computed.
    pub(crate) fn data(&self) -> Vec<u8> {
        self.1.to_owned()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WrappedSignature(pub lcrypto::Signature, pub Vec<u8>, pub Vec<u8>);

impl WrappedSignature {
    // Computed signature.
    pub(crate) fn inner(&self) -> &lcrypto::Signature {
        &self.0
    }

    // Data over which signature was computed.
    pub(crate) fn data(&self) -> Vec<u8> {
        self.1.to_owned()
    }

    // Signature verification key.
    pub(crate) fn vkey(&self) -> Vec<u8> {
        self.2.to_owned()
    }
}
