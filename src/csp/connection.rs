use std::{collections::VecDeque, rc::Rc, cell::RefCell};

use super::{types::CspPacket, CspId};

#[derive(Clone, Copy)]
pub enum ConnectionType {
    Client,
    Server,
}

pub enum ConnectionState {
    Open,
    Closed,
}

pub struct CspConnection {
    conn_type: ConnectionType,
    conn_state: ConnectionState,
    rx_queue: VecDeque<CspPacket>,
    id_out: CspId, 
    id_in: CspId,
}

impl CspConnection {
    pub fn from_ids(sid: CspId, did: CspId, status: ConnectionType) -> Self {
        CspConnection { conn_type: status, conn_state: ConnectionState::Open, rx_queue: VecDeque::new(), id_out: did, id_in: sid }
    }

    pub fn id_out(&self) -> &CspId {
        &self.id_out
    }
    pub fn id_in(&self) -> &CspId {
        &self.id_in
    }
    pub fn conn_type(&self) -> ConnectionType {
        self.conn_type
    }

    pub fn push(&mut self, packet: CspPacket) { 
        self.rx_queue.push_back(packet);
    }

    pub fn pop(&mut self) -> Option<CspPacket> { 
        self.rx_queue.pop_front()
    }

}
