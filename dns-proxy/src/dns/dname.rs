use crate::dns::MessageBytes;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::str;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct DomainName {
    pub labels: Vec<String>,
}

impl DomainName {
    pub fn empty() -> Self {
        DomainName { labels: vec![] }
    }
    pub fn parse_url(url: &str) -> Self {
        let mut v = vec![];
        for label in url.split(".") {
            if label.len() == 0 {
                continue;
            }
            v.push(String::from(label));
        }

        DomainName { labels: v }
    }
    fn internal_parse(
        mut labels: Vec<String>,
        original: &mut Bytes,
        buffer: &mut Bytes,
    ) -> Vec<String> {
        loop {
            let len = buffer.get_u8() as usize;
            if len == 0 {
                break;
            }

            if len >> 6 & 0b11 == 3 {
                let offset = (len & 0b111111) << 8 | buffer.get_u8() as usize;
                let mut new_buffer = original.clone();
                new_buffer.advance(offset);
                labels = DomainName::internal_parse(labels, original, &mut new_buffer);
                break;
            } else {
                let label_buff = buffer.copy_to_bytes(len);
                let label = str::from_utf8(&label_buff[..]).expect("Invalid UTF-8");
                labels.push(String::from(label));
            }
        }
        labels
    }
    pub fn parse(mp: &mut MessageBytes) -> Self {
        let labels = DomainName::internal_parse(vec![], &mut mp.original, &mut mp.buffer);
        DomainName { labels }
    }

    pub fn write(&self, mut msg: BytesMut) -> BytesMut {
        for label in self.labels.iter() {
            msg.put_u8(label.len() as u8);
            msg.put_slice(label.as_bytes())
        }
        msg.put_u8(0);
        msg
    }
}
// tests available at question.rs and record.rs
