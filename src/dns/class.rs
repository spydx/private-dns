use crate::dns::{BufferSlice, ErrorCondition, FromBytes, ToSizedBytes, ToU16Slice};

#[derive(Debug, Clone)]
pub enum Type {
    A = 1,      // a host address
    NS = 2,     // an authorative name server
    MD = 3,     // a mail destination (obsolete - use MX)
    MF = 4,     // a mail forwarder (obsolete - use MX)
    CNAME = 5,  // the canonical name for an alias
    SOA = 6,    // marks the start of a zone of authorithy
    MB = 7,     // a mailbox domain name (experimental)
    MG = 8,     // a mail group member (exper)
    MR = 9,     // a mail rename domain (exper)
    NULL = 10,  // a null PR (exper)
    WKS = 11,   // a well known service description
    PTR = 12,   // a domain name pointer
    HINFO = 13, // host information
    MINFO = 14, // mailbox or mail list information
    MX = 15,    // mail exchange
    TXT = 16,   // text string
    // QTYPES
    AXFR = 252,  // a request for transfere of an entire zone
    MAILB = 253, // a request for mailbox-related records (MB, MG, MR)
    MAILA = 254, // a request for mail agents RRs (Obsolete - see MX)
    _ALL_ = 255, // a request for all records
}

#[derive(Debug, Clone)]
pub enum Class {
    IN = 1, // the internet
    CS = 2, // the CSNET (obsolete)
    CH = 3, // the CHAOS class
    HS = 4, // Hesiod [Dyer 87]
            // _ALL_ = 255 // Bug?
}

impl FromBytes for Type {
    fn from_bytes(buf: &BufferSlice) -> Result<Self, ErrorCondition> {
        let u16_slice = buf.to_u16();
        match u16::from_be_bytes(u16_slice) {
            1 => Ok(Type::A),
            2 => Ok(Type::NS),
            3 => Ok(Type::MD),
            4 => Ok(Type::MF),
            5 => Ok(Type::CNAME),
            6 => Ok(Type::SOA),
            7 => Ok(Type::MB),
            8 => Ok(Type::MG),
            9 => Ok(Type::MR),
            10 => Ok(Type::NULL),
            11 => Ok(Type::WKS),
            12 => Ok(Type::PTR),
            13 => Ok(Type::HINFO),
            14 => Ok(Type::MINFO),
            15 => Ok(Type::MX),
            16 => Ok(Type::TXT),
            252 => Ok(Type::AXFR),
            253 => Ok(Type::MAILB),
            254 => Ok(Type::MAILA),
            255 => Ok(Type::_ALL_),
            n => Err(ErrorCondition::DeserializationErr(
                format!("Unknown Question Type {}", n).to_string(),
            )),
        }
    }
}

impl ToSizedBytes for Type {
    fn to_bytes(&self) -> [u8; 2] {
        let num = match self {
            Type::A => 1,
            Type::NS => 2,
            Type::MD => 3,
            Type::MF => 4,
            Type::CNAME => 5,
            Type::SOA => 6,
            Type::MB => 7,
            Type::MG => 8,
            Type::MR => 9,
            Type::NULL => 10,
            Type::WKS => 11,
            Type::PTR => 12,
            Type::HINFO => 13,
            Type::MINFO => 14,
            Type::MX => 15,
            Type::TXT => 16,
            Type::AXFR => 252,
            Type::MAILB => 253,
            Type::MAILA => 254,
            Type::_ALL_ => 255,
        };

        u16::to_be_bytes(num)
    }
}

impl FromBytes for Class {
    fn from_bytes(buf: &BufferSlice) -> Result<Self, ErrorCondition> {
        let u16_slice = buf.to_u16();
        let num = u16::from_be_bytes(u16_slice);
        match num {
            1 => Ok(Class::IN),
            2 => Ok(Class::CS),
            3 => Ok(Class::CH),
            4 => Ok(Class::HS),
            _ => Err(ErrorCondition::DeserializationErr(
                format!("Unknown Question Class {}", num).to_string(),
            )),
        }
    }
}

impl ToSizedBytes for Class {
    fn to_bytes(&self) -> [u8; 2] {
        let num = match self {
            Class::IN => 1,
            Class::CS => 2,
            Class::CH => 3,
            Class::HS => 4,
            // Class:_ALL_ => 255, Bug?
        };
        u16::to_be_bytes(num)
    }
}
