use super::super::utils::{CodecError, Decode, Encode};
use ltypes::{
    crypto::Signature,
    primitives::bites::{Bytes32, Bytes33, Bytes64, Bytes65},
};

// ------------------------------------------------------------------------
// Constants.
// ------------------------------------------------------------------------

const TAG_ED25519: u8 = 1;
const TAG_SECP256K1: u8 = 2;

// ------------------------------------------------------------------------
// Codec.
// ------------------------------------------------------------------------

impl Decode for Signature {
    #[inline(always)]
    fn decode(encoded: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, remainder) = Bytes65::decode(encoded).unwrap();
        let sig_tag = inner.data()[0];
        let sig_bytes = Bytes64::from(&inner.data()[1..]);
        Ok((
            match sig_tag {
                TAG_ED25519 => Signature::new_ed25519(sig_bytes),
                TAG_SECP256K1 => Signature::new_secp256k1(sig_bytes),
                _ => panic!("Unsupported signature key type prefix"),
            },
            remainder,
        ))
    }
}

impl Encode for Signature {
    fn get_encoded_size(&self) -> usize {
        Bytes65::len()
    }

    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        let mut result: Vec<u8> = Vec::<u8>::new();
        match self {
            Signature::ED25519(inner) => {
                result.push(TAG_ED25519);
                result.extend(inner.to_vec());
            }
            Signature::SECP256K1(inner) => {
                result.push(TAG_SECP256K1);
                result.extend(inner.to_vec());
            }
        }
        Ok(result)
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
    const PBK_ED25519_HEX: &str =
        "764f83295812c03354e0cd64718a7e50b452696799dc9d6e446338d668f3b2d9";
    const SIG_ED25519_HEX: &str =
        "2fa8e929a7514496545d098e86841463ef66358ff0930073fde3b138f66a2cef5304d884baa693a971d002d7e071f658fb16de8c1e5c80ba5ecea8b3866f8106";
    const PBK_SECP256K1_HEX: &str =
        "03eed4eb0b40b3131679c365e3a23780eabfeaeb01776b0f908223ad1d4bd06f0d";
    const SIG_SECP256K1_HEX: &str =
            "5ed6e5b71fa8f87dfb197a3d85c926d075f0b15651b59224a9a41a9fa1deb8cc2b2de5a8312a310af9b5321f67b744e1b3814994b13ec6db2769e9e6a9cc9364";

    fn get_bytes_ed25519() -> (Bytes32, Bytes32, Bytes64) {
        (
            Bytes32::from(hex::decode(MSG_DIGEST_BLAKE2B_HEX).unwrap()),
            Bytes32::from(hex::decode(PBK_ED25519_HEX).unwrap()),
            Bytes64::from(hex::decode(SIG_ED25519_HEX).unwrap()),
        )
    }

    fn get_bytes_secp256k1() -> (Bytes32, Bytes33, Bytes64) {
        (
            Bytes32::from(hex::decode(MSG_DIGEST_BLAKE2B_HEX).unwrap()),
            Bytes33::from(hex::decode(PBK_SECP256K1_HEX).unwrap()),
            Bytes64::from(hex::decode(SIG_SECP256K1_HEX).unwrap()),
        )
    }

    #[test]
    fn test_setup() {
        let _ = get_bytes_ed25519();
        let _ = get_bytes_secp256k1();
    }

    #[test]
    fn test_from_new_ed25519() {
        let (_, _, sig) = get_bytes_ed25519();

        assert_codec(&Signature::new_ed25519(sig));
    }

    // #[test]
    // fn test_from_new_secp256k1() {
    //     let (_, _, sig) = get_bytes_secp256k1();

    //     assert_codec(&Signature::new(sig.as_slice()));
    // }

    // #[test]
    // fn test_from_bytes_ed25519() {
    //     let bytes_32 = get_digest_bytes();
    //     let (entity, remainder) = Digest::from_bytes(bytes_32.as_slice()).unwrap();
    //     assert_eq!(remainder.len(), 0);
    //     assert_codec(&entity);
    // }
}
