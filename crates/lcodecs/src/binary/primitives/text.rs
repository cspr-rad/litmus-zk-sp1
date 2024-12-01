use crate::binary::{
    constants,
    utils::{
        encode_byte_slice, get_encoded_size_of_byte_slice, safe_split_at, CodecError, Decode,
        Encode,
    },
};

// ------------------------------------------------------------------------
// Codec: str.
// ------------------------------------------------------------------------

impl Encode for str {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U32 + self.as_bytes().len()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(encode_byte_slice(self.as_bytes()).unwrap().as_slice());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: &str.
// ------------------------------------------------------------------------

impl Decode for &str {
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!()
    }
}

impl Encode for &str {
    fn get_encoded_size(&self) -> usize {
        (*self).get_encoded_size()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice((*self).encode().unwrap().as_slice());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: String.
// ------------------------------------------------------------------------

impl Decode for String {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (size, remainder) = u32::decode(bytes)?;
        let (str_bytes, remainder) = safe_split_at(remainder, size as usize)?;
        let result = String::from_utf8(str_bytes.to_vec()).map_err(|_| CodecError::Formatting)?;
        Ok((result, remainder))
    }
}

impl Encode for String {
    fn get_encoded_size(&self) -> usize {
        get_encoded_size_of_byte_slice(self.as_bytes())
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(encode_byte_slice(self.as_bytes()).unwrap().as_slice());
        Ok(())
    }
}
