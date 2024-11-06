use lcrypto::{Signature, VerificationKey};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// A signature over a block within the context of some form of consensus process.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BlockSignature {
    /// Signature over a block issued by an entity, e.g. validator.
    signature: Signature,

    /// Verification key associated with signing key used to issue block signature.
    verification_key: VerificationKey,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl BlockSignature {
    pub fn new(signature: Signature, verification_key: VerificationKey) -> Self {
        Self {
            signature,
            verification_key,
        }
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl BlockSignature {
    pub fn signature(&self) -> Signature {
        self.signature
    }
    pub fn verification_key(&self) -> &VerificationKey {
        &self.verification_key
    }
}
