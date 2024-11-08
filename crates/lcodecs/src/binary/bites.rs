use super::utils::Encode;
use lutils::bites::Byte;

impl<const N: usize> Encode for [Byte; N] {
    fn to_bytes(&self) -> Result<Vec<Byte>, super::utils::CodecError> {
        Ok(self.to_vec())
    }

    fn into_bytes(self) -> Result<Vec<u8>, super::utils::CodecError>
    where
        Self: Sized,
    {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn serialized_length(&self) -> usize {
        N
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), super::utils::CodecError> {
        writer.extend_from_slice(&self.as_slice());
        Ok(())
    }
}
