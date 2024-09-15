const LENGTH_OF_DIGEST: usize = 32;
const LENGTH_OF_SIGNATURE: usize = 64;
const LENGTH_OF_VERIFIER_KEY_ED25519: usize = 32;
const LENGTH_OF_VERIFIER_KEY_SECP256K1: usize = 33;

// Raw digest bytes.
type DigestBytesRaw = [u8; LENGTH_OF_DIGEST];

// Raw signature bytes.
type SignatureBytesRaw = [u8; LENGTH_OF_SIGNATURE];

/// Raw digest bytes scoped by hashing algo type.
pub enum DigestBytes {
    BLAKE2B(DigestBytesRaw),
}

/// Raw verification key bytes scoped by ECC algo type.
pub enum VerificationKeyBytes {
    ED25519([u8; LENGTH_OF_VERIFIER_KEY_ED25519]),
    SECP256K1([u8; LENGTH_OF_VERIFIER_KEY_SECP256K1]),
}

/// Verifies a digest over passed data.
///
/// # Arguments
///
/// * `data` - Data over which to generate a digest.
/// * `digest` - Digest to be verified.
///
#[sp1_derive::cycle_tracker]
pub fn verify_digest(data: Vec<u8>, digest: DigestBytes) {
    match digest {
        DigestBytes::BLAKE2B(digest) => {
            use blake2::{
                digest::{Update, VariableOutput},
                Blake2bVar,
            };

            let mut hasher = Blake2bVar::new(LENGTH_OF_DIGEST).unwrap();
            hasher.update(&data);
            let mut buffer = [0u8; LENGTH_OF_DIGEST];
            hasher.finalize_variable(&mut buffer).unwrap();

            assert_eq!(digest, buffer);
        }
    }
}

/// Verifies an ECC signature.
///
/// # Arguments
///
/// * `digest` - Digest over a message.
/// * `sig` - Signature to be verified.
/// * `vk` - Verification key associated over message digest signing key.
///
#[sp1_derive::cycle_tracker]
pub fn verify_signature(digest: DigestBytesRaw, sig: SignatureBytesRaw, vk: VerificationKeyBytes) {
    match vk {
        VerificationKeyBytes::ED25519(vk) => {
            use ed25519_consensus::{Signature, VerificationKey};

            let sig = Signature::try_from(sig.as_slice()).unwrap();
            let vk = VerificationKey::try_from(vk.as_slice()).unwrap();

            assert_eq!(vk.verify(&sig, &digest), Ok(()));
        }
        VerificationKeyBytes::SECP256K1(vk) => {
            use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};

            let msg = Message::from_digest_slice(&digest).unwrap();
            let pbk = PublicKey::from_slice(vk.as_slice()).unwrap();
            let sig = Signature::from_compact(&sig).unwrap();

            assert_eq!(
                Secp256k1::verification_only().verify_ecdsa(&msg, &sig, &pbk),
                Ok(())
            );
        }
    }
}
