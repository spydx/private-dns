use crate::dns::class::{Class, Type};
use crate::dns::{ErrorCondition, ToSizedBytes};
use crate::dns::{FromBytes, ToBytes};

#[derive(Debug)]
pub struct Question {
    pub name: Vec<Label>,
    pub qtype: Type,
    pub qclass: Class,
}

#[derive(Debug, Clone)]
pub struct Label(String);

impl FromBytes for Question {
    fn from_bytes(buf: &[u8]) -> Result<Self, ErrorCondition> {
        let mut index = 0;
        let mut labels: Vec<Label> = Vec::new();

        println!("Labels:");
        while buf[index] != 0 {
            let len = buf[index] as usize;
            index += 1;
            labels.push(Label::new(&buf[index..index + len])?);
            println!("{:?}", labels);
            index += len;
        }

        index += 1;

        let qtype = Type::from_bytes(&buf[index..index + 2])?;
        index += 2;

        let qclass = Class::from_bytes(&buf[index..index + 2])?;

        let question = Question {
            name: labels,
            qtype,
            qclass,
        };

        Ok(question)
    }
}

impl ToBytes for Question {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // write the labels to the buffer s
        for label in &self.name {
            buf.push(label.len() as u8);
            buf.extend_from_slice(label.0.as_bytes());
        }
        // end with 0
        buf.push(0);

        // write the question type and class to the buffer
        buf.extend_from_slice(&self.qtype.to_bytes());
        buf.extend_from_slice(&self.qclass.to_bytes());

        buf
    }
}

impl Label {
    pub fn new(buf: &[u8]) -> Result<Self, ErrorCondition> {
        let slice = buf.to_vec();
        match String::from_utf8(slice) {
            Ok(content) => Ok(Label(content)),
            Err(_) => Err(ErrorCondition::DeserializationErr(format!(
                "Could not create label from bytes"
            ))),
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
