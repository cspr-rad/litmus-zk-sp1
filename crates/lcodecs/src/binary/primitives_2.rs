use super::constants;
use super::utils::{allocate_buffer, CodecError, Decode, Encode};
use lutils::bites::Byte;

// ------------------------------------------------------------------------
// Type: Option<T>.
// ------------------------------------------------------------------------

impl<T: Decode> Decode for Option<T> {
    fn from_bytes(bytes: &[Byte]) -> Result<(Self, &[Byte]), CodecError> {
        let (tag, rem) = u8::from_bytes(bytes)?;
        match tag {
            constants::TAG_OPTION_NONE => Ok((None, rem)),
            constants::TAG_OPTION_SOME => {
                let (t, rem) = T::from_bytes(rem)?;
                Ok((Some(t), rem))
            }
            _ => Err(CodecError::Formatting),
        }
    }
}

impl<T: Encode> Encode for Option<T> {
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError> {
        match self {
            None => Ok(vec![constants::TAG_OPTION_NONE]),
            Some(v) => {
                let mut result = allocate_buffer(self)?;
                result.push(constants::TAG_OPTION_SOME);

                let mut value = v.to_bytes()?;
                result.append(&mut value);

                Ok(result)
            }
        }
    }

    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U8
            + match self {
                Some(v) => v.get_encoded_size(),
                None => 0,
            }
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), CodecError> {
        match self {
            None => writer.push(constants::TAG_OPTION_NONE),
            Some(v) => {
                writer.push(constants::TAG_OPTION_SOME);
                v.write_bytes(writer)?;
            }
        };
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Type: Result<T, E>.
// ------------------------------------------------------------------------

impl<T: Decode, E: Decode> Decode for Result<T, E> {
    fn from_bytes(bytes: &[Byte]) -> Result<(Self, &[Byte]), CodecError> {
        let (variant, rem) = u8::from_bytes(bytes)?;
        match variant {
            constants::TAG_RESULT_ERR => {
                let (value, rem) = E::from_bytes(rem)?;
                Ok((Err(value), rem))
            }
            constants::TAG_RESULT_OK => {
                let (value, rem) = T::from_bytes(rem)?;
                Ok((Ok(value), rem))
            }
            _ => Err(CodecError::Formatting),
        }
    }
}

impl<T: Encode, E: Encode> Encode for Result<T, E> {
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError> {
        let mut result = allocate_buffer(self)?;
        let (variant, mut value) = match self {
            Err(error) => (constants::TAG_RESULT_ERR, error.to_bytes()?),
            Ok(result) => (constants::TAG_RESULT_OK, result.to_bytes()?),
        };
        result.push(variant);
        result.append(&mut value);
        Ok(result)
    }

    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U8
            + match self {
                Ok(ok) => ok.get_encoded_size(),
                Err(error) => error.get_encoded_size(),
            }
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), CodecError> {
        match self {
            Err(error) => {
                writer.push(constants::TAG_RESULT_ERR);
                error.write_bytes(writer)?;
            }
            Ok(result) => {
                writer.push(constants::TAG_RESULT_OK);
                result.write_bytes(writer)?;
            }
        };
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
mod proptests {
    use crate::binary::utils::test_serialization_roundtrip;
    use proptest::prelude::*;

    proptest! {

        #[test]
        fn test_option_t(_ in any::<bool>()) {
            unimplemented!("test_option_t");
        }

        #[test]
        fn test_result_t_e(_ in any::<bool>()) {
            unimplemented!("test_result_t_e");
        }
    }
}
