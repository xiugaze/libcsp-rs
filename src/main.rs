pub mod csp;
use std::{fs::read, sync::Arc};

use csp::{Csp, InterfaceType, types::CspPacket, CspId, port::CspSocket};

use crate::csp::utils::test_buffer;

fn main() {

    // initialize csp
    let mut csp = Csp::default();
    // add loopback interface
    csp.add_interface("loopback");

    // send packet on buffer (enqueue on global qfifo)
    let buffer = test_buffer();
    let packet = CspPacket::new(256, buffer, CspId::default());
    Csp::send_direct(Arc::clone(csp.interfaces.get(0).unwrap()), packet);

    // initialize a server?
    let socket = CspSocket::new(true);
    csp.bind(socket);
    csp.route_work();
    for port in csp.ports.lock().unwrap().iter_mut() {
        port.socket.flush_rx_queue();
    }

}
