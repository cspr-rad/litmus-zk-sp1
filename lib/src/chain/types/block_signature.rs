use crate::crypto::{SignatureBytesRaw, VerificationKeyBytes};

/// Type declaration.
/// A signature over a block within the context of some form of consensus process.
pub struct BlockSignature {
    /// A validator's signature over a block.
    signature: SignatureBytesRaw,

    /// A validator's verification key.
    validator_vkey: VerificationKeyBytes,
}

/// Type field accessors.
impl BlockSignature {
    pub fn signature(&self) -> SignatureBytesRaw {
        self.signature
    }
    pub fn validator_vkey(&self) -> VerificationKeyBytes {
        self.validator_vkey
    }
}

/// Type constructors.
impl BlockSignature {
    pub fn new(signature: SignatureBytesRaw, validator_vkey: VerificationKeyBytes) -> Self {
        Self {
            signature,
            validator_vkey,
        }
    }
}
