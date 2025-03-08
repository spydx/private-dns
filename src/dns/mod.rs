pub mod answer;
pub mod class;
pub mod header;
pub mod question;
pub mod response;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorCondition {
    #[error("Serialization Error : {0}")]
    SerializationErr(String),
    #[error("Deserialization Error: {0}")]
    DeserializationErr(String),
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait FromBytes
where
    Self: Sized,
{
    fn from_bytes(buf: &BufferSlice) -> Result<Self, ErrorCondition>;
}

pub trait ToSizedBytes {
    fn to_bytes(&self) -> [u8; 2];
}

type BufferSlice = [u8];

trait ToU16Slice {
    fn to_u16(&self) -> [u8; 2];
}

impl ToU16Slice for BufferSlice {
    fn to_u16(&self) -> [u8; 2] {
        [self[0], self[1]]
    }
}

trait ToResponseHeader {
    fn to_response_header(&self) -> Self;
}
