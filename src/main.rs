pub mod csp;
use std::sync::{Arc, Mutex};

use csp::{port::Socket, types::Packet, utils::test_buffer, Csp, router::PORT_ANY};

fn main() {

    let mut csp = Csp::default();
    csp.add_interface("udp 8080 8090");

    let server_port = 10;
    let socket = Socket::conn();
    csp.bind(&socket, PORT_ANY);

    loop {
        csp.route_work();
        if let Some(connection) = socket.lock().unwrap().accept() {
            let packet = connection.lock().unwrap().read().unwrap();
            let dport = connection.lock().unwrap().dport();
            if dport == server_port {
                println!(
                    "Packet received on server_port {}: {:?}",
                    server_port,
                    String::from_utf8_lossy(packet.data())
                );
            } else {
                println!("service on port {}", dport);
                csp.service_handler(packet)
            }
            connection.lock().unwrap().close();
        }
    }
}
