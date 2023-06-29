use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{sync, thread};

use super::connection::{ConnectionState, ConnectionType, CspConnection};
use super::port::CspPort;
use super::qfifo::CspQfifo;
use super::types::{CspError, CspResult};
use super::{Csp, CspId};

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
    ports: Arc<Mutex<Vec<CspPort>>>,
    connections: Vec<CspConnection>,
}
impl Router {
    pub fn new(qfifo: Arc<Mutex<CspQfifo>>, ports: Arc<Mutex<Vec<CspPort>>>) -> Self {
        // TODO: Implement
        Router {
            qfifo,
            ports,
            connections: Vec::new(),
        }
    }

    // TODO: Fix error types/Ok("message")?
    pub fn route_work(&mut self) -> CspResult<()> {
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
            Csp::send_direct(iface, packet);
            return Ok(());
        }

        /*
            TODO: Handle callbacks
            let callback = get_callback(packet->id.dport);
            if callback not null {
                callback(packet)
            } return
        */

        // TODO: Make this better
        let socket = &mut self.ports.lock().unwrap()[packet.id().dport as usize].socket;

        /* If connectionless, add the packet directly to the socket queue */
        if socket.is_conn_less() {
            // socket.add_connection(packet);
            return Ok(());
        }

        let index = self.find_connection_index(packet.id());
        let connection: &mut CspConnection = match index {
            /* Find an existing connection */
            Some(index) => {
                let conn = &mut self.connections[index];
                conn
            }
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

                let conn = CspConnection::from_ids(sid.clone(), did, ConnectionType::Server);
                self.connections.push(conn);
                self.connections.last_mut().unwrap()
            }
        };
        connection.push(packet);
        Ok(())
    }

    fn find_connection_index(&self, id: &CspId) -> Option<usize> {
        for (i, conn) in self.connections.iter().enumerate() {
            let conn_status = (conn.id_in().dport, conn.id_in().sport, conn.id_in().source);
            let id_status = (id.dport, id.sport, id.source);
            let found = match conn.conn_type() {
                ConnectionType::Client => conn_status.0 == id_status.0,
                ConnectionType::Server => conn_status == id_status,
            };
            if found {
                return Some(i);
            };
        }
        None
    }

    // TODO: Implement
    fn route_security_check() {
        // do nothing
    }

    pub fn get_connection_pool(&mut self) -> &mut Vec<CspConnection> {
        &mut self.connections
    }
}
