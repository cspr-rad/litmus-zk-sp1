use super::utils::{CodecError, Encode};
use ltypes::crypto::Digest;
use lutils::bites::Byte;

impl Encode for Digest {
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError> {
        match self {
            Digest::BLAKE2B(inner) => Ok(inner.to_vec()),
        }
    }

    fn into_bytes(self) -> Result<Vec<u8>, CodecError>
    where
        Self: Sized,
    {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn serialized_length(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.as_slice());
        Ok(())
    }
}

// #[inline(always)]
// fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
//     self.0.to_bytes()
// }

// #[inline(always)]
// fn serialized_length(&self) -> usize {
//     self.0.serialized_length()
// }

// #[inline(always)]
// fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), bytesrepr::Error> {
//     writer.extend_from_slice(&self.0);
//     Ok(())
// }
