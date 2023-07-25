use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{sync, thread};
use std::cmp;

use super::connection::{ConnectionState, ConnectionType, Connection};
use super::port::Port;
use super::port::PortState;
use super::port::Socket;
use super::qfifo::CspQfifo;
use super::types::{CspError, CspResult}; use super::{Csp, CspId};

/**
   @file

   Routing table.

   The routing table maps a CSP destination address to an interface (and optional a via address).

   Normal routing: If the route's via address is set to #CSP_NO_VIA_ADDRESS, the packet will be sent directly to the destination address
   specified in the CSP header, otherwise the packet will be sent the to the route's via address.
*/

#[derive(Default)]
pub struct Router {
    qfifo: Arc<Mutex<CspQfifo>>,
    ports: Vec<Arc<Mutex<Port>>>,
    connections: Vec<Arc<Mutex<Connection>>>,
}
impl Router {
    pub fn new(qfifo: Arc<Mutex<CspQfifo>>) -> Self {
        Router {
            qfifo,
            ports: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn bind(&mut self, socket: Socket, index: u8) -> CspResult<usize> {
        if index <= 16 {
            let port = Port {
                state: PortState::Open,
                socket: Socket::conn_less()
            };
            self.ports[index as usize] = Arc::new(Mutex::new(port));
            return Ok(self.ports.len())
        } else {
            return Err(CspError::InvalidPort)
        }
    }

    // pub fn port_get_socket(&self, port: u8) -> Option<&Socket> {
    //     if port <= 16 && self.ports[port as usize].lock().unwrap().is_open() {
    //         return Some(&self.ports[port as usize].lock().unwrap().socket)
    //     }
    //     None
    // }

    // TODO: Fix error types/Ok("message")?
    /**
        Routes the next packet from the global packet queue
        Outgoing: Sends the packet on the corresponding interface
        Inbound
    */
    pub fn route_work(&mut self) -> CspResult<()> {

        // Get the packet to route, either outgoing or inbound
        let (packet, iface) = match self.qfifo.lock().unwrap().pop() {
            Some((packet, iface)) => (packet, iface),
            // Return error if the queue is empty
            None => return Err(CspError::EmptyQfifo),
        };

        // increment received packets
        iface.get_state().lock().unwrap().increment_rx();

        /*
            Interface has an address field which is?
        */
        let is_to_me: bool = packet.id().destination == iface.get_state().lock().unwrap().address();

        // if the message isn't to me, send the mesage to the correct interface
        if !is_to_me {
            // TODO: Handle this result
            Csp::send_direct_iface(iface, packet);
            return Ok(());
        }

        /*
            TODO: Handle callbacks
            let callback = get_callback(packet->id.dport);
            if callback not null {
                callback(packet)
            } return``
        */

        // TODO: Make this better
        // let something =

        let socket = match self.ports.get(packet.id().dport as usize) {
            Some(port) =>  {
                let mut port = port.lock().unwrap();
                port.socket
            },
            /* FIX: What is this error? I think this means that the socket is unbound?
               but binding the socket is not dependent on index? 
            */
            None => return Err(CspError::NoPort)
        };

        if socket.is_conn_less() {
            socket.push(packet);
            return Ok(());
        }

        /* Find an existing connection */
        let connection: Arc<Mutex<Connection>> = match self.find_existing(packet.id()) {
            Some(conn) => conn,
            /* Accept a new incoming connection */

            None => {
                // security check
                Router::route_security_check();
                let sid = packet.id();
                let did = CspId {
                    priority: sid.priority,
                    flags: sid.flags,
                    source: sid.destination,
                    destination: sid.source,
                    dport: sid.sport,
                    sport: sid.dport,
                };

                let conn = Arc::new(Mutex::new(Connection::new(
                    sid.clone(),
                    did,
                    ConnectionType::Server,
                )));
                self.connections.push(Arc::clone(&conn));
                conn
            }
        };
        connection.lock().unwrap().push(packet);
        Ok(())
    }

    fn find_existing(&self, id: &CspId) -> Option<Arc<Mutex<Connection>>> {
        for conn in self.connections.iter() {
            let conn_lock = conn.lock().unwrap();
            let conn_status = (
                conn_lock.id_in().dport,
                conn_lock.id_in().sport,
                conn_lock.id_in().source,
            );
            let id_status = (id.dport, id.sport, id.source);
            let found = match conn_lock.conn_type() {
                ConnectionType::Client => conn_status.0 == id_status.0,
                ConnectionType::Server => conn_status == id_status,
            };
            if found {
                return Some(Arc::clone(&conn));
            }
        }
        None
    }

    /**
        Initializes a connection and adds it to the connection pool (inside router struct).
        Returns an Arc<Mutex<CspConnection>> pointing to the connection in the pool.
    */
    pub fn connect(
        &mut self,
        priority: u8,
        destination: u16,
        destination_port: u8,
    ) -> Arc<Mutex<Connection>> {
        let incoming_id = CspId {
            priority,            // same priority
            flags: 0,            // no flags
            source: destination, // from incoming
            destination: 0,      // disables input filter on destination node? (csp_conn.c)
            dport: 0,            // temp, changes later on
            sport: destination_port,
        };

        let outgoing_id = CspId {
            priority,
            flags: 0,
            source: 0,
            destination: destination,
            dport: destination_port,
            sport: 0,
        };

        let conn = Arc::new(Mutex::new(Connection::new(
            incoming_id, outgoing_id, ConnectionType::Client
        )));

        self.connections.push(Arc::clone(&conn));
        conn
    }

    // TODO: Implement
    fn route_security_check() {
        // do nothing
    }

    pub fn connection_find_dport(&self, dport: u8) -> Option<Arc<Mutex<Connection>>> {
        for conn in self.connections.iter() {
            let conn_lock = conn.lock().unwrap();
            if conn_lock.id_in().dport != dport {
                continue;
            }
            if let ConnectionType::Server = conn_lock.conn_type() {
                continue;
            }
            if conn_lock.id_in().dport != dport {
                continue;
            }
            return Some(Arc::clone(conn));
        }
        None
    }
}
