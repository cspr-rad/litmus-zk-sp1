use super::super::utils::{CodecError, Decode, Encode};
use ltypes::{crypto::Digest, primitives::bites::Bytes32};

// ------------------------------------------------------------------------
// Type: Digest.
// ------------------------------------------------------------------------

impl Decode for Digest {
    #[inline(always)]
    fn from_bytes(encoded: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, remainder) = Bytes32::from_bytes(encoded).unwrap();

        Ok((Digest::new(inner), remainder))
    }
}

impl Encode for Digest {
    fn get_encoded_size(&self) -> usize {
        match self {
            Digest::BLAKE2B(inner) => inner.as_slice().len(),
        }
    }

    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        match self {
            Digest::BLAKE2B(inner) => Ok(inner.to_vec()),
        }
    }
}

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    const MSG: &[u8] = "أبو يوسف يعقوب بن إسحاق الصبّاح الكندي‎".as_bytes();
    const MSG_DIGEST_BLAKE2B_HEX: &str =
        "44682ea86b704fb3c65cd16f84a76b621e04bbdb3746280f25cf062220e471b4";

    #[test]
    fn test_new_from_vec() {
        let as_vec = hex::decode(MSG_DIGEST_BLAKE2B_HEX).unwrap();

        let _ = Digest::from_bytes(as_vec.as_slice()).unwrap();

        assert_eq!(as_vec.len(), 32)
    }
}
// 44682ea86b704fb3c65cd16f84a76b621e04bbdb3746280f25cf062220e471b4
