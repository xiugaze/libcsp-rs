#[cfg(test)]
use super::*;
use std::{net::UdpSocket, time::Duration, thread};

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
    csp.add_interface("udp");

    // client UDP port
    let client = UdpSocket::bind(("127.0.0.1", 0)).unwrap();
    client.connect(("127.0.0.1", 8080));

    // server CSP port
    let socket = CspSocket::new(false);
    csp.bind(socket);

    // buffer for packet and UDP send
    let buffer = utils::test_buffer();
    let packet = CspPacket::new(256, buffer, CspId::default());

    // send packet as [u8; 256]
    client.send(&buffer);
    std::thread::sleep(Duration::from_millis(100));

    // route the packet
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
fn test_udp_send() {
    let mut csp = Csp::default();
    csp.add_interface("udp");
    println!("test started");


    let server = thread::spawn(|| {
        let sock = UdpSocket::bind(("127.0.0.1", 35535)).unwrap();
        // server.connect(("127.0.0.1", 8080));
        let mut buf = [0; 256];
        let (len, src_addr) = sock.recv_from(&mut buf).unwrap();
        println!("{buf:?}");
    });

    Csp::send_direct(Arc::clone(csp.interfaces.get(0).unwrap()), CspPacket::new(256, utils::test_buffer(), CspId::default()));

    server.join();

    // 
}
