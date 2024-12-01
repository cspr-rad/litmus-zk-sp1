use super::super::utils::{CodecError, Decode, Encode};
use ltypes::chain::ProtocolVersion;
use ltypes::{
    crypto::VerificationKey,
    primitives::bites::{Bytes32, Bytes33, Bytes64, Bytes65},
};

// ------------------------------------------------------------------------
// Codec: SemanticVersion.
// ------------------------------------------------------------------------

impl Decode for VerificationKey {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!()
    }
}

impl Encode for VerificationKey {
    fn get_encoded_size(&self) -> usize {
        unimplemented!()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        unimplemented!()
    }
}
