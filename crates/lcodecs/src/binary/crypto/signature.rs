use super::super::utils::{CodecError, Decode, Encode};
use ltypes::crypto::Signature;

// ------------------------------------------------------------------------
// Type: Signature.
// ------------------------------------------------------------------------

impl Decode for Signature {
    #[inline(always)]
    fn from_bytes(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for Signature {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        match self {
            Signature::ED25519(inner) => Ok(inner.to_vec()),
            Signature::SECP256K1(inner) => Ok(inner.to_vec()),
        }
    }

    fn get_encoded_size(&self) -> usize {
        match self {
            Signature::ED25519(inner) => inner.as_slice().len(),
            Signature::SECP256K1(inner) => inner.as_slice().len(),
        }
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.as_slice());
        Ok(())
    }
}
