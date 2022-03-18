use crate::dns::dname::DomainName;
use crate::dns::{MessagePacket, QType};
use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Debug, PartialEq)]
pub struct ResourceRecord {
    pub domain_name: DomainName, // QNAME - needs to be parsed as labels
    pub resource_type: QType,    // TYPE - 16 bits
    pub resource_class: u16,     // CLASS - 16 bits. Only IN will be supported.
    pub ttl: u32,                // TTL - 32 bits. Time to live in seconds
    pub data_length: u16,        // RDLENGTH - 16 bits
    pub resource_data: Bytes,    // RDATA
}

impl ResourceRecord {
    pub fn parse(mp: &mut MessagePacket) -> Self {
        let domain_name = DomainName::parse(mp);
        let resource_type = QType::from_u16(mp.buffer.get_u16());
        let resource_class = mp.buffer.get_u16();
        let ttl = mp.buffer.get_u32();
        println!("ttl: {:?}", ttl);
        let data_length = mp.buffer.get_u16();
        println!("data_length: {:?}", data_length);

        // println!("{:?}", DomainName::parse(&mut mp.clone()));
        let resource_data = mp.buffer.copy_to_bytes(data_length as usize);

        ResourceRecord {
            domain_name,
            resource_type,
            resource_class,
            ttl,
            data_length,
            resource_data,
        }
    }

    pub fn write(&self, mut msg: BytesMut) -> BytesMut {
        let mut msg = self.domain_name.write(msg);

        msg.put_u16(self.resource_type as u16);
        msg.put_u16(self.resource_class);
        msg.put_u32(self.ttl);
        msg.put_u16(self.resource_data.len() as u16);
        msg.put_slice(self.resource_data.as_ref());

        msg
    }
}

#[cfg(test)]
mod tests {
    use crate::dns::dname::DomainName;
    use crate::dns::record::ResourceRecord;
    use crate::dns::{MessagePacket, QType};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn test_record() {
        let b = Bytes::from(&b"\x02ns\x04alfa\x03net\x00\x00\x02\x00\x01\x00\x00\x0e\x10\x00\x02\xc0\x00\xc0\x03\x00\x05\x00\x01\x00\x00\x04\xb0\x00\x07\x04ip00\xc0\x03\xc0%\x00\x1c\x00\x01\x00\x00\x0e\x10\x00\x10\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x02mx\xc0\x03\x00\x0f\x00\x01\x00\x00\x0e\x10\x00\x07\x00\n\x03::1\x00\x03txt\xc0\x03\x00\x10\x00\x01\x00\x00\x0e\x10\x00\t\x08Test 001"[..]);
        let mut a = MessagePacket::from_bytes(b);
        let hs = ResourceRecord::parse(&mut a);
        assert_eq!(hs.domain_name.labels, vec!["ns", "alfa", "net"]);
        assert_eq!(hs.resource_type, QType::NS);
        assert_eq!(hs.resource_class, 1);
        assert_eq!(hs.ttl, 3600);
        assert_eq!(hs.resource_data, Bytes::from(&b"\xc0\0"[..]));

        let hs = ResourceRecord::parse(&mut a);
        assert_eq!(hs.domain_name.labels, vec!["alfa", "net"]);
        assert_eq!(hs.resource_type, QType::CNAME);
        assert_eq!(hs.resource_class, 1);
        assert_eq!(hs.ttl, 1200);
        assert_eq!(hs.resource_data, Bytes::from(&b"\x04ip00\xc0\x03"[..]));

        let hs = ResourceRecord::parse(&mut a);
        assert_eq!(hs.domain_name.labels, vec!["ip00", "alfa", "net"]);
        assert_eq!(hs.resource_type, QType::AAAA);
        assert_eq!(hs.resource_class, 1);
        assert_eq!(hs.ttl, 3600);
        assert_eq!(
            hs.resource_data,
            Bytes::from(&b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01"[..])
        );

        let hs = ResourceRecord::parse(&mut a);
        assert_eq!(hs.domain_name.labels, vec!["mx", "alfa", "net"]);
        assert_eq!(hs.resource_type, QType::MX);
        assert_eq!(hs.resource_class, 1);
        assert_eq!(hs.ttl, 3600);
        assert_eq!(hs.resource_data, Bytes::from(&b"\0\n\x03::1\0"[..]));

        let hs = ResourceRecord::parse(&mut a);
        assert_eq!(hs.domain_name.labels, vec!["txt", "alfa", "net"]);
        assert_eq!(hs.resource_type, QType::TXT);
        assert_eq!(hs.resource_class, 1);
        assert_eq!(hs.ttl, 3600);
        assert_eq!(hs.resource_data, Bytes::from(&b"\x08Test 001"[..]));
    }

    #[test]
    fn test_write() {
        // check only if the bits are right
        let b = Bytes::from(&b"aaaaaaaaa"[..]);
        let a = ResourceRecord {
            domain_name: DomainName::parse_url("alfa.example.com"),
            resource_type: QType::AAAA,
            resource_class: 1,
            ttl: 3600,
            data_length: b.len() as u16,
            resource_data: b,
        };

        let w = a.write(BytesMut::new());

        let mut mp = MessagePacket::from_bytes(w.freeze());
        assert_eq!(ResourceRecord::parse(&mut mp), a);

        let b = Bytes::from(&b"xxxxxxxxxx"[..]);
        let a = ResourceRecord {
            domain_name: DomainName::parse_url("mail.x.example.com"),
            resource_type: QType::TXT,
            resource_class: 1,
            ttl: 3412,
            data_length: b.len() as u16,
            resource_data: b,
        };

        let w = a.write(BytesMut::new());

        let mut mp = MessagePacket::from_bytes(w.freeze());
        assert_eq!(ResourceRecord::parse(&mut mp), a);
    }
}
