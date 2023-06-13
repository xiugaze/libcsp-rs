use std::{sync::{Arc, Mutex}, io};

use crate::csp::types::CspPacket;

use super::{NextHop, CspInterfaceState};

/**
    Interface that can send a packet and just prints it
*/
pub struct DrainInterface { iface: Arc<Mutex<CspInterfaceState>>}

/** 
    Creates a new DrainInterface 
*/
impl DrainInterface {
    pub fn new() -> Self {
        DrainInterface { iface: Arc::new(Mutex::new(CspInterfaceState::from_name("Drain"))) }
    }
}

impl NextHop for DrainInterface {

    /** 
        Consumes and prints the packet
    */
    fn nexthop(self: Arc<Self>, packet: CspPacket) -> io::Result<usize> {
        println!("DRAINED: {}", packet);
        Ok(0)
    }

    fn get_state(&self) -> Arc<Mutex<CspInterfaceState>> {
        Arc::clone(&self.iface)
    }

}
