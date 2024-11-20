use lcrypto::{Digest, Signature, VerificationKey};
use ltypes::chain::{BlockWithProofs, ChainNameDigest};
use serde::{Deserialize, Serialize};

// V1 block with associated proof set.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WrappedBlockV1WithProofs(pub BlockWithProofs);

impl WrappedBlockV1WithProofs {
    // Block and set of associated finality signatures.
    pub(crate) fn inner(&self) -> &BlockWithProofs {
        &self.0
    }
}

// V2 block with associated proof set.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WrappedBlockV2WithProofs(pub BlockWithProofs, pub ChainNameDigest);

impl WrappedBlockV2WithProofs {
    // Name of chain associated with block.
    pub(crate) fn chain_name_digest(&self) -> &ChainNameDigest {
        &self.1
    }

    // Block and set of associated finality signatures.
    pub(crate) fn inner(&self) -> &BlockWithProofs {
        &self.0
    }
}

// Cryptographic digest.
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

// Cryptographic signature.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WrappedSignature(pub Signature, pub VerificationKey, pub Digest);

impl WrappedSignature {
    // Data over which signature was computed.
    pub(crate) fn msg(&self) -> &Digest {
        &self.2
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
