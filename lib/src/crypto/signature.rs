use super::digest::Digest;
use crate::utils::bites::{Byte, Bytes32, Bytes33, Bytes64};

// Length of fixed byte signature array.
const LENGTH_OF_SIGNATURE: usize = 64;

// Raw signature bytes.
pub type SignatureBytesRaw = [u8; LENGTH_OF_SIGNATURE];

/// Raw verification key bytes scoped by ECC algo type.
#[derive(Clone, Debug)]
pub enum VerificationKeyBytes {
    ED25519(Bytes32),
    SECP256K1(Bytes33),
}

impl VerificationKeyBytes {
    /// Verifies an ECC signature over a digest.
    ///
    /// # Arguments
    ///
    /// * `sig` - Signature to be verified.
    /// * `digest` - Digest over a message over which signature was claimed to have been isssued.
    ///
    #[sp1_derive::cycle_tracker]
    pub fn verify_signature_over_digest(&self, sig: SignatureBytesRaw, digest: Digest) {
        match self {
            VerificationKeyBytes::ED25519(vk) => verify_ed25519(digest, sig, vk.to_owned()),
            VerificationKeyBytes::SECP256K1(vk) => verify_sec256k1(digest, sig, vk.to_owned()),
        }
    }
}

fn verify_ed25519(digest: Digest, sig: SignatureBytesRaw, vk: Bytes32) {
    use ed25519_consensus::{Signature, VerificationKey};

    let sig = Signature::try_from(sig.as_slice()).unwrap();
    let vk = VerificationKey::try_from(vk.as_slice()).unwrap();

    assert_eq!(vk.verify(&sig, &digest.as_slice()), Ok(()));
}

fn verify_sec256k1(digest: Digest, sig: SignatureBytesRaw, vk: Bytes33) {
    use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};

    let msg = Message::from_digest_slice(&digest.as_slice()).unwrap();
    let pbk = PublicKey::from_slice(vk.as_slice()).unwrap();
    let sig = Signature::from_compact(&sig).unwrap();

    assert_eq!(Secp256k1::new().verify_ecdsa(&msg, &sig, &pbk), Ok(()));
}
