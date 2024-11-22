use crate::binary::utils::{CodecError, Decode, Encode};
use ltypes::chain::{BlockHash, BlockHeight, EraId};

// ------------------------------------------------------------------------
// Type: BlockHash.
// ------------------------------------------------------------------------

impl Decode for BlockHash {
    #[inline(always)]
    fn from_bytes(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for BlockHash {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn get_encoded_size(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }
}

// ------------------------------------------------------------------------
// Type: EraId.
// ------------------------------------------------------------------------

impl Encode for EraId {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        self.inner().to_bytes()
    }

    fn get_encoded_size(&self) -> usize {
        self.inner().get_encoded_size()
    }
}

impl Decode for EraId {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, bytes) = u64::from_bytes(&bytes).unwrap();

        Ok((Self::new(inner), &bytes))
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
    pub fn era_id() -> impl Strategy<Value = EraId> {
        any::<u64>().prop_map(EraId::new)
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use crate::binary::utils::assert_codec;

    proptest! {
        #[test]
        fn codec_era_id(era_id in arbs::era_id()) {
            assert_codec(&era_id);
        }
    }
}

// let serialized = ToBytes::to_bytes(t).expect("Unable to serialize data");
// assert_eq!(
//     serialized.len(),
//     t.serialized_length(),
//     "\nLength of serialized data: {},\nserialized_length() yielded: {},\n t is {:?}",
//     serialized.len(),
//     t.serialized_length(),
//     t
// );
// let mut written_bytes = vec![];
// t.write_bytes(&mut written_bytes)
//     .expect("Unable to serialize data via write_bytes");
// assert_eq!(serialized, written_bytes);

// let deserialized_from_slice =
//     deserialize_from_slice(&serialized).expect("Unable to deserialize data");
// assert_eq!(*t, deserialized_from_slice);

// let deserialized = deserialize::<T>(serialized).expect("Unable to deserialize data");
// assert_eq!(*t, deserialized);
