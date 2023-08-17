pub mod csp;
use std::sync::{Arc, Mutex};

use csp::{port::Socket, types::Packet, utils::test_buffer, Csp};

fn main() {
    // let mut csp = Csp::default();
    // csp.add_interface("loopback");
    //
    // for _ in 0..2 {
    //
    //     /*
    //     * Client/Server initialization
    //     */
    //     // 1: Make a new connection and add it to the connection pool;
    //     let priority = 2;
    //     let server_address = 0;
    //     let server_port = 10;
    //     // client connects to server
    //     let client_conn = csp.connect(priority, server_address, server_port);
    //
    //     // 2: Initialize the server
    //     let socket = Socket::conn();
    //     // bind socket to port 10
    //     let _ = csp.bind(&socket, 10);
    //
    //     // 3: Make a packet (id is set in send_direct from connection)
    //     let packet = Packet::from(256, test_buffer());
    //
    //     // 4: Send the packet
    //     // TODO: Currently on default interface
    //     csp.send(&client_conn, packet);
    //
    //     // 5. Close the connection
    //     client_conn.lock().unwrap().close();
    //
    //     /*
    //     * Router
    //     *
    //     * At this point, the loopback interface should have sent and then
    //     * received the packet and added it to the global queue. We can then
    //     * route the packet to the correct endpoint.
    //     */
    //     csp.route_work();
    //     let server_conn = socket.lock().unwrap().accept();
    //     if server_conn.is_some() {
    //         let server_conn = server_conn.unwrap();
    //         let packet = server_conn.lock().unwrap().read().unwrap();
    //         if packet.id().dport() == server_port {
    //             println!("Packet received on server_port {}: {:?}", server_port, packet.data())
    //         } else {
    //             csp.service_handler(packet);
    //         }
    //         Csp::conn_close(server_conn);
    //     }
    // }
    //
    let mut csp = Csp::default();
    csp.add_interface("udp 8080 8090");

    let server_port = 10;
    let socket = Socket::conn();
    csp.bind(&socket, server_port);

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
