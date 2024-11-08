use super::utils::Encode;
use ltypes::chain::{Block, BlockHash, BlockV2, BlockV2Body, BlockV2Header};
use lutils::bites::Byte;

impl Encode for Block {
    fn to_bytes(&self) -> Result<Vec<Byte>, super::utils::Error> {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn into_bytes(self) -> Result<Vec<u8>, super::utils::Error>
    where
        Self: Sized,
    {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn serialized_length(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), super::utils::Error> {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }
}

impl Encode for BlockHash {
    fn to_bytes(&self) -> Result<Vec<Byte>, super::utils::Error> {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn into_bytes(self) -> Result<Vec<u8>, super::utils::Error>
    where
        Self: Sized,
    {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn serialized_length(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), super::utils::Error> {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }
}

impl Encode for BlockV2 {
    fn to_bytes(&self) -> Result<Vec<Byte>, super::utils::Error> {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn into_bytes(self) -> Result<Vec<u8>, super::utils::Error>
    where
        Self: Sized,
    {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn serialized_length(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), super::utils::Error> {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }
}

impl Encode for BlockV2Body {
    fn to_bytes(&self) -> Result<Vec<Byte>, super::utils::Error> {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn into_bytes(self) -> Result<Vec<u8>, super::utils::Error>
    where
        Self: Sized,
    {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn serialized_length(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), super::utils::Error> {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }
}

impl Encode for BlockV2Header {
    fn to_bytes(&self) -> Result<Vec<Byte>, super::utils::Error> {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn into_bytes(self) -> Result<Vec<u8>, super::utils::Error>
    where
        Self: Sized,
    {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn serialized_length(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), super::utils::Error> {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }
}
