use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;
use std::{sync, thread};
use std::sync::atomic::AtomicBool;

use super::Csp;
use super::qfifo::CspQfifo;

#[derive(Default)]
pub struct Router {
    thread: Option<thread::JoinHandle<()>>,
    alive: sync::Arc<AtomicBool>,
    qfifo: Arc<Mutex<CspQfifo>>,
}

impl Router {
    pub fn new(qfifo: Arc<Mutex<CspQfifo>>) -> Self {
        // TODO: Implement
        Router {
            thread: None,
            alive: sync::Arc::new(AtomicBool::new(false)),
            qfifo
        }
    }

    // TODO: Unimplemented
    pub fn start<F>(&mut self, route_work: F)
        where F: 'static + Send + FnMut() {
        
        self.alive.store(true, Ordering::SeqCst);
        // self.thread = Some(thread::spawn(move || {
        //     let mut route_work = route_work;
        //     while self.alive.load(Ordering::SeqCst) {
        //         route_work();
        //     }
        // })) ;
    }

    pub fn route_work(&mut self) {
        // 1. Get the next packet to route
        // Removes the packet
        let (packet, iface) = self.qfifo.lock().unwrap().pop();

        // increment received packets
        iface.get_state().lock().unwrap().increment_rx();
        
        let is_to_me = packet.id().destination == 
            iface.get_state().lock().unwrap().address();

        if !is_to_me {
            Csp::send_direct(Arc::clone(&iface), packet);
        }

    }
}
