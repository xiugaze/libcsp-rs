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

pub struct CspSocket {
    rx_queue: VecDeque<CspPacket>,
    conn_less: bool,
}
impl CspSocket {
    pub fn conn_less(&self) -> bool {
        self.conn_less
    }

    pub fn enqueue(&mut self, packet: CspPacket) {
        self.rx_queue.push_back(packet);
    }
}

// pub struct
