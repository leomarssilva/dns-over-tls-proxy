use bytes::Bytes;

pub mod dname;
pub mod header;
pub mod message;
pub mod question;
pub mod record;

#[derive(Debug, Clone)]
pub struct MessageBytes {
    original: Bytes,
    buffer: Bytes,
}

impl MessageBytes {
    pub fn from_bytes(b: Bytes) -> Self {
        MessageBytes {
            original: b.clone(),
            buffer: b.clone(),
        }
    }
}
#[derive(PartialEq, Debug, Copy, Clone, Hash, Eq)]
pub enum QType {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    NULL = 10,
    PTR = 12,
    HINFO = 13,
    MX = 15,
    TXT = 16,
    RP = 17,
    AFSDB = 18,
    SIG = 24,
    KEY = 25,
    AAAA = 28,
    LOC = 29,
    SRV = 33,
    NAPTR = 35,
    KX = 36,
    CERT = 37,
    A6 = 38,
    DNAME = 39,
    OPT = 41,
    APL = 42,
    DS = 43,
    SSHFP = 44,
    IPSECKEY = 45,
    RRSIG = 46,
    NSEC = 47,
    DNSKEY = 48,
    DHCID = 49,
    NSEC3 = 50,
    NSEC3PARAM = 51,
    TLSA = 52,
    HIP53 = 53,
    HIP55 = 55,
    CDS = 59,
    CDNSKEY = 60,
    OPENPGPKEY = 61,
    CSYNC = 62,
    ZONEMD = 63,
    SVCB = 64,
    HTTPS = 65,
    SPF = 99,
    EUI48 = 108,
    EUI64 = 109,
    TKEY = 249,
    TSIG = 250,
    IXFR = 251,
    AXFR = 252,
    ANY = 255,
    URI = 256,
    CAA = 257,
    TA = 32768,
    DLV = 32769,
}

impl QType {
    fn from_u16(var: u16) -> Self {
        match var {
            1 => QType::A,
            2 => QType::NS,
            5 => QType::CNAME,
            6 => QType::SOA,
            10 => QType::NULL,
            12 => QType::PTR,
            13 => QType::HINFO,
            15 => QType::MX,
            16 => QType::TXT,
            17 => QType::RP,
            18 => QType::AFSDB,
            24 => QType::SIG,
            25 => QType::KEY,
            28 => QType::AAAA,
            29 => QType::LOC,
            33 => QType::SRV,
            35 => QType::NAPTR,
            36 => QType::KX,
            37 => QType::CERT,
            38 => QType::A6,
            39 => QType::DNAME,
            41 => QType::OPT,
            42 => QType::APL,
            43 => QType::DS,
            44 => QType::SSHFP,
            45 => QType::IPSECKEY,
            46 => QType::RRSIG,
            47 => QType::NSEC,
            48 => QType::DNSKEY,
            49 => QType::DHCID,
            50 => QType::NSEC3,
            51 => QType::NSEC3PARAM,
            52 => QType::TLSA,
            53 => QType::HIP53,
            55 => QType::HIP55,
            59 => QType::CDS,
            60 => QType::CDNSKEY,
            61 => QType::OPENPGPKEY,
            62 => QType::CSYNC,
            63 => QType::ZONEMD,
            64 => QType::SVCB,
            65 => QType::HTTPS,
            99 => QType::SPF,
            108 => QType::EUI48,
            109 => QType::EUI64,
            249 => QType::TKEY,
            250 => QType::TSIG,
            251 => QType::IXFR,
            252 => QType::AXFR,
            255 => QType::ANY,
            256 => QType::URI,
            257 => QType::CAA,
            32768 => QType::TA,
            32769 => QType::DLV,
            _ => QType::NULL,
        }
    }
}
