pub ResourceRecordType{
    UNKNOW(u16),
    A, //1
    PTR, //12 (0x0C)
    TXT, //16 (0x10)
    AAAA, //28 (0x1C)
    SRV, //33 (0x21)
    NSEC, //47 (0x2f)
    ANY, //0xFF
}

impl ResourceRecordType {
    pub fn from_num(num: u16) -> Self{
        match num {
            0x1 => Self::A,
            0x0C => Self::PTR,
            0x10 => Self::TXT,
            0x1C => Self::AAAA,
            0x21 => Self::SRV,
            0x2f => Self::NSEC,
            0xff => Self::ANY,
            _ => Self::UNKNOW(num),
        }
    }

    pub fn to_num(*self) -> u16 {
        match self{
            Self::A => 0x1,
            Self::PTR => 0x0C,
            Self::TXT => 0x10,
            Self::AAAA => 0x1C,
            Self::SRV => 0x21,
            Self::NSEC => 0x2f,
            Self::ANY => 0xff,
            Self::UNKNOW(x) => x,
        }
    }
}
