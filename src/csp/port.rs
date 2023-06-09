
pub enum CspPortState {
    Closed,
    Open, 
    // Callback,
}
pub struct CspPort {
    state: CspPortState,
    socket: CspSocket,
}

pub struct CspSocket {
    rx_queue: VecDeque<CspPacket>,
}

// pub struct
