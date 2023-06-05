use std::{collections::VecDeque, sync::{Mutex, Arc}, io};

use super::{CspQueue, CspId};

#[repr(C)]
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
            header: data[0..5].try_into().unwrap(), 
            data: data[6..length as usize].to_owned(),
        }
    }
    pub fn len(&self) -> u16 {
        self.length
    }
    pub fn make_header(self) -> Vec<u8> {
        let mut header = self.header.to_vec();
        let mut data = self.data;
        header.append(&mut data);
        header
    }
    pub fn id(&self) -> &CspId {
        &self.id
    }


}

// pub struct Outer {
//     inner: Arc<Mutex<Inner>>,
// }
//
// pub struct Inner {
//     state: i32,
// }
// pub struct NextHop {
//     iface: Arc<Mutex<Box<dyn InterfaceHandle>>>
// }
// impl NextHop {
//     pub fn from(handle: Box<dyn InterfaceHandle>) -> NextHop {
//         NextHop {
//             iface: Arc::new(Mutex::new(handle)),
//         }
//     }
// }


pub trait NextHop {
    fn nexthop(&self, via: u16, packet: CspPacket, from_me: bool) -> io::Result<usize>;
    // TODO: Move out to UDP type only
    // fn start_thread(&mut self, iface: &mut Arc<Mutex<Box<dyn NextHop>>>, qfifo: CspQueue)
}


