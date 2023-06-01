use std::sync::atomic::Ordering;
use std::{sync, thread};
use std::sync::atomic::AtomicBool;

#[derive(Default)]
pub struct Router {
    thread: Option<thread::JoinHandle<()>>,
    alive: sync::Arc<AtomicBool>,
}

impl Router {
    pub fn new() -> Self {
        // TODO: Implement
        Router {
            thread: None,
            alive: sync::Arc::new(AtomicBool::new(false)),
        }
    }

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

    pub fn route_work() {

    }


        
}
