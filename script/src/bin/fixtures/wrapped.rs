use lcrypto::{Signature, VerificationKey};
use ltypes::chain::{BlockWithProofs, ChainNameDigest};
use serde::{Deserialize, Serialize};

// Wrapped domain type -> foreign trait constraint workaround.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WrappedBlockWithProofs(pub BlockWithProofs, pub ChainNameDigest);

impl WrappedBlockWithProofs {
    // Nmae of chain associated with block.
    pub(crate) fn chain_name_digest(&self) -> &ChainNameDigest {
        &self.1
    }

    // Block and set of associated finality signatures.
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
    pub(crate) fn msg(&self) -> Vec<u8> {
        self.1.to_owned()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WrappedSignature(pub Signature, pub VerificationKey, pub Vec<u8>);

impl WrappedSignature {
    // Data over which signature was computed.
    pub(crate) fn msg(&self) -> Vec<u8> {
        self.2.to_owned()
    }

    // Computed signature.
    pub(crate) fn sig(&self) -> &Signature {
        &self.0
    }

    // Signature verification key.
    pub(crate) fn vkey(&self) -> &VerificationKey {
        &self.1
    }
}
