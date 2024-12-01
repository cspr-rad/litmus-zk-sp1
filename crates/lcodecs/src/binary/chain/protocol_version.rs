use super::super::utils::{CodecError, Decode, Encode};
use ltypes::{chain::ProtocolVersion, primitives::SemanticVersion};

// ------------------------------------------------------------------------
// Codec: SemanticVersion.
// ------------------------------------------------------------------------

impl Decode for ProtocolVersion {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, bytes) = SemanticVersion::decode(&bytes).unwrap();

        Ok((Self::new(inner), &bytes))
    }
}

impl Encode for ProtocolVersion {
    fn get_encoded_size(&self) -> usize {
        self.inner().get_encoded_size()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.inner().write_bytes(writer).unwrap();
        Ok(())
    }
}
