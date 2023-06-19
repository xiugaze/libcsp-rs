use std::sync::{Arc, Mutex};
use crate::csp::{qfifo::{CspQfifo, self}, types::CspPacket};

use super::{CspInterfaceState, NextHop};


pub struct LoopbackInterface {
    iface: Arc<Mutex<CspInterfaceState>>,
    state: Arc<LoopbackState>,
    index: usize,
    // callback: Box<dyn Fn(CspPacket, usize) + Send + Sync>
}
pub struct LoopbackState {
    qfifo: Arc<Mutex<CspQfifo>>,
}

impl LoopbackInterface {
    pub fn init(qfifo: &Arc<Mutex<CspQfifo>>,  index: usize) -> Self {
        let qfifo = Arc::clone(qfifo);
        let state = Arc::new(LoopbackState::from(qfifo));

        // TODO: Come up with a better way to initialize CspInterfaceState
        let iface = Arc::new(Mutex::new(CspInterfaceState::default()));
        LoopbackInterface { iface, state, index }
    }
}

impl LoopbackState {
    pub fn from(qfifo: Arc<Mutex<CspQfifo>>) -> Self {
        LoopbackState { qfifo }
    }
}

impl NextHop for LoopbackInterface {
    /**
        Loopback TX: Adds packet back to Receive queue
    */
    fn nexthop(self: Arc<Self>, packet: crate::csp::types::CspPacket) -> std::io::Result<usize> {
        let mut queue = self.state.qfifo.lock().unwrap();
        let iface = Arc::clone(&self);
        queue.push(packet, iface)
    }

    /** 
        Returns the interface metadata struct
    */
    fn get_state(&self) -> Arc<Mutex<CspInterfaceState>> {
        Arc::clone(&self.iface)
    }
}
unsafe impl Send for LoopbackInterface {}

