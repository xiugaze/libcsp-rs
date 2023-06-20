use std::{sync::{Mutex, Arc}, collections::VecDeque, io};

use super::{types::CspPacket, interfaces::{NextHop, CspInterfaceState}};

#[derive(Debug)]
pub struct QfifoElement {
    pub packet: CspPacket,
    pub interface: Arc<dyn NextHop+Send+Sync>,
}

#[derive(Default, Debug)]
pub struct CspQfifo {
    qfifo: VecDeque<QfifoElement>
}

impl CspQfifo {
    pub fn new() -> Self {
        CspQfifo {
            qfifo: VecDeque::new(),
        }
    }

    pub fn push(&mut self, packet: CspPacket, interface: Arc<dyn NextHop+Send+Sync>) -> io::Result<usize> {
        let qfifo_element = QfifoElement {
            packet,
            interface: Arc::clone(&interface),
        };
        self.qfifo.push_back(qfifo_element);
        return Ok(0)
    }

    pub fn pop(&mut self) -> Option<(CspPacket, Arc<dyn NextHop + Send + Sync>)> {
        // removes from queue, qfifio_element is only owner of Arcs
        match self.qfifo.pop_front() {
            Some(qfifo_element) => Some((qfifo_element.packet, qfifo_element.interface)),
            None => None
        }
    }   
}

