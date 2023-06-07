use std::sync::{Arc, Mutex};
use crate::csp::qfifo::{CspQfifo, self};

use super::{CspInterfaceState, NextHop};


pub struct LoopbackInterface {
    iface: Arc<Mutex<CspInterfaceState>>,
    state: Arc<Mutex<LoopbackState>>,
}
pub struct LoopbackState {
    qfifo: Arc<Mutex<CspQfifo>>,
}

impl LoopbackInterface {
    pub fn init(qfifo: &Arc<Mutex<CspQfifo>>) -> Self {
        let qfifo = Arc::clone(qfifo);
        let state = Arc::new(Mutex::new(LoopbackState::from(qfifo)));

        // TODO: Come up with a better way to initialize CspInterfaceState
        let iface = Arc::new(Mutex::new(CspInterfaceState::default()));
        LoopbackInterface { iface, state }
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
    fn nexthop(&self, packet: crate::csp::types::CspPacket) -> std::io::Result<usize> {
        let queue_mutex = &self.state.lock().unwrap().qfifo;
        let mut queue = queue_mutex.lock().unwrap();
        queue.push(
            Arc::new(Mutex::new(packet)),
            Arc::clone(&self.iface))

    }

    fn get_state(&self) -> Arc<Mutex<CspInterfaceState>> {
        Arc::clone(&self.iface)
    }
}
