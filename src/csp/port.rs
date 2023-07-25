use std::{collections::VecDeque, sync::{Mutex, Arc}};

use super::{types::Packet, connection::Connection};

pub enum PortState {
    Closed,
    Open,
    // Callback,
}

/**
    This port type is equivalent to a bound socket in the original implementation.
*/
pub struct Port {
    pub state: PortState,
    // TODO: Should socket be owned by the port?
    pub socket: Socket,
}

impl Port {
    pub fn bind(socket: Socket) -> Port {
        Port {
            state: PortState::Open,
            socket,
        }
    }

    pub fn close(&mut self) {
        self.state = PortState::Closed;
    }

    pub fn is_open(&self) -> bool {
        match self.state {
            PortState::Closed => false,
            PortState::Open => true,
        }
    }
}

pub enum SocketType {
    Connectionless,
    ConnectionOriented,
}

pub struct Socket  {
    queue: VecDeque<Packet>,
    socket_type: SocketType,
    inc_connection: Option<Arc<Mutex<Connection>>>,
}


impl Socket {

    pub fn conn_less() -> Socket {
        Socket {
            queue: VecDeque::new(),
            socket_type: SocketType::Connectionless,
            inc_connection: None,
        }
    }

    pub fn conn() -> Socket {
        Socket {
            queue: VecDeque::new(),
            socket_type: SocketType::ConnectionOriented,
            inc_connection: None, 
        }
    }

    pub fn is_conn_less(&self) -> bool {
        match self.socket_type {
            SocketType::Connectionless => true,
            SocketType::ConnectionOriented => false,
        }
    }

    /**
        Note: can be called on a *Reference*
    */
    pub fn set_conn(&mut self, conn: &Arc<Mutex<Connection>>) {
        self.inc_connection = Some(Arc::clone(conn));
    }

    /** 
        Attempts to read an connection pointer from the socket 
    */
    pub fn accept(&mut self) -> Option<Arc<Mutex<Connection>>> {
        // swaps the None into inc_connection and returns the connection
        std::mem::replace(&mut self.inc_connection, None)
    }

    pub fn push(&mut self, packet: Packet) {
        self.queue.push_back(packet);
    }


    pub fn receive(&mut self) -> Option<Packet>{
        self.queue.pop_front()
    }
}

