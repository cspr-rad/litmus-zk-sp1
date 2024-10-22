use crate::crypto::{SignatureBytesRaw, VerificationKeyBytes};

/// A signature over a block within the context of some form of consensus process.
pub struct BlockSignature {
    /// Signature over a block issued by an entity, e.g. validator.
    signature: SignatureBytesRaw,

    /// Verification key associated with signing key used to issue block signature.
    verification_key: VerificationKeyBytes,
}

/// Constructors.
impl BlockSignature {
    pub fn new(signature: SignatureBytesRaw, verification_key: VerificationKeyBytes) -> Self {
        Self {
            signature,
            verification_key,
        }
    }
}

/// Accessors.
impl BlockSignature {
    pub fn signature(&self) -> SignatureBytesRaw {
        self.signature
    }
    pub fn verification_key(&self) -> &VerificationKeyBytes {
        &self.verification_key
    }
}
