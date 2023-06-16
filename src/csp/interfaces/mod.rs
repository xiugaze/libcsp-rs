use std::sync::Mutex;
use std::sync::Arc;
use std::io;

use crate::csp::CspPacket;

pub mod if_udp;
pub mod if_loopback;
pub mod if_drain;

// Common metadata for all interfaces
// Interfaces are a struct that holds a CspInterface and 
// implements NextHop

/**
    Common metadata for each interface
*/
#[derive(Default, Debug)]
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

    pub fn from_name(name: &str) -> Self {
        CspInterfaceState {
            name: String::from(name), 
            ..Default::default()
        }
    }
}

pub trait NextHop {
    /**
        Transmits the packet on the target interface
    */
    fn nexthop(self: Arc<Self>, packet: CspPacket) -> io::Result<usize>;

    /**
        Returns an `Arc` to the state struct of the target interface
    */
    fn get_state(&self) -> Arc<Mutex<CspInterfaceState>>;
}
impl core::fmt::Debug for dyn NextHop {
     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "State: {:?}", self.get_state().lock().unwrap())
        }
}
