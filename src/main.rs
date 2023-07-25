pub mod csp;
use csp::{Csp, port::Socket, types::Packet, utils::test_buffer};

fn main() {
    let mut csp = Csp::default();
    csp.add_interface("loopback");

    for _ in 0..1 {

        /*
        * Client/Server initialization
        */
        // 1: Make a new connection and add it to the connection pool;
        let priority = 2;
        let server_address = 255;
        let server_port = 10;
        let conn = csp.connect(priority, server_address, server_port);

        // 2: Initialize the server
        let socket = Socket::conn();
        csp.bind(socket, 0);

        // 3: Make a packet (id is set in send_direct from connection)
        let packet = Packet::new(256, test_buffer());

        // 4: Send the packet 
        // TODO: Currently on default interface
        csp.send(&conn, packet);

        // 5. Close the connection
        conn.lock().unwrap().close();


        /*
        * Router
        *
        * At this point, the loopback interface should have sent and then 
        * received the packet and added it to the global queue. We can then 
        * route the packet to the correct endpoint. 
        */
        csp.route_work();
    }







}




