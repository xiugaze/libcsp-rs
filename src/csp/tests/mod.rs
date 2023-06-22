use crate::csp::types::CspError;

#[cfg(test)]
use super::*;
use crate::{
    csp::{self, types, utils, CspId, CspPacket, CspResult, CspSocket},
    Csp,
};
use std::{net::UdpSocket, sync::Arc, thread, time::Duration};

#[test]
fn test_loopback_send_direct() {
    let mut csp = Csp::default();
    csp.add_interface("loopback");

    let packet = CspPacket::new(256, utils::test_buffer(), CspId::default());
    let to_send = packet.clone();

    csp.send_from_list(0, to_send);
    let rec = csp.read();
    assert_eq!(packet, rec);
}

#[test]
fn test_loopback_route_to_socket_conn_less() {
    let mut csp = Csp::default();
    csp.add_interface("loopback");

    // send packet on buffer (enqueue on global qfifo)
    let buffer = utils::test_buffer();
    let packet = CspPacket::new(256, buffer, CspId::default());
    let to_send = packet.clone();

    Csp::send_direct(Arc::clone(csp.interfaces.get(0).unwrap()), to_send);

    // conn_less = true
    let socket = CspSocket::new(true);
    csp.bind(socket);
    csp.route_work();

    let rec = csp
        .ports
        .lock()
        .unwrap()
        .get_mut(0)
        .unwrap()
        .get_socket()
        .pop()
        .unwrap();
    assert_eq!(packet, rec)
}

#[test]
fn test_loopback_route_to_socket_conn() {
    let mut csp = Csp::default();
    csp.add_interface("loopback");

    // send packet on buffer (enqueue on global qfifo)
    let buffer = utils::test_buffer();
    let packet = CspPacket::new(256, buffer, CspId::default());
    let to_send = packet.clone();

    Csp::send_direct(Arc::clone(csp.interfaces.get(0).unwrap()), to_send);

    // conn_less = false
    let socket = CspSocket::new(false);
    csp.bind(socket);
    csp.route_work();

    let rec = csp
        .router
        .get_connection_pool()
        .get_mut(0)
        .unwrap()
        .pop()
        .unwrap();
    assert_eq!(packet, rec)
}

#[test]
fn test_udp_rec() {
    let mut csp = Csp::default();

    // RX thread starts here
    // interface lport rport
    csp.add_interface("udp 8080 0");

    // server CSP port (conn_less = false)
    let socket = CspSocket::new(false);
    let _ = csp.bind(socket);

    // buffer for packet and UDP send
    let buffer = utils::test_buffer();
    let packet = CspPacket::new(256, buffer, CspId::default());

    // send packet as [u8; 256]
    // HACK: "race condition" occurs on route_work() before UDP thread is done
    let mut sent: Result<(), CspError> = CspResult::Err(types::CspError::EmptyQfifo);

    let sender_thread = thread::spawn(move || {
        println!("Starting sender thread");
        let client = UdpSocket::bind(("127.0.0.1", 0)).expect("Error: Could not bind to address");

        println!("Sender thread starting send");
        while sent.is_err() {
            client.send_to(&buffer, ("127.0.0.1", 8080)).unwrap();
        }
        println!("Sender thread done");
    });

    println!("Waiting for route_work success");
    loop {
        let Ok(_sent) = csp.route_work() else {
            continue;
        };
        println!("route_work succeeded");

        let rec = csp
            .router
            .get_connection_pool()
            .get_mut(0)
            .unwrap()
            .pop()
            .unwrap();

        assert_eq!(packet, rec)
    }
}

#[test]
fn test_udp_send() {
    let mut csp = Csp::default();
    csp.add_interface("udp 8080 35535");
    println!("test started");

    let socket = UdpSocket::bind(("127.0.0.1", 35535)).unwrap();
    Csp::send_direct(
        Arc::clone(csp.interfaces.get(0).unwrap()),
        CspPacket::new(256, utils::test_buffer(), CspId::default()),
    );

    let mut buf = [0; 256];
    let len = socket.recv(&mut buf).unwrap();

    let rec = CspPacket::new(len, buf, CspId::default());
    let packet = CspPacket::new(256, utils::test_buffer(), CspId::default());
    assert_eq!(packet, rec);
}

/* #[test]
fn test_udp() {
    let sender = thread::spawn(|| test_udp_send());
    let receiver = thread::spawn(|| test_udp_rec());
    sender.join().unwrap();
    receiver.join().unwrap();
}
 */
#[test]
fn test_udp_rec_send() {
    // csp
    // add udp
    // rec packet that says to send back on interface
    // nexthop
    // rec on other socket
    // compare
}
