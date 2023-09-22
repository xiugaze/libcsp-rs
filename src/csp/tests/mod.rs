use crate::csp::{types::CspError, utils::test_buffer};

#[cfg(test)]
use super::*;
use super::router::PORT_ANY;
use crate::{
    csp::{self, types, utils, CspId, Packet, CspResult, Socket},
    Csp,
};
use std::{net::UdpSocket, sync::Arc, thread, time::Duration};
#[test]
fn test_csp_id() {
    // ID taken from client_server_test
    let id = CspId {
        priority: 2,
        flags: 0x0,
        source: 0,
        destination: 0,
        dport: 10,
        sport: 18,
    };

    let data = [0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0xA4, 0x80];
    let stripped = Packet::strip_id(data);
    assert_eq!(id, stripped);

    let data = vec![0x80, 0x00, 0x00, 0x00, 0xA4, 0x80];
    assert_eq!(data, Packet::prepend_id(&id));
}

#[test]
fn test_loopback_send_direct() {
    let mut csp = Csp::default();
    csp.add_interface("loopback");

    let packet = Packet::from(256, utils::test_buffer());
    let to_send = packet.clone();

    csp.send_from_list(0, to_send);
    let rec = csp.read();
    assert_eq!(packet, rec);
}

#[test]
fn test_loopback_route_to_socket_conn() {
    let mut csp = Csp::default();
    csp.add_interface("loopback");

    // send packet on buffer (enqueue on global qfifo)
    let mut packet = Packet::from(256, test_buffer());
    let to_send = packet.clone();

    let connection = csp.connect(2, 0, 10).unwrap();
    // TODO: get assigned port out of connection
    let destination = connection.lock().unwrap().dport();

    let sent = csp.sendto(2, 0, destination, 10, to_send);

    // make sure packet was actually sent
    assert_eq!(true, sent.is_ok());

    csp.route_work();

    let rec = connection.lock().unwrap().read().unwrap();

    assert_eq!(packet.data(), rec.data())
}

#[test]
fn test_udp_send() {
    let mut csp = Csp::default();
    csp.add_interface("udp 8090 35535");

    let udp_socket = UdpSocket::bind(("127.0.0.1", 35535)).unwrap();

    let send_packet = Packet::from(256, utils::test_buffer());
    let _ = csp.sendto(2, 0, 0, 0,  send_packet.clone());
    let mut buf = [0; 256];
    let len = udp_socket.recv(&mut buf).unwrap();
    let received_packet = Packet::from(len, buf);
    assert_eq!(send_packet.data(), received_packet.data());
}

/* #[test]
fn test_udp() {
    let sender = thread::spawn(|| test_udp_send());
    let receiver = thread::spawn(|| test_udp_rec());
    sender.join().unwrap();
    receiver.join().unwrap();
} */

#[test]
fn test_udp_rec_send() {
    // csp
    // add udp
    // rec packet that says to send back on interface
    // nexthop
    // rec on other socket
    // compare
}

fn test_connection_pool() {
    let mut csp = Csp::default();
    let first = csp.connect(0, 0, 0).unwrap();
    let second = csp.connect(0, 0, 0).unwrap();
    assert_eq!(0, first.lock().unwrap().dport());
    assert_eq!(1, second.lock().unwrap().dport());

}

// fn test_service_handler() {
//     let mut csp = Csp::default();
//     csp.add_interface("udp 8080 8091");

//     let server_port = 10;
//     let socket = Socket::conn();
//     csp.bind(&socket, PORT_ANY);

//     let udp_socket = UdpSocket::bind(("127.0.0.1", 8091)).unwrap();

//     let id = CspId {
//         priority: 2,
//         flags: 0x0,
//         source: 0,
//         destination: 0,
//         dport: 10,
//         sport: 18,
//     };

//     let packet = Packet::default();

//     udp_socket.send_to(, "127.0.0.1:8080")

    

// }
