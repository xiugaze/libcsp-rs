#[derive(Clone)]
pub struct CspPacket {
    pub id: CspId,
    pub data: Vec<u8>,
}

impl CspPacket {
    pub fn new() -> CspPacket {
        CspPacket {
            id: CspId::new(),
            data: Vec::new(),
        }
    }
    pub fn push(&mut self, value: u8) {
        self.data.push(value);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CspId {
    pub pri: u8,
    pub flags: u8,
    pub src: u8,
    pub dst: u8,
    pub dport: u8,
    pub sport: u8,
}

impl CspId {
    pub fn new() -> CspId {
        CspId {
            pri: 0,
            flags: 0,
            src: 0,
            dst: 0,
            dport: 0,
            sport: 0,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let mut packet = CspPacket::new();
        packet.push(5);
        assert_eq!(packet.data[0], 5);
    }

}

