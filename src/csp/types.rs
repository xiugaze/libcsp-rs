use std::{collections::VecDeque, sync::{Mutex, Arc}};

use super::CspQueue;

#[repr(C)]
pub struct CspPacket {
    length: u16,
    // id: CspID,
    // next: &CspPacket
    header: [u8; 6],
    data: Vec<u8>,
}

impl CspPacket {
    pub fn new(length: usize, data: [u8; 256]) -> Self {
        CspPacket {
            length: length as u16, 
            header: data[0..5].try_into().unwrap(), 
            data: data[6..length as usize].to_owned(),
        }
    }
}

pub trait CspInterface {
    fn nexthop(&self);
    // TODO: Move out to UDP type only
    fn start_thread(&mut self, iface: &mut Arc<Mutex<Box<dyn CspInterface>>>, qfifo: CspQueue)
}


pub struct Outer {
    inner: Arc<Mutex<Inner>>,
}

pub struct Inner {
    state: i32,
}
