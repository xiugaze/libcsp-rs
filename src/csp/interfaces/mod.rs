pub mod if_udp;

// Common metadata for all interfaces
// Interfaces are a struct that holds a CspInterface and 
// implements NextHop
pub struct CspInterface {
}

pub trait NextHop {
    fn nexthop(&self);
    // TODO: Move out to UDP type only
    // fn start_thread(&mut self, iface: &mut Arc<Mutex<Box<dyn NextHop>>>, qfifo: CspQueue)
}


