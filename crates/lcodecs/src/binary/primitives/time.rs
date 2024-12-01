use super::super::utils::{CodecError, Decode, Encode};
use crate::binary::decode;
use ltypes::primitives::time::Timestamp;

// ------------------------------------------------------------------------
// Type: Digest.
// ------------------------------------------------------------------------

impl Decode for Timestamp {
    #[inline(always)]
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, bytes) = u128::from_bytes(&bytes).unwrap();

        Ok((Self::new(inner), &bytes))
    }
}

impl Encode for Timestamp {
    fn get_encoded_size(&self) -> usize {
        self.inner().get_encoded_size()
    }

    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        Ok(self.inner().to_le_bytes().to_vec())
    }
}

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
use proptest::prelude::*;

#[cfg(test)]
mod arbs {
    use super::*;

    #[cfg(test)]
    pub fn timestamp() -> impl Strategy<Value = Timestamp> {
        any::<u128>().prop_map(Timestamp::new)
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use crate::binary::utils::assert_codec;

    proptest! {
        #[test]
        fn codec_timestamp(timestamp in arbs::timestamp()) {
            assert_codec(&timestamp);
        }
    }
}
