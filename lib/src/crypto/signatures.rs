use super::digests::DigestBytesRaw;

// Length of fixed byte signature array.
const LENGTH_OF_SIGNATURE: usize = 64;

// Length of fixed byte verification key array -> ed25519.
const LENGTH_OF_VERIFIER_KEY_ED25519: usize = 32;

// Length of fixed byte verification key array -> secp256k1 (compressed).
const LENGTH_OF_VERIFIER_KEY_SECP256K1: usize = 33;

// Raw signature bytes.
pub type SignatureBytesRaw = [u8; LENGTH_OF_SIGNATURE];

/// Raw verification key bytes scoped by ECC algo type.
#[derive(Clone, Debug)]
pub enum VerificationKeyBytes {
    ED25519(VerificationKeyBytesRawEd25519),
    SECP256K1(VerificationKeyBytesRawSecp256k1),
}

// Raw verification key bytes -> ed25519.
type VerificationKeyBytesRawEd25519 = [u8; LENGTH_OF_VERIFIER_KEY_ED25519];

// Raw verification key bytes -> secp256k1.
type VerificationKeyBytesRawSecp256k1 = [u8; LENGTH_OF_VERIFIER_KEY_SECP256K1];

/// Verifies an ECC signature.
///
/// # Arguments
///
/// * `digest` - Digest over a message.
/// * `sig` - Signature to be verified.
/// * `vk` - Verification key associated over message digest signing key.
///
#[sp1_derive::cycle_tracker]
pub fn verify(digest: DigestBytesRaw, sig: SignatureBytesRaw, vk: VerificationKeyBytes) {
    match vk {
        VerificationKeyBytes::ED25519(vk) => verify_ed25519(digest, sig, vk),
        VerificationKeyBytes::SECP256K1(vk) => verify_sec256k1(digest, sig, vk),
    }
}

fn verify_ed25519(
    digest: DigestBytesRaw,
    sig: SignatureBytesRaw,
    vk: VerificationKeyBytesRawEd25519,
) {
    use ed25519_consensus::{Signature, VerificationKey};

    let sig = Signature::try_from(sig.as_slice()).unwrap();
    let vk = VerificationKey::try_from(vk.as_slice()).unwrap();

    assert_eq!(vk.verify(&sig, &digest), Ok(()));
}

fn verify_sec256k1(
    digest: DigestBytesRaw,
    sig: SignatureBytesRaw,
    vk: VerificationKeyBytesRawSecp256k1,
) {
    use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};

    let msg = Message::from_digest_slice(digest.as_slice()).unwrap();
    let pbk = PublicKey::from_slice(vk.as_slice()).unwrap();
    let sig = Signature::from_compact(&sig).unwrap();

    assert_eq!(Secp256k1::new().verify_ecdsa(&msg, &sig, &pbk), Ok(()));
}
