use crate::dns::header::Header;
use crate::dns::question::{Answer, Question};
use crate::dns::record::ResourceRecord;
use crate::dns::MessagePacket;
use bytes::BytesMut;

// https://datatracker.ietf.org/doc/html/rfc1035#section-4.1
#[derive(Debug, PartialEq)]
pub struct Message {
    pub header: Header,
    pub question: Vec<Question>,
    pub answer: Vec<Answer>,
    pub authority: Vec<ResourceRecord>,
    pub additional_records: Vec<ResourceRecord>,
}

impl Message {
    pub fn parse(mp: &mut MessagePacket) -> Self {
        let header = Header::parse(mp);

        let question_count = header.question_count as usize;
        let mut question = Vec::with_capacity(question_count);
        for _ in 0..question_count {
            question.push(Question::parse(mp));
        }

        let answer_count = header.answer_count as usize;
        let mut answer = Vec::with_capacity(answer_count);
        for _ in 0..answer_count {
            answer.push(Answer::parse(mp));
        }

        let authority_count = header.nameserver_count as usize;
        let mut authority = Vec::with_capacity(authority_count);
        for _ in 0..authority_count {
            authority.push(ResourceRecord::parse(mp));
        }

        let additional_records_count = header.additional_records_count as usize;
        let mut additional_records = Vec::with_capacity(additional_records_count);
        for _ in 0..additional_records_count {
            additional_records.push(ResourceRecord::parse(mp));
        }

        Message {
            header,
            question,
            answer,
            authority,
            additional_records,
        }
    }

    pub fn write(&self, msg: BytesMut) -> BytesMut {
        let msg = self.header.write(msg);
        let msg = self.question.iter().fold(msg, |buff, q| q.write(buff));
        let msg = self.answer.iter().fold(msg, |buff, q| q.write(buff));
        let msg = self.authority.iter().fold(msg, |buff, q| q.write(buff));
        let msg = self
            .additional_records
            .iter()
            .fold(msg, |buff, q| q.write(buff));

        msg
    }
}

#[cfg(test)]
mod tests {
    use crate::dns::dname::DomainName;
    use crate::dns::header::ResponseCode::NoError;
    use crate::dns::header::{Header, ResponseCode};
    use crate::dns::message::Message;
    use crate::dns::question::Question;
    use crate::dns::{MessagePacket, QType};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn test_read_write() {
        let b = Bytes::from(
            &b"[\xa3\x01\0\0\x01\0\0\0\0\0\0\x04mail\x06google\x03com\0\0\x1c\0\x01"[..],
        );
        let k = Message {
            header: Header {
                id: 23459,
                question_response: 0,
                opcode: 0,
                authoritative_answer: 0,
                truncation: false,
                recursion_desired: true,
                recursion_available: false,
                z: 0,
                response_code: ResponseCode::NoError,
                question_count: 1,
                answer_count: 0,
                nameserver_count: 0,
                additional_records_count: 0,
            },
            question: vec![Question {
                domain_name: DomainName::parse_url("mail.google.com"),
                query_type: QType::AAAA,
                query_class: 1,
            }],
            answer: vec![],
            authority: vec![],
            additional_records: vec![],
        };
        let mut a = MessagePacket::from_bytes(b);
        assert_eq!(Message::parse(&mut a), k);

        let w = k.write(BytesMut::new());

        let mut mp = MessagePacket::from_bytes(w.freeze());
        assert_eq!(Message::parse(&mut mp), k);
    }
}
