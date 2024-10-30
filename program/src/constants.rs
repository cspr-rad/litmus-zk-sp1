use litmus_zk_lib::Byte;

// Input bytes stream constants.
pub const DIGEST_TYPE_BLAKE2B: Byte = 0;
pub const SIGNATURE_TYPE_ED25519: Byte = 0;
pub const SIGNATURE_TYPE_SECP256K1: Byte = 1;
pub const VERIFICATION_TYPE_BLOCK: Byte = 2;
pub const VERIFICATION_TYPE_DIGEST: Byte = 0;
pub const VERIFICATION_TYPE_SIGNATURE: Byte = 1;
