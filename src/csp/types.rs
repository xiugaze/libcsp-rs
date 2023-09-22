use std::{collections::VecDeque, sync::{Mutex, Arc}, io::{self, Cursor}, fmt};
use byteorder::{ReadBytesExt, BigEndian, LittleEndian, WriteBytesExt};

use super::{
    qfifo::CspQfifo,
    interfaces::CspInterfaceState,
    CspId, 
};


const HEADER_SIZE: u16      = 6;
const PRIO_MASK: u8        = 0x03;
const PRIO_OFFSET: usize    = 46;
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
pub struct Packet {
    length: u16,
    id: CspId,
    header: [u8; 6],
    data: Vec<u8>,
}

impl Packet {
    /**
    * Constructs a packet from a buffer. Intended to be used to construct a packet from a received
    * buffer. 
    */
    pub fn from(length: usize, data: [u8; 256]) -> Self {
        let mut header: [u8; 8]= [0; 8];
        header[2..].copy_from_slice(&data[0..6]);

        let id = Packet::strip_id(header);
        Packet {
            // length = data size - header size
            length: length as u16 - HEADER_SIZE, 
            id,
            header: data[0..6].to_owned().try_into().unwrap(),
            data: data[6..length as usize].to_owned(),
        }
    }

    // pub fn new(data: Vec<u8>) -> Self {
    //
    // }

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

    pub fn set_id(&mut self, id: CspId) {
        self.id = id;
    }

    pub fn set_dport(&mut self, dport: u8) {
        self.id.dport = dport;
    }

    pub fn set_sport(&mut self, sport: u8) {
        self.id.sport = sport;
    }

    pub fn strip_id(data: [u8; 8]) -> CspId {
        // 48 bits (6 bytes of data) to build the ID
        // Network Byte order is Big-endian, so we need
        // be64toh (Big Endian 64 unsigned to host architecture)
        let id: u64 = Cursor::new(data).read_u64::<BigEndian>().unwrap() as u64;
        println!("id: {id:#018x}");
        CspId {
            priority: ((id >> PRIO_OFFSET) as u8 & PRIO_MASK),
            flags: ((id >> FLAGS_OFFSET) as u8 & FLAGS_MASK),
            source: ((id >> SRC_OFFSET) as u16 & SRC_MASK),
            destination: ((id >> DST_OFFSET) as u16 & DST_MASK),
            dport: ((id >> DPORT_OFFSET) as u8 & DPORT_MASK),
            sport: ((id >> SPORT_OFFSET) as u8 & SPORT_MASK),
        }
    }

    pub fn prepend_id(id: &CspId) -> Vec<u8> {
        let id: u64 = ((id.priority as u64) << PRIO_OFFSET) |
                          ((id.flags as u64) << FLAGS_OFFSET) |
                          ((id.source as u64) << SRC_OFFSET) | 
                          ((id.dport as u64) << DPORT_OFFSET) | 
                          ((id.sport as u64) << SPORT_OFFSET);
        let mut wtr = vec![];
        wtr.write_u64::<BigEndian>(id << 16).unwrap();
        wtr[..6].to_vec()
    }
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

impl fmt::Display for Packet {
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

#[derive(Debug)]
pub enum CspError {
    InvalidPort,
    OutOfPorts,
    NoPort,
    EmptyQfifo,
    InterfaceSendFailed,
    ClosedConnection,
}
