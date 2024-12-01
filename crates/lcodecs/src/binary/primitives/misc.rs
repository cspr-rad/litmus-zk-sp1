use super::super::constants;
use crate::binary::utils::{CodecError, Decode, Encode};

// ------------------------------------------------------------------------
// Codec: bool.
// ------------------------------------------------------------------------

impl Decode for bool {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        match bytes.split_first() {
            None => Err(CodecError::EarlyEndOfStream),
            Some((byte, rem)) => match byte {
                1 => Ok((true, rem)),
                0 => Ok((false, rem)),
                _ => Err(CodecError::Formatting),
            },
        }
    }
}

impl Encode for bool {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_BOOL
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.push(u8::from(*self));
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: Option<T>.
// ------------------------------------------------------------------------

impl<T: Decode> Decode for Option<T> {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (tag, rem) = u8::decode(bytes)?;
        match tag {
            constants::TAG_OPTION_NONE => Ok((None, rem)),
            constants::TAG_OPTION_SOME => {
                let (t, rem) = T::decode(rem)?;
                Ok((Some(t), rem))
            }
            _ => Err(CodecError::Formatting),
        }
    }
}

impl<T: Encode> Encode for Option<T> {
    fn get_encoded_size(&self) -> usize {
        match self {
            Some(v) => constants::ENCODED_SIZE_U8 + v.get_encoded_size(),
            None => constants::ENCODED_SIZE_U8,
        }
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        match self {
            None => {
                writer.push(constants::TAG_OPTION_NONE);
            }
            Some(inner) => {
                writer.push(constants::TAG_OPTION_SOME);
                inner.write_bytes(writer).unwrap();
            }
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: Result<T, E>.
// ------------------------------------------------------------------------

impl<T: Decode, E: Decode> Decode for Result<T, E> {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (variant, rem) = u8::decode(bytes)?;
        match variant {
            constants::TAG_RESULT_ERR => {
                let (value, rem) = E::decode(rem)?;
                Ok((Err(value), rem))
            }
            constants::TAG_RESULT_OK => {
                let (value, rem) = T::decode(rem)?;
                Ok((Ok(value), rem))
            }
            _ => Err(CodecError::Formatting),
        }
    }
}

impl<T: Encode, E: Encode> Encode for Result<T, E> {
    fn get_encoded_size(&self) -> usize {
        match self {
            Err(error) => constants::ENCODED_SIZE_U8 + error.get_encoded_size(),
            Ok(value) => constants::ENCODED_SIZE_U8 + value.get_encoded_size(),
        }
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        match self {
            Err(error) => {
                writer.push(constants::TAG_RESULT_ERR);
                error.write_bytes(writer).unwrap();
            }
            Ok(value) => {
                writer.push(constants::TAG_RESULT_OK);
                value.write_bytes(writer).unwrap();
            }
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: unit.
// ------------------------------------------------------------------------

impl Decode for () {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        Ok(((), bytes))
    }
}

impl Encode for () {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_UNIT
    }

    fn write_bytes(&self, _: &mut Vec<u8>) -> Result<(), CodecError> {
        Ok(())
    }
}
