use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;
use std::{sync, thread};
use std::sync::atomic::AtomicBool;

use super::{Csp, CspId};
use super::connection::{CspConnection, ConnectionType};
use super::port::CspPort;
use super::qfifo::CspQfifo;

#[derive(Default)]
pub struct Router {
    thread: Option<thread::JoinHandle<()>>,
    alive: sync::Arc<AtomicBool>,
    qfifo: Arc<Mutex<CspQfifo>>,
    ports: Arc<Mutex<Vec<CspPort>>>,
    connections: Vec<Rc<CspConnection>>,
}

impl Router {
    pub fn new(qfifo: Arc<Mutex<CspQfifo>>, ports: Arc<Mutex<Vec<CspPort>>>) -> Self {
        // TODO: Implement
        Router {
            thread: None,
            alive: sync::Arc::new(AtomicBool::new(false)),
            qfifo,
            ports, 
            connections: Vec::new(),
        }
    }

    pub fn route_work(&mut self) {
        // 1. Get the next packet to route
        // Removes the packet
        let (packet, iface) = self.qfifo.lock().unwrap().pop();

        // increment received packets
        iface.get_state().lock().unwrap().increment_rx();
        
        let is_to_me = packet.id().destination == 
            iface.get_state().lock().unwrap().address();

        // if the message isn't to me, send the mesage to the correct interface
        if !is_to_me {
            Csp::send_direct(iface, packet);
            return;
        }

        // Ok, now we're handling callbacks
        /*
            let callback = get_callback(packet->id.dport);
            if callback not null {
                callback(packet)
            }
            return
        */

        // Ok, now we're handling port stuff
        /*
            socket = csp_port_get_socket(packet->id.dport);
            if(socket is connectionless) {
                socket.add_to_rx(packet)
                return
            }

            connection = get_connection(packet->id)
            if !exists {
               // accept a new incoming connection
               connection = new connection(packet-> id, make idout)
            } else {
                security check
            }
            
            // finally
            connection.add_to_rx_queue(packet)

        */
        let socket = &mut self.ports.lock().unwrap()[packet.id().dport as usize].socket;
        if socket.conn_less() {
            socket.enqueue(packet);
            return;
        }

        let index = self.find_connection_index(packet.id());
        let mut connection: Rc<CspConnection> = match index {
            Some(index) =>  {
                connection = Rc::clone(&self.connections[index]);
                do_security_check()
            },
            None => {
                push_new_connection()
            },
        }
    }

    fn find_connection_index(&self, id: &CspId) ->  Option<usize> {
        for (i, conn) in self.connections.iter().enumerate() {
            let conn_status = (conn.id_in().dport, conn.id_in().sport, conn.id_in().source);
            let id_status = (id.dport, id.sport, id.source);
            let found = match conn.conn_type() {
                ConnectionType::Client => conn_status.0 == id_status.0,
                ConnectionType::Server => conn_status == id_status,
            };
            if found { return Some(i) };
        }
        None
    }

}
