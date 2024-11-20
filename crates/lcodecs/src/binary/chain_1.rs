use super::utils::Encode;
use ltypes::chain::EraId;

impl Encode for EraId {
    fn to_bytes(&self) -> Result<Vec<u8>, super::utils::CodecError> {
        self.inner().to_bytes()
    }

    fn into_bytes(self) -> Result<Vec<u8>, super::utils::CodecError>
    where
        Self: Sized,
    {
        unimplemented!("Encode for EraId::into_bytes");
    }

    fn get_encoded_size(&self) -> usize {
        unimplemented!("Encode for EraId::get_encoded_size");
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), super::utils::CodecError> {
        unimplemented!("Encode for EraId::write_bytes");
    }
}

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::u64;

    use super::super::encode;
    use ltypes::chain::EraId;

    #[test]
    fn test_new() {
        for (f, g) in [(u64::MAX, [255; 8]), (u64::MIN, [0; 8])] {
            let encoded = encode::<EraId>(&EraId::new(f));
            assert_eq!(encoded, g);
        }
    }
}
