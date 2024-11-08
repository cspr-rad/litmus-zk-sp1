use std::usize;

use super::constants;
use super::utils::{CodecError, Encode};
use lutils::bites::Byte;

// Encoder: `bool`.
impl Encode for bool {
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

// Encoder: `i32`.
impl Encode for i32 {
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn serialized_length(&self) -> usize {
        constants::ENCODED_SIZE_I32
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
}

// Encoder: `i64`.
impl Encode for i64 {
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn serialized_length(&self) -> usize {
        constants::ENCODED_SIZE_I64
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
}

// Encoder: `i128`.
impl Encode for i128 {
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn serialized_length(&self) -> usize {
        constants::ENCODED_SIZE_I128
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
}

// Encoder: `unit`.
impl Encode for () {
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError> {
        Ok(Vec::new())
    }

    fn serialized_length(&self) -> usize {
        constants::ENCODED_SIZE_UNIT
    }
}

// Encoder: `u8`.
impl Encode for u8 {
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
impl Encode for u16 {
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn serialized_length(&self) -> usize {
        constants::ENCODED_SIZE_U16
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
}

// Encoder: `u32`.
impl Encode for u32 {
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn serialized_length(&self) -> usize {
        constants::ENCODED_SIZE_U32
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
}

// Encoder: `u64`.
impl Encode for u64 {
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn serialized_length(&self) -> usize {
        constants::ENCODED_SIZE_U64
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
}
