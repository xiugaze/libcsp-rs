use crate::csp::interfaces::{CspInterfaceState, NextHop};
use crate::csp::qfifo::CspQfifo;
use crate::csp::types::Packet;
use crate::csp::{utils, CspId};
use std::io;
use std::net::UdpSocket;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub struct UdpInterface {
    iface: Arc<Mutex<CspInterfaceState>>,
    state: Arc<Mutex<UdpQfifo>>,
    rx_thread: Mutex<Option<JoinHandle<()>>>,
    host: IpAddr,
    lport: u16,
    rport: u16,
    socket: UdpSocket,
}

impl UdpInterface {

    /**
       Construct a UDP interface from a hostname and port, a pointer to the global queue
       and a `CspInterfaceState` struct. Fields are stored in an underlying UdpState struct
       in the `state` field.
    */
    pub fn from(
        address: &str,
        lport: u16,
        rport: u16,
        qfifo: &Arc<Mutex<CspQfifo>>,
        iface: CspInterfaceState,
    ) -> Arc<Self> {
        let state = UdpQfifo::from(qfifo);
        let iface = Arc::new(UdpInterface {
            iface: Arc::new(Mutex::new(iface)),
            state: Arc::new(Mutex::new(state)),
            rx_thread: Mutex::new(None),
            host: address.parse::<IpAddr>().unwrap(),
            lport,
            rport,
            socket: UdpSocket::bind((address, lport)).unwrap(),
        });
        Arc::clone(&iface).start_rx_thread();
        iface
    }

    /**
        Start the UDP Receive thread, which will accept incoming connections and
        store incoming packets in the global queue.
    */
    pub fn start_rx_thread(self: Arc<Self>) {
        let udp_state = Arc::clone(&self.state);
        let clone = Arc::clone(&self);
        *self.rx_thread.lock().unwrap() = Some(thread::spawn(move || {
            let mut udp_state = udp_state.lock().unwrap();
            loop {
                let mut buf: [u8; 256] = [0; 256];
                let (len, src_addr) = clone.socket.recv_from(&mut buf).unwrap();
                println!("Message from {src_addr}: ");
                utils::dump_buffer(&buf, len);

                let packet = Packet::new(len, buf);
                udp_state.push_qfifo(packet, Arc::clone(&clone));
            }
        }));
    }

    pub fn ports(&self) -> (u16, u16) {
        (self.lport, self.rport)
    }
}

pub struct UdpQfifo {
    qfifo: Arc<Mutex<CspQfifo>>,
}

impl UdpQfifo {

    pub fn from(qfifo: &Arc<Mutex<CspQfifo>>) -> Self {
        UdpQfifo { qfifo: Arc::clone(qfifo) }
    }

    fn push_qfifo(
        &mut self,
        packet: Packet,
        iface: Arc<UdpInterface>,
    ) -> Result<usize, io::Error> {
        self.qfifo.lock().unwrap().push(packet, iface)
    }
}

impl NextHop for UdpInterface {

    fn nexthop(self: Arc<Self>, packet: Packet) -> io::Result<usize> {
        let buf = packet.make_buffer();
        self.socket.send_to(&buf, (self.host, self.rport))
    }

    fn get_state(&self) -> Arc<Mutex<CspInterfaceState>> {
        Arc::clone(&self.iface)
    }
}
