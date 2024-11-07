use super::utils::ToBytes;
use ltypes::crypto::Digest;
use lutils::bites::Byte;

impl ToBytes for Digest {
    fn to_bytes(&self) -> Result<Vec<Byte>, super::utils::Error> {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn into_bytes(self) -> Result<Vec<u8>, super::utils::Error>
    where
        Self: Sized,
    {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn serialized_length(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), super::utils::Error> {
        writer.extend_from_slice(&self.as_slice());
        Ok(())
    }
}
