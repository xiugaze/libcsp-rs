use std::collections::VecDeque;

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
    pub fn connect(priority: u8, destination: u16, dport: u8) -> Self {
        let incoming_id = CspId {
            destination: 0, 
            source: destination,
            priority, 
            sport: dport, 
            dport: 0,       // no value?
            flags: 0,
        };
        let outgoing_id = CspId {
            destination, 
            source: 0,
            priority, 
            sport: 0, 
            dport,
            flags: 0,
        };

        CspConnection { 
            conn_type: ConnectionType::Client, 
            conn_state: ConnectionState::Open, 
            rx_queue: VecDeque::new(),
            id_out: outgoing_id,
            id_in: incoming_id 
        }
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
}
