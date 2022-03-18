use crate::dns::dname::DomainName;
use crate::dns::{MessageBytes, QType};
use bytes::{Buf, BufMut, BytesMut};

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct Question {
    pub domain_name: DomainName, // QNAME - needs to be parsed as labels
    pub query_type: QType,       // QTYPE - 16 bits
    pub query_class: u16,        // QCLASS - 16 bits. Only IN will be supported.
}

impl Question {
    pub fn parse(mp: &mut MessageBytes) -> Self {
        Question {
            domain_name: DomainName::parse(mp),
            query_type: QType::from_u16(mp.buffer.get_u16()),
            query_class: mp.buffer.get_u16(),
        }
    }

    pub fn write(&self, mut msg: BytesMut) -> BytesMut {
        let mut msg = self.domain_name.write(msg);

        msg.put_u16(self.query_type as u16);
        msg.put_u16(self.query_class);

        msg
    }
}

#[cfg(test)]
mod tests {
    use crate::dns::dname::DomainName;
    use crate::dns::question::Question;
    use crate::dns::{MessageBytes, QType};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn test_question_section_1domain_without_compression() {
        let b = Bytes::from(&b"\x03abc\x03com\x00\x00\x01\x00\x01"[..]);
        let mut a = MessageBytes::from_bytes(b);
        let hs = Question::parse(&mut a);
        assert_eq!(hs.domain_name.labels, vec!["abc", "com"]);
        assert_eq!(hs.query_type, QType::A);
        assert_eq!(hs.query_class, 1);

        let b = Bytes::from(&b"\x03abc\x03com\x00\x00\x0f\x00\x01"[..]);
        let mut a = MessageBytes::from_bytes(b);
        let hs = Question::parse(&mut a);
        assert_eq!(hs.domain_name.labels, vec!["abc", "com"]);
        assert_eq!(hs.query_type, QType::MX);
        assert_eq!(hs.query_class, 1);

        let b = Bytes::from(&b"\x04alfa\x04beta\x03abc\x03com\x00\x00\x1c\x00\x01"[..]);
        let mut a = MessageBytes::from_bytes(b);
        let hs = Question::parse(&mut a);
        assert_eq!(hs.domain_name.labels, vec!["alfa", "beta", "abc", "com"]);
        assert_eq!(hs.query_type, QType::AAAA);
        assert_eq!(hs.query_class, 1);
    }

    #[test]
    fn test_question_section_domains_with_compression() {
        let b = Bytes::from(&b"\x04alfa\x04beta\x03abc\x03com\x00\x00\x1c\x00\x01\x04mail\xc0\n\x00\x0f\x00\x01\xc0\n\x00\x02\x00\x01"[..]);
        let mut a = MessageBytes::from_bytes(b);
        let hs = Question::parse(&mut a);
        assert_eq!(hs.domain_name.labels, vec!["alfa", "beta", "abc", "com"]);
        assert_eq!(hs.query_type, QType::AAAA);
        assert_eq!(hs.query_class, 1);

        let hs = Question::parse(&mut a);
        assert_eq!(hs.domain_name.labels, vec!["mail", "abc", "com"]);
        assert_eq!(hs.query_type, QType::MX);
        assert_eq!(hs.query_class, 1);

        let hs = Question::parse(&mut a);
        assert_eq!(hs.domain_name.labels, vec!["abc", "com"]);
        assert_eq!(hs.query_type, QType::NS);
        assert_eq!(hs.query_class, 1);

        let b = Bytes::from(&b"\x04alfa\x03net\x00\x00\x1c\x00\x01\x01x\xc0\x00\x00\x0f\x00\x01\xc0\x00\x00\x02\x00\x01\x04mail\xc0\x0e\x00\x0f\x00\x01\x04mail\xc0\x00\x00\x0f\x00\x01"[..]);
        let mut a = MessageBytes::from_bytes(b);

        let hs = Question::parse(&mut a);
        assert_eq!(hs.domain_name.labels, vec!["alfa", "net"]);
        assert_eq!(hs.query_type, QType::AAAA);
        assert_eq!(hs.query_class, 1);

        let hs = Question::parse(&mut a);
        assert_eq!(hs.domain_name.labels, vec!["x", "alfa", "net"]);
        assert_eq!(hs.query_type, QType::MX);
        assert_eq!(hs.query_class, 1);

        let hs = Question::parse(&mut a);
        assert_eq!(hs.domain_name.labels, vec!["alfa", "net"]);
        assert_eq!(hs.query_type, QType::NS);
        assert_eq!(hs.query_class, 1);

        let hs = Question::parse(&mut a);
        assert_eq!(hs.domain_name.labels, vec!["mail", "x", "alfa", "net"]);
        assert_eq!(hs.query_type, QType::MX);
        assert_eq!(hs.query_class, 1);

        let hs = Question::parse(&mut a);
        assert_eq!(hs.domain_name.labels, vec!["mail", "alfa", "net"]);
        assert_eq!(hs.query_type, QType::MX);
        assert_eq!(hs.query_class, 1);
    }

    #[test]
    fn test_write() {
        // check only if the bits are right
        let a = Question {
            domain_name: DomainName::parse_url("alfa.example.com"),
            query_type: QType::AAAA,
            query_class: 1,
        };

        let w = a.write(BytesMut::new());

        let mut mp = MessageBytes::from_bytes(w.freeze());
        assert_eq!(Question::parse(&mut mp), a);

        let a = Question {
            domain_name: DomainName::parse_url("mail.x.example.com"),
            query_type: QType::TXT,
            query_class: 1,
        };

        let w = a.write(BytesMut::new());

        let mut mp = MessageBytes::from_bytes(w.freeze());
        assert_eq!(Question::parse(&mut mp), a);
    }
}
