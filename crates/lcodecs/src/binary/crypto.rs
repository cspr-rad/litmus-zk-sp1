use super::utils::{CodecError, Decode, Encode};
use ltypes::crypto::{Digest, Signature};

// ------------------------------------------------------------------------
// Type: Digest.
// ------------------------------------------------------------------------

impl Decode for Digest {
    #[inline(always)]
    fn from_bytes(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!("Digest::decode");
        // Decode::from_bytes(bytes).map(|(arr, rem)| (Digest::new(arr), rem))
    }
}

impl Encode for Digest {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        match self {
            Digest::BLAKE2B(inner) => Ok(inner.to_vec()),
        }
    }

    fn get_encoded_size(&self) -> usize {
        match self {
            Digest::BLAKE2B(inner) => inner.as_slice().len(),
        }
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.as_slice());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Type: Signature.
// ------------------------------------------------------------------------

impl Decode for Signature {
    #[inline(always)]
    fn from_bytes(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!("Signature::decode");
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
