use crate::dns::ToBytes;
use crate::dns::class::{Class, Type};

use super::ToSizedBytes;

#[derive(Debug, Clone)]
pub struct ResourceRecord {
    pub name: String,
    pub rtype: Type,
    pub rclass: Class,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
}

impl ResourceRecord {
    pub const MAX_DNS_MESSAGE_SIZE: usize = 1;
}
impl ToBytes for ResourceRecord {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(ResourceRecord::MAX_DNS_MESSAGE_SIZE);
        self.name.split(".").for_each(|label| {
            buf.push(label.len() as u8);
            buf.extend_from_slice(label.as_bytes());
        });
        buf.push(0);

        buf.extend_from_slice(&self.rtype.to_bytes());
        buf.extend_from_slice(&self.rclass.to_bytes());
        buf.extend_from_slice(&self.ttl.to_be_bytes());
        buf.extend_from_slice(&self.rdlength.to_be_bytes());
        buf.extend_from_slice(&self.rdata);

        buf
    }
}

impl Default for ResourceRecord {
    fn default() -> Self {
        Self {
            name: String::from("www.kefo.no"),
            rtype: Type::A,
            rclass: Class::IN,
            ttl: 60,
            rdlength: 4,
            rdata: vec![76,76,21,21],
        }
    }
}
