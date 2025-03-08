use crate::dns::{
    header::Header, 
    question::Question,
    ToBytes, ToResponseHeader
};

use super::answer::ResourceRecord;

pub struct Response {
    response_header: Header,
    question: Question,
    answer: ResourceRecord
}

impl ToBytes for Response {
    fn to_bytes(&self) -> Vec<u8> {
        let mut response: Vec<u8> = Vec::new();

        response.extend_from_slice(&self.response_header.to_bytes());
        response.extend_from_slice(&self.question.to_bytes());
        response.extend_from_slice(&self.answer.to_bytes());

        response
    }
}


pub struct ResponseBuilder {
    response_header: Option<Header>,
    question: Option<Question>,
    answer: Option<ResourceRecord>
}

impl ResponseBuilder {
    pub fn new() -> ResponseBuilder {
        ResponseBuilder{
            response_header: None,
            question: None,
            answer: Some(ResourceRecord::default()),
        }
    }

    pub fn add_header(mut self, header: Header) -> ResponseBuilder {
        self.response_header = Some(header.to_response_header());
        self
    }

    pub fn add_question(mut self, question: Question) -> ResponseBuilder {
        self.question = Some(question);
        self
    }

    pub fn build(self) -> Response {
        // There is a bug here
        // No non checking, and therfore this can fail.
        Response { 
            response_header: self.response_header.unwrap(),
            question: self.question.unwrap(),
            answer: self.answer.unwrap() 
        }
    }
}