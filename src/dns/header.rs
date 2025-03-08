use crate::dns::{BufferSlice, ErrorCondition, ToU16Slice};
use crate::dns::{FromBytes, ToBytes, ToResponseHeader};

#[derive(Debug)]
pub struct Header {
    pub id: u16,      // identifier
    pub qr: bool,     // 0 for query, 1 for response
    pub opcode: u8,   // 0 for standard query
    pub aa: bool,     // authorative answer
    pub tc: bool,     // truncated message
    pub rd: bool,     // recursion desired
    pub ra: bool,     // recursion available
    pub z: u8,        // reserved for future use
    pub rcode: u8,    // response code
    pub qdcount: u16, // numbre of entried in question
    pub ancount: u16, // number of entries in answer section
    pub nscount: u16, // number of entries in authority section
    pub arcount: u16, // number of entries in additional section
}

impl Header {
    pub const DNS_HEADER_SIZE: usize = 12;
}

impl ToBytes for Header {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Header::DNS_HEADER_SIZE);

        buf.extend_from_slice(&self.id.to_be_bytes());
        buf.push(
            (self.qr as u8) << 7
                | self.opcode << 3
                | (self.aa as u8) << 2
                | (self.tc as u8) << 1
                | self.rd as u8,
        );
        buf.push((self.ra as u8) << 7 | self.z << 4 | self.rcode);

        buf.extend_from_slice(&self.qdcount.to_be_bytes());
        buf.extend_from_slice(&self.ancount.to_be_bytes());
        buf.extend_from_slice(&self.nscount.to_be_bytes());
        buf.extend_from_slice(&self.arcount.to_be_bytes());

        buf
    }
}

impl FromBytes for Header {
    fn from_bytes(buf: &BufferSlice) -> Result<Self, ErrorCondition> {
        if buf.len() < Header::DNS_HEADER_SIZE {
            return Err(ErrorCondition::DeserializationErr(
                "Buffer length is less than header length".to_string(),
            ));
        }

        let u16_slice = buf.to_u16();
        let header = Header {
            id: u16::from_be_bytes(u16_slice),
            qr: (buf[2] & 0b1000_0000) != 0,
            opcode: (buf[2] & 0b0111_1000) >> 3,
            aa: (buf[2] & 0b0000_0100) != 0,
            tc: (buf[2] & 0b0000_0010) != 0,
            rd: (buf[2] & 0b0000_0001) != 0,
            ra: (buf[2] & 0b1000_0000) != 0,
            z: (buf[3] & 0b0111_0000) >> 4,
            rcode: buf[3] & 0b0000_1111,
            qdcount: u16::from_be_bytes([buf[4], buf[5]]),
            ancount: u16::from_be_bytes([buf[6], buf[7]]),
            nscount: u16::from_be_bytes([buf[8], buf[9]]),
            arcount: u16::from_be_bytes([buf[10], buf[11]]),
        };

        Ok(header)
    }
}

impl ToResponseHeader for Header {
    fn to_response_header(&self) -> Self {
        Header {
            id: self.id,
            qr: true,
            opcode: self.opcode,
            aa: false,
            tc: false,
            rd: self.rd,
            ra: false,
            z: 0,
            rcode: if self.opcode == 0 { 0 } else { 4 },
            qdcount: 1, // Question cound we assume is 1
            ancount: 1, // Answer cound is 1
            nscount: 0, // Name server count i 0
            arcount: 0, // Additional record count is 0
        }
    }
}