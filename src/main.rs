pub mod csp;
use csp::{Csp, port::CspSocket};

fn main() {
    let mut csp = Csp::default();
    // lport (receive on): 8080
    // rport (send to): 8090
    csp.add_interface("udp 8080 8090");
    // conn_less = true
    let socket = CspSocket::new(false);
    csp.bind(socket);
    loop {
        csp.route_work();
    }
}

fn server() {
    println!("Server Task Started");
    // Socket (conn_less = false);
    let socket = CspSocket::new(false);
    csp.bind(socket);
}
