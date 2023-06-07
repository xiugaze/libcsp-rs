use std::{sync::{Mutex, Arc}, collections::VecDeque, io};

use super::{types::CspPacket, interfaces::{NextHop, CspInterfaceState}};

pub struct QfifoElement {
    pub packet: Arc<Mutex<CspPacket>>,
    pub interface: Arc<Mutex<CspInterfaceState>>,
}

#[derive(Default)]
pub struct CspQfifo {
    qfifo: VecDeque<QfifoElement>
}

impl CspQfifo {
    pub fn new() -> Self {
        CspQfifo {
            qfifo: VecDeque::new(),
        }
    }

    pub fn push(&mut self, packet: Arc<Mutex<CspPacket>>, interface: Arc<Mutex<CspInterfaceState>>) -> io::Result<usize> {
        let qfifo_element = QfifoElement {
            packet: Arc::clone(&packet),
            interface: Arc::clone(&interface),
        };
        self.qfifo.push_back(qfifo_element);
        return Ok(0)
    }

    pub fn pop(&mut self) -> (Arc<Mutex<CspPacket>>, Arc<Mutex<CspInterfaceState>>) {
        let qfifo_element = self.qfifo.pop_front().unwrap();
        let packet = Arc::clone(&qfifo_element.packet);
        let interface = Arc::clone(&qfifo_element.interface);
        (packet, interface)
    }
}

