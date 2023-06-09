use std::sync::Mutex;
use std::sync::Arc;
use std::io;

use crate::csp::CspPacket;

//pub mod if_udp;
pub mod if_loopback;

// Common metadata for all interfaces
// Interfaces are a struct that holds a CspInterface and 
// implements NextHop

/**
    Common metad
*/
#[derive(Default)]
pub struct CspInterfaceState {
    address: u16,           // address on this submet
    netmask: u16,           // subnet mask
    name: String,           // name of interface
    tx: u32,                // successfully transmitted packets
    rx: u32,                // successfully received packets
    tx_bytes: u32,          // successfully transmitted bytes
    rx_bytes: u32,          // successfully received bytes
}
impl CspInterfaceState {
    pub fn increment_tx(&mut self) {
        self.tx += 1;
    }
    pub fn increment_rx(&mut self) {
        self.rx += 1;
    }
    pub fn address(&self) -> u16 {
        self.address
    }
}


pub trait NextHop {
    /**
        Transmits the packet on the target interface
    */
    fn nexthop(&self, packet: CspPacket) -> io::Result<usize>;

    /**
        Returns an `Arc` to the state struct of the target interface
    */
    fn get_state(&self) -> Arc<Mutex<CspInterfaceState>>;
}
