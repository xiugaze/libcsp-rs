use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::Rc,
    sync::{Arc, Mutex},
};

use super::{port::Socket, types::Packet, CspId};

#[derive(Debug, Clone, Copy)]
pub enum ConnectionType {
    Client,
    Server,
}

#[derive(Debug, Clone, Copy)]
pub enum ConnectionState {
    Open,
    Closed,
}

/**
    Represents a connection between two sockets???
*/
#[derive(Debug)]
pub struct Connection {
    conn_type: ConnectionType,
    conn_state: ConnectionState,
    rx_queue: VecDeque<Packet>,
    id_out: CspId,
    id_in: CspId,
    dest_socket: Option<Arc<Mutex<Socket>>>,
    sport_outgoing: u8,
}

impl Default for Connection {
    fn default() -> Self {
        Self {
            conn_type: ConnectionType::Client,
            conn_state: ConnectionState::Closed,
            rx_queue: VecDeque::new(),
            id_out: CspId::default(),
            id_in: CspId::default(),
            dest_socket: None,
            sport_outgoing: 0,
        }
    }
}

impl Connection {
    pub fn new(sid: CspId, did: CspId, status: ConnectionType, sport_outgoing: u8) -> Self {
        Connection {
            conn_type: status,
            conn_state: ConnectionState::Closed,
            rx_queue: VecDeque::new(),
            id_out: did,
            id_in: sid,
            dest_socket: None,
            sport_outgoing: 0,
        }
    }

    pub fn id_out(&self) -> &CspId {
        &self.id_out
    }
    pub fn id_in(&self) -> &CspId {
        &self.id_in
    }

    pub fn set_ids(&mut self, id_in: CspId, id_out: CspId) {
        self.id_in = id_in;
        self.id_out = id_out;
    }

    pub fn conn_type(&self) -> ConnectionType {
        self.conn_type
    }

    pub fn conn_state(&self) -> ConnectionState {
        self.conn_state
    }

    pub fn push(&mut self, packet: Packet) {
        self.rx_queue.push_back(packet);
    }

    pub fn read(&mut self) -> Option<Packet> {
        self.rx_queue.pop_front()
    }

    pub fn close(&mut self) {
        self.conn_state = ConnectionState::Closed;
    }
    pub fn open(&mut self) {
        self.conn_state = ConnectionState::Open;
    }

    pub fn set_destination_socket(&mut self, socket: &Arc<Mutex<Socket>>) {
        self.dest_socket = Some(Arc::clone(socket));
    }

    pub fn get_destination_socket(&mut self) -> Option<Arc<Mutex<Socket>>> {
        match &self.dest_socket {
            Some(socket) => Some(Arc::clone(&socket)),
            None => None,
        }
    }

    pub fn dport(&self) -> u8 {
        self.id_in().dport
    }

    pub fn into_server(&mut self) {
        self.conn_type = ConnectionType::Server;
    }

    pub fn into_client(&mut self) {
        self.conn_type = ConnectionType::Client;
    }
}
