use std::{sync::{Mutex, Arc}, collections::VecDeque, io};

use super::{types::CspPacket, interfaces::{NextHop, CspInterfaceState}};

#[derive(Debug)]
pub struct QfifoElement {
    pub packet: CspPacket,
    pub interface: Arc<dyn NextHop>,
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

    pub fn push(&mut self, packet: CspPacket, interface: Arc<dyn NextHop>) -> io::Result<usize> {
        let qfifo_element = QfifoElement {
            packet,
            interface: Arc::clone(&interface),
        };
        self.qfifo.push_back(qfifo_element);
        return Ok(0)
    }

    pub fn pop(&mut self) -> (CspPacket, Arc<dyn NextHop>) {
        // removes from queue, qfifio_element is only owner of Arcs
        let qfifo_element = self.qfifo.pop_front().unwrap();
        
        let packet = qfifo_element.packet;          // moves out of
        let interface = qfifo_element.interface;    // moves out of
        (packet, interface)
    }   // qfifo_element dropped here
}

