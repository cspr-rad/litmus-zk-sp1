use super::super::utils::{CodecError, Decode, Encode};
use ltypes::{crypto::Digest, primitives::bites::Bytes32};

// ------------------------------------------------------------------------
// Type: Digest.
// ------------------------------------------------------------------------

impl Decode for Digest {
    #[inline(always)]
    fn decode(encoded: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, remainder) = Bytes32::decode(encoded).unwrap();

        Ok((Digest::new(inner), remainder))
    }
}

impl Encode for Digest {
    fn get_encoded_size(&self) -> usize {
        match self {
            Digest::BLAKE2B(inner) => inner.as_slice().len(),
        }
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        match self {
            Digest::BLAKE2B(inner) => {
                inner.write_bytes(writer).unwrap();
            }
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary::utils::assert_codec;
    use hex;

    const MSG_DIGEST_BLAKE2B_HEX: &str =
        "44682ea86b704fb3c65cd16f84a76b621e04bbdb3746280f25cf062220e471b4";

    fn get_digest_bytes() -> Bytes32 {
        Bytes32::from(hex::decode(MSG_DIGEST_BLAKE2B_HEX).unwrap())
    }

    #[test]
    fn test_from_new() {
        assert_codec(&Digest::new(get_digest_bytes()));
    }

    #[test]
    fn test_from_bytes() {
        let bytes_32 = get_digest_bytes();
        let (entity, remainder) = Digest::decode(bytes_32.as_slice()).unwrap();
        assert_eq!(remainder.len(), 0);
        assert_codec(&entity);
    }
}
