use super::utils::ToBytes;
use lutils::bites::Byte;

impl<const N: usize> ToBytes for [Byte; N] {
    fn to_bytes(&self) -> Result<Vec<Byte>, super::utils::Error> {
        Ok(self.to_vec())
    }

    fn into_bytes(self) -> Result<Vec<u8>, super::utils::Error>
    where
        Self: Sized,
    {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn serialized_length(&self) -> usize {
        N
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), super::utils::Error> {
        writer.extend_from_slice(&self.as_slice());
        Ok(())
    }
}
