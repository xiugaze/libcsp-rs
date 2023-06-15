use std::collections::VecDeque;

use super::types::CspPacket;


pub enum CspPortState {
    Closed,
    Open, 
    // Callback,
}
pub struct CspPort {
    pub state: CspPortState,
    pub socket: CspSocket,
}

impl CspPort {
    pub fn get_socket(&mut self) -> &mut CspSocket {
        &mut self.socket
    }
}

pub struct CspSocket {
    rx_queue: VecDeque<CspPacket>,
    conn_less: bool,
}
impl CspSocket {
    pub fn new(conn_less: bool) -> CspSocket {
        CspSocket { rx_queue: VecDeque::new(), conn_less }
    }
    pub fn conn_less(&self) -> bool {
        self.conn_less
    }

    pub fn push(&mut self, packet: CspPacket) {
        self.rx_queue.push_back(packet);
    }
    pub fn pop(&mut self) -> Option<CspPacket> {
        self.rx_queue.pop_front()
    }

    // TODO: This is just a print function, should probably do something more useful
    pub fn flush_rx_queue(&mut self) {
        for packet in self.rx_queue.drain(0..) {
            print!("{packet}");
        }
    }

}

// pub struct
