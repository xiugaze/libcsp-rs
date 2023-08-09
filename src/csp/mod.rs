use std::{
    collections::VecDeque,
    io,
    sync::{Arc, Mutex},
};

use self::{
    interfaces::{
        if_drain::DrainInterface,
        if_loopback::{LoopbackInterface},
        if_udp::UdpInterface,
        CspInterfaceState,
        NextHop,
    },
    port::{Port, Socket},
    qfifo::CspQfifo,
    router::Router,
    types::{Packet, CspResult, CspError}, connection::{Connection, ConnectionState},
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
    // pub ports: Arc<Mutex<Vec<Port>>>,
}

pub enum ServicePort {
    Port(u8),
    Compare, 
    Ping, 
    // Process,
    // MemFree,
    Reboot,
    // BufFree,
    Uptime,
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
    pub fn dport(&self) -> u8 {
        self.dport
    }
}

pub enum InterfaceType {
    Udp,
}

impl Default for Csp {
    fn default() -> Self {
        let qfifo = Arc::new(Mutex::new(CspQfifo::new()));
        Csp {
            qfifo: Arc::clone(&qfifo),
            interfaces: VecDeque::new(),
            num_interfaces: 0,
            router: Router::new(Arc::clone(&qfifo)),
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


    pub fn send(&mut self, conn: &Arc<Mutex<Connection>>, packet: Packet) -> CspResult<usize>{
        let conn = Arc::clone(conn);
        let conn = conn.lock().unwrap();
        match conn.conn_state() {
            ConnectionState::Open => {
                Self::send_direct(self, *conn.id_out(), packet, None)
            },
            ConnectionState::Closed => { Err(CspError::ClosedConnection) },
        }
    }

    pub fn send_direct(&mut self, idout: CspId, packet: Packet, routed_from: Option<Arc<dyn NextHop>>) -> CspResult<usize>{
        let mut packet = packet;
        let from_me: bool = routed_from.is_none();

        /*
        * TODO: 
        * 1. Send to destination address on local subnet
        * 2. Send via routing table
        */

        let default = Arc::clone(self.interfaces.get(0).unwrap());

        // Copy identifier to packet
        packet.set_id(idout);
        Self::send_direct_iface(default, packet)
    }

    pub fn send_direct_iface(iface: Arc<dyn NextHop>, packet: Packet) -> CspResult<usize> {
        match iface.nexthop(packet) {
            Ok(len) => CspResult::Ok(len),
            Err(_) => CspResult::Err(CspError::InterfaceSendFailed),

        }
    }

    pub fn send_from_list(&mut self, index: usize, packet: Packet) -> io::Result<usize> {
        let iface = Arc::clone(&self.interfaces[index]);
        iface.nexthop(packet)
    }

    pub fn conn_close(conn: Arc<Mutex<Connection>>) {
        conn.lock().unwrap().close();
    }

    pub fn read(&self) -> Packet {
        let (packet, _) = self.qfifo.lock().unwrap().pop().unwrap();
        packet
    }

    pub fn route_work(&mut self) -> CspResult<()> {
        self.router.route_work()
    }

    /**
        Binds a socket to a port, and returns the port index.
    */
    pub fn bind(&mut self, socket: &Arc<Mutex<Socket>>, port: u8) -> CspResult<usize> {
        self.router.bind(socket, port)
    }

    pub fn connect(&mut self, priority: u8, destination: u16, destination_port: u8) -> Arc<Mutex<Connection>> {
        self.router.connect(priority, destination, destination_port)
    }

    pub fn check_service_port(port: u8) -> ServicePort {
        match port { 
            0 => ServicePort::Compare,
            1 => ServicePort::Ping,
            //2 => ServicePort::Process,
            //3 => ServicePort::MemFree,
            4 => ServicePort::Reboot,
            //5 => ServicePort::BufFree,
            6 => ServicePort::Uptime,
            _ => ServicePort::Port(port)
        }
    }

    pub fn service_handler(&self, packet: Packet) {
        match Csp::check_service_port(packet.id().dport) {
            ServicePort::Port(_) => todo!(),
            ServicePort::Compare => todo!(),
            ServicePort::Ping => todo!(),
            ServicePort::Reboot => todo!(),
            ServicePort::Uptime => todo!(),
        }

    }
}
