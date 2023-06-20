use crate::csp::interfaces::{CspInterfaceState, NextHop};
use crate::csp::qfifo::CspQfifo;
use crate::csp::types::CspPacket;
use crate::csp::{types, utils, CspId};
use std::collections::VecDeque;
use std::io;
use std::net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub struct UdpInterface {
    iface: Arc<Mutex<CspInterfaceState>>,
    state: Arc<Mutex<UdpState>>,
    rx_thread: Mutex<Option<JoinHandle<()>>>,
}

impl UdpInterface {
    /**
       Construct a UDP interface from a hostname and port, a pointer to the global queue
       and a `CspInterfaceState` struct. Fields are stored in an underlying UdpState struct
       in the `state` field.
    */
    pub fn from(
        address: &str,
        port: u16,
        qfifo: &Arc<Mutex<CspQfifo>>,
        iface: CspInterfaceState,
    ) -> Arc<Self> {
        let state = UdpState::from(address, port, qfifo);
        let mut iface = Arc::new(UdpInterface {
            iface: Arc::new(Mutex::new(iface)),
            state: Arc::new(Mutex::new(state)),
            rx_thread: Mutex::new(None),
        });
        Arc::clone(&iface).start_rx_thread();
        iface
    }

    // fn rx_work(self: Arc<Self>, socket: &UdpSocket) {
    //     let mut buf: [u8; 256] = [0; 256];
    //     let (len, src_addr) = socket.recv_from(&mut buf).unwrap();
    //     println!("Message from {src_addr}: ");
    //     utils::dump_buffer(&buf, len);
    //
    //     let result = self.state.lock().unwrap().push_qfifo(
    //         CspPacket::new(len, buf, CspId::default()),
    //         Arc::clone(&self),
    //     );
    // }
    //
    /**
        Start the UDP Receive thread, which will accept incoming connections and
        store incoming packets in the global queue.
    */
    pub fn start_rx_thread(self: Arc<Self>) {
        let udp_state = Arc::clone(&self.state);
        let iface_state = Arc::clone(&self.iface);
        let clone = Arc::clone(&self);
        *self.rx_thread.lock().unwrap() = Some(thread::spawn(move || {
            println!("in thread");
            let mut udp_state = udp_state.lock().unwrap();
            //let mut iface_state = iface_state.lock().unwrap();
            loop {
                println!("in loop");
                let socket = UdpSocket::bind((udp_state.host, udp_state.rport))
                    .expect("Error: Can't create socket");
                let mut buf: [u8; 256] = [0; 256];
                let (len, src_addr) = socket.recv_from(&mut buf).unwrap();
                println!("Message from {src_addr}: ");
                utils::dump_buffer(&buf, len);

                let packet = CspPacket::new(len, buf, CspId::default());
                // NOTE: I don't think this is needed
                match udp_state.push_qfifo(packet, Arc::clone(&clone)) {
                    Ok(_) => {
                        println!("UDP Pushed packet");
                    },
                    Err(_) => println!("UDP Failed to push packet"),
                }
            }
        }));
    }
}

pub struct UdpState {
    host: IpAddr,
    lport: u16,
    rport: u16,
    qfifo: Arc<Mutex<CspQfifo>>,
}

impl UdpState {
    /**
        Construct a `UdpState` struct from a hostname, a port number, and a pointer
        to the global queue.
    */
    pub fn from(host: &str, port: u16, qfifo: &Arc<Mutex<CspQfifo>>) -> Self {
        let host = host.parse::<IpAddr>().unwrap();
        let qfifo = Arc::clone(qfifo);
        UdpState {
            host,
            lport: port,
            rport: port,
            qfifo,
        }
    }

    fn push_qfifo(&mut self, packet: CspPacket, iface: Arc<UdpInterface>) -> Result<usize, io::Error> {
        self.qfifo.lock().unwrap().push(packet, iface)
    }
}

impl NextHop for UdpInterface {
    fn nexthop(self: Arc<Self>, packet: CspPacket) -> io::Result<usize> {
        let lock = self.state.lock().unwrap();
        let socket =
            UdpSocket::bind((lock.host, 8080)).expect("Error: Can't bind to local socket");
        socket.connect(("127.0.0.1", 35535));
        let buf = packet.make_buffer();
        socket.send(&buf)
    }

    fn get_state(&self) -> Arc<Mutex<CspInterfaceState>> {
        let iface = Arc::clone(&self.iface);
        iface
    }
}
