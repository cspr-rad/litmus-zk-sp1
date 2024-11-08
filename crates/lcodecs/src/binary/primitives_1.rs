use super::constants;
use super::utils::{Error as CodecError, ToBytes};
use lutils::bites::Byte;

// Encoder: `unit`.
impl ToBytes for () {
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError> {
        Ok(Vec::new())
    }

    fn serialized_length(&self) -> usize {
        constants::ENCODED_SIZE_UNIT
    }
}

// Encoder: `bool`.
impl ToBytes for bool {
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError> {
        u8::from(*self).to_bytes()
    }

    fn serialized_length(&self) -> usize {
        constants::ENCODED_SIZE_BOOL
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), CodecError> {
        writer.push(*self as u8);
        Ok(())
    }
}

// Encoder: `u8`.
impl ToBytes for u8 {
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError> {
        Ok(vec![*self])
    }

    fn serialized_length(&self) -> usize {
        constants::ENCODED_SIZE_U8
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), CodecError> {
        writer.push(*self);
        Ok(())
    }
}

// Encoder: `u16`.
impl ToBytes for u16 {
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError> {
        Ok(vec![*self])
    }

    fn serialized_length(&self) -> usize {
        constants::ENCODED_SIZE_U16
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), CodecError> {
        writer.push(*self);
        Ok(())
    }
}
