pub mod csp;
use csp::Csp;

fn main() {
    let mut csp = Csp::default();
    csp.add_interface("udp 8080 0");
    loop {
        csp.route_work();
    }
}
