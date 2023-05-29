
#![allow(dead_code)]
// TODO: For unimplemented!()
#![allow(unreachable_code)]

use std::collections::VecDeque;

use self::{router::Router, interfaces::if_udp::CspInterface};

pub mod utils;
pub mod interfaces;
pub mod buffer;
pub mod types;
pub mod router;


// these are going to be architecture specific, use feature guards??
fn router_start() -> u32 { !unimplemented!() }
fn server_start() -> u32 { !unimplemented!() }
fn client_start() -> u32 { !unimplemented!() }

pub struct Csp {
    qfifo: VecDeque<types::CspPacket>,
    interfaces: VecDeque<Box<dyn CspInterface>>,
    router: router::Router,
}

impl Default for Csp {
    fn default() -> Self {
        // buffer init
        // conn init
        // qfifo: ?
        // interface list?
        Csp {
            qfifo: VecDeque::new(),
            interfaces: VecDeque::new(),
            router: Router::new(),
        }
    }

}

impl Csp {
    pub fn add_interface(&mut self, interface: Box<dyn CspInterface>) {
        self.interfaces.push_back(interface);
    }

    pub fn router_start(&self) {
        let router = self.router.start(Router::route_work);
    }
}

// > 1.  the driver layer forwards the raw data frames to the interface, in
// >     this case CAN frames
// > 2.  the interface will acquire a free buffer (e.g.
// >     `csp_buffer_get_isr()`) for
// >     assembling the CAN frames into a complete packet
// > 3.  once the interface has successfully assembled a packet, the packet
// >     is queued for routing - primarily to decouple the interface, e.g.
// >     if the interfaces/drivers uses interrupt (ISR).
// > 4.  the router picks up the packet from the incoming queue and routes
// >     it on - this can either be to a local destination, or another
// >     interface.
// > 5.  the application waits for new packets at its Rx queue, by calling
// >     `csp_read()` or
// >     `csp_accept` in case it is a server
// >     socket.
// > 6.  the application can now process the packet, and either send it
// >     using e.g. `csp_send()`, or free the
// >     packet using `csp_buffer_free()`.
//

// fn csp_init() {
//     // initialize a buffer
//     csp_buffer_init();
//     // initialize a connection
//     csp_conn_init();
//     // initialize qfifo
//     csp_qfifo_init();
//
//     // loopback
// }

// order:
// 1. driver calles into the interface with the received data, e.g. csp_can_rx()
// 2. Interface converts/copies data into a packet
// 3. Packet is queued for later CSP processing, by calling csp_qfifo_write













