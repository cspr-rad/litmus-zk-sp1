use super::utils::{
    encode_byte_slice, get_encoded_size_of_byte_slice, safe_split_at, write_byte_slice, CodecError,
    Decode, Encode,
};
use lutils::bites::Byte;

// ------------------------------------------------------------------------
// Type: str.
// ------------------------------------------------------------------------

impl Encode for str {
    #[inline(always)]
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        encode_byte_slice(self.as_bytes())
    }

    #[inline(always)]
    fn get_encoded_size(&self) -> usize {
        get_encoded_size_of_byte_slice(self.as_bytes())
    }

    #[inline]
    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        write_byte_slice(self.as_bytes(), writer)?;
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Type: &str.
// ------------------------------------------------------------------------

impl Encode for &str {
    #[inline(always)]
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        (*self).to_bytes()
    }

    #[inline(always)]
    fn get_encoded_size(&self) -> usize {
        (*self).get_encoded_size()
    }

    #[inline]
    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        write_byte_slice(self.as_bytes(), writer)?;
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Type: String.
// ------------------------------------------------------------------------

impl Decode for String {
    fn from_bytes(bytes: &[Byte]) -> Result<(Self, &[Byte]), CodecError> {
        let (size, remainder) = u32::from_bytes(bytes)?;
        let (str_bytes, remainder) = safe_split_at(remainder, size as usize)?;
        let result = String::from_utf8(str_bytes.to_vec()).map_err(|_| CodecError::Formatting)?;
        Ok((result, remainder))
    }
}

impl Encode for String {
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError> {
        let bytes = self.as_bytes();
        encode_byte_slice(bytes)
    }

    fn get_encoded_size(&self) -> usize {
        get_encoded_size_of_byte_slice(self.as_bytes())
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), CodecError> {
        write_byte_slice(self.as_bytes(), writer)?;
        Ok(())
    }
}
