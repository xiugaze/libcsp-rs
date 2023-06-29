use std::{collections::VecDeque, sync::{Mutex, Arc}, io::{self, Cursor}, fmt};
use byteorder::{ReadBytesExt, BigEndian};

use super::{
    qfifo::CspQfifo,
    interfaces::CspInterfaceState,
    CspId,
};


#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CspPacket {
    length: u16,
    id: CspId,
    header: [u8; 6],
    data: Vec<u8>,
}


impl CspPacket {
    pub fn new(length: usize, data: [u8; 256], id: CspId) -> Self {
        CspPacket {
            length: length as u16, 
            id,
            header: data[0..6].to_owned().try_into().unwrap(),
            data: data[6..length as usize].to_owned(),
        }
    }

    pub fn len(&self) -> u16 {
        self.length
    }
    pub fn make_buffer(self) -> Vec<u8> {
        let mut header = self.header.to_vec();
        let mut data = self.data;
        header.append(&mut data);
        header
    }
    pub fn id(&self) -> &CspId {
        &self.id
    }

    fn strip_id(id: [u8; 8]) -> CspId {
        // network order is big endian
        let id: u64 = Cursor::new(id).read_u64::<BigEndian>();
        CspId {
            priority: todo!(),
            flags: todo!(),
            source: todo!(),
            destination: todo!(),
            dport: todo!(),
            sport: todo!(),
        }
    

    }
}

impl fmt::Display for CspPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes = &self.data[0..5];
        let mut data_preview = String::new();
        for byte in bytes {
            data_preview.push_str(&format!("{:02X} ", byte))
        }

        write!(f, "Packet {{\n\tSource: {},\n\tDestination: {},\n\tHeader{:?}\n\tData: {data_preview}...\n}}", 
        self.id.source, 
        self.id.destination,
        self.header)
    }
}


pub type CspResult<T> = Result<T, CspError>; 
pub enum CspError {
    OutOfPorts,
    EmptyQfifo
}
