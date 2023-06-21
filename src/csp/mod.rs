use std::{
    collections::VecDeque,
    io,
    rc::Rc,
    sync::{Arc, Mutex},
};

use self::{
    connection::CspConnection,
    interfaces::{
        if_drain::DrainInterface,
        //if_udp::UdpInterface, CspInterfaceState,
        if_loopback::{self, LoopbackInterface},
        if_udp::UdpInterface,
        CspInterfaceState,
        NextHop,
    },
    port::{CspPort, CspSocket},
    qfifo::CspQfifo,
    router::Router,
    types::{CspPacket, CspResult},
};

pub mod connection;
pub mod interfaces;
pub mod port;
pub mod qfifo;
pub mod router;
pub mod tests;
pub mod types;
pub mod utils;

pub type InterfaceList = VecDeque<Arc<dyn NextHop>>;

pub struct Csp {
    pub qfifo: Arc<Mutex<CspQfifo>>,
    pub interfaces: InterfaceList,
    pub num_interfaces: usize,
    router: router::Router,
    pub ports: Arc<Mutex<Vec<CspPort>>>,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CspId {
    priority: u8,
    flags: u8,
    source: u16,
    destination: u16,
    dport: u8,
    sport: u8,
}

impl CspId {
    pub fn copy_from(&mut self, other: CspId) {
        *self = other;
    }
}

pub enum InterfaceType {
    Udp,
}

impl Default for Csp {
    fn default() -> Self {
        let qfifo = Arc::new(Mutex::new(CspQfifo::new()));
        let ports = Arc::new(Mutex::new(Vec::new()));
        Csp {
            qfifo: Arc::clone(&qfifo),
            interfaces: VecDeque::new(),
            num_interfaces: 0,
            ports: Arc::clone(&ports),
            router: Router::new(Arc::clone(&qfifo), Arc::clone(&ports)),
        }
    }
}

impl Csp {
    pub fn add_interface(&mut self, iface_type: &str) {
        let qfifo = Arc::clone(&self.qfifo);
        let strings: Vec<&str> = iface_type.split(" ").collect();
        let iface: Arc<dyn NextHop> = match strings[0] {
            // TODO: Take in an Arc to self, and pass it in
            "loopback" => Arc::new(LoopbackInterface::init(&qfifo, self.num_interfaces)),
            "drain" => Arc::new(DrainInterface::new()),
            "udp" => UdpInterface::from(
                "127.0.0.1",
                strings[1].parse::<u16>().unwrap(),
                strings[2].parse::<u16>().unwrap(),
                &qfifo,
                CspInterfaceState::default(),
            ),
            _ => panic!("Error: invalid interface name (may not exist)"),
        };
        self.interfaces.push_back(iface);
        self.num_interfaces += 1;
    }

    pub fn send_direct(iface: Arc<dyn NextHop>, packet: CspPacket) -> io::Result<usize> {
        iface.nexthop(packet)
    }

    pub fn send_from_list(&mut self, index: usize, packet: CspPacket) -> io::Result<usize> {
        let iface = Arc::clone(&self.interfaces[index]);
        iface.nexthop(packet)
    }

    pub fn read(&self) -> CspPacket {
        let (packet, _) = self.qfifo.lock().unwrap().pop().unwrap();
        packet
    }
    pub fn route_work(&mut self) -> CspResult<()> {
        self.router.route_work()
    }

    /**
        Binds a socket to a port, and returns the port index.
    */
    pub fn bind(&mut self, socket: CspSocket) -> CspResult<usize> {
        let port = CspPort {
            state: port::CspPortState::Open,
            socket,
        };
        let mut ports = self.ports.lock().unwrap();
        ports.push(port);
        Ok(ports.len())
    }
}
