use crate::dns::MessageBytes;
use bytes::{Buf, BufMut, BytesMut};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
    // 6 - 15 Reserved for future use
}

impl Default for ResponseCode {
    fn default() -> Self {
        ResponseCode::NotImplemented
    }
}

impl ResponseCode {
    fn from_u16(var: u16) -> Self {
        match var {
            0 => ResponseCode::NoError,
            1 => ResponseCode::FormatError,
            2 => ResponseCode::ServerFailure,
            3 => ResponseCode::NameError,
            5 => ResponseCode::Refused,
            _ => ResponseCode::NotImplemented,
        }
    }
}

// https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1
#[derive(Default, Debug, PartialEq)]
pub struct Header {
    pub id: u16,                       // ID - 16 bits
    pub question_response: u8,         // QR - 1 bit (0 = question, 1 = answer)
    pub opcode: u8,                    // OPCODE - 4 bits
    pub authoritative_answer: u8,      // AA - 1 bit
    pub truncation: bool,              // TC - 1 bit
    pub recursion_desired: bool,       // RD - 1 bit
    pub recursion_available: bool,     // RA - 1 bit
    pub z: u8,                         // Z - 3 bits as zero
    pub response_code: ResponseCode,   // RCODE - 4 bits
    pub question_count: u16,           // QDCOUNT - 16 bits
    pub answer_count: u16,             // ANCOUNT - 16 bits
    pub nameserver_count: u16,         // NSCOUNT - 16 bits
    pub additional_records_count: u16, // ARCOUNT - 16 bits
}

impl Header {
    pub fn parse(mp: &mut MessageBytes) -> Self {
        let buffer = &mut mp.buffer;
        let id = buffer.get_u16();
        let flags = buffer.get_u16();
        // println!("{:016b}", flags);
        Header {
            id,
            question_response: (flags >> 15 & 0b1) as u8,
            opcode: (flags >> 11 & 0b1111) as u8,
            authoritative_answer: (flags >> 10 & 0b1) as u8,
            truncation: (flags >> 9 & 0b1) == 1,
            recursion_desired: (flags >> 8 & 0b1) == 1,
            recursion_available: (flags >> 7 & 0b1) == 1,
            z: (flags >> 4 & 0b111) as u8, // not used
            response_code: ResponseCode::from_u16(flags & 0b1111),
            question_count: buffer.get_u16(),
            answer_count: buffer.get_u16(),
            nameserver_count: buffer.get_u16(),
            additional_records_count: buffer.get_u16(),
        }
    }

    pub fn write(&self, mut msg: BytesMut) -> BytesMut {
        let mut flags = 0 as u16;
        flags |= (self.question_response as u16) << 15;
        flags |= (self.opcode as u16) << 11;
        flags |= (self.authoritative_answer as u16) << 10;
        flags |= if self.truncation { 1 } else { 0 } << 9;
        flags |= if self.recursion_desired { 1 } else { 0 } << 8;
        flags |= if self.recursion_available { 1 } else { 0 } << 7;
        flags |= (self.response_code as u16) & 0b1111;

        msg.put_u16(self.id);
        msg.put_u16(flags);
        msg.put_u16(self.question_count);
        msg.put_u16(self.answer_count);
        msg.put_u16(self.nameserver_count);
        msg.put_u16(self.additional_records_count);

        msg
    }
}

#[cfg(test)]
mod tests {
    use crate::dns::header::{Header, ResponseCode};
    use crate::dns::MessageBytes;
    use bytes::{Bytes, BytesMut};

    #[test]
    fn test_header_read() {
        let b = Bytes::from(&b"09\tE\x00\x00\x00\x00\x00\x00\x00\x00"[..]);
        let mut a = MessageBytes::from_bytes(b);
        let hs = Header::parse(&mut a);
        assert_eq!(hs.id, 12345);
        assert_eq!(hs.question_response, 0);
        assert_eq!(hs.opcode, 1);
        assert_eq!(hs.authoritative_answer, 0);
        assert_eq!(hs.truncation, false);
        assert_eq!(hs.recursion_desired, true);
        assert_eq!(hs.response_code, ResponseCode::Refused);

        let b = Bytes::from(&b"\xd41\xfd\x00\x00\x00\x00\x00\x00\x00\x00\x00"[..]);
        let mut a = MessageBytes::from_bytes(b);
        let hs = Header::parse(&mut a);
        assert_eq!(hs.id, 54321);
        assert_eq!(hs.question_response, 1);
        assert_eq!(hs.opcode, 15);
        assert_eq!(hs.authoritative_answer, 1);
        assert_eq!(hs.truncation, false);
        assert_eq!(hs.recursion_desired, true);
        assert_eq!(hs.response_code, ResponseCode::NoError);

        let b = Bytes::from(&b"\x00\x00\x02\x03\x00\x03\x00\x17\x00\x05\x00\x07"[..]);
        let mut a = MessageBytes::from_bytes(b);
        let hs = Header::parse(&mut a);
        assert_eq!(hs.id, 0);
        assert_eq!(hs.question_response, 0);
        assert_eq!(hs.opcode, 0);
        assert_eq!(hs.authoritative_answer, 0);
        assert_eq!(hs.truncation, true);
        assert_eq!(hs.recursion_desired, false);
        assert_eq!(hs.response_code, ResponseCode::NameError);
        assert_eq!(hs.question_count, 3);
        assert_eq!(hs.answer_count, 23);
        assert_eq!(hs.nameserver_count, 5);
        assert_eq!(hs.additional_records_count, 7);
    }

    #[test]
    fn test_header_write() {
        // check only if the bits are right
        let a = Header {
            id: 123,
            question_response: 0,
            opcode: 3,
            authoritative_answer: 1,
            truncation: true,
            recursion_desired: false,
            recursion_available: true,
            z: 0,
            response_code: ResponseCode::Refused,
            question_count: 3,
            answer_count: 9,
            nameserver_count: 4,
            additional_records_count: 6,
        };

        let w = a.write(BytesMut::new());

        let mut mp = MessageBytes::from_bytes(w.freeze());
        assert_eq!(Header::parse(&mut mp), a);

        let a = Header {
            id: 432,
            question_response: 1,
            opcode: 2,
            authoritative_answer: 0,
            truncation: false,
            recursion_desired: true,
            recursion_available: false,
            z: 0,
            response_code: ResponseCode::ServerFailure,
            question_count: 1,
            answer_count: 3,
            nameserver_count: 6,
            additional_records_count: 3,
        };

        let w = a.write(BytesMut::new());

        let mut mp = MessageBytes::from_bytes(w.freeze());
        assert_eq!(Header::parse(&mut mp), a);
    }
}
