use std::{collections::VecDeque, sync::{Mutex, Arc}, io::{self, Cursor}, fmt};
use byteorder::{ReadBytesExt, BigEndian};

use super::{
    qfifo::CspQfifo,
    interfaces::CspInterfaceState,
    CspId,
};


const HEADER_SIZE: u16 = 6;
const PRIO_MASK: u16        = 6;
const PRIO_OFFSET: usize    = 6;
const DST_MASK: u16         = 0x3FFF;
const DST_OFFSET: usize     = 32;
const SRC_MASK: u16         = 0x3FFF;
const SRC_OFFSET: usize     = 18;
const DPORT_MASK: u8        = 0x3F;
const DPORT_OFFSET: usize   = 12;
const SPORT_MASK: u8        = 0x3F;
const SPORT_OFFSET: usize   = 6;
const FLAGS_MASK: u8        = 0x3F;
const FLAGS_OFFSET: usize   = 0;



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
        let mut header: [u8; 8]= [0; 8];
        header[2..].copy_from_slice(&data[0..6]);

        let id = CspPacket::strip_id(header);
        CspPacket {
            // length = data size - header size
            length: length as u16 - HEADER_SIZE, 
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

    fn strip_id(data: [u8; 8]) -> CspId {
        // 48 bits (6 bytes of data) to build the ID
        // Network Byte order is Big-endian, so we need
        // be64toh (Big Endian 64 unsigned to host architecture)
        let id = Cursor::new(data).read_u64::<BigEndian>().unwrap();
        println!("{id:#016X}");
        CspId {
            priority: ((id >> 46) & 0x3) as u8,
            destination: ((id >> 32) & 0x3FFF) as u16,

        
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
