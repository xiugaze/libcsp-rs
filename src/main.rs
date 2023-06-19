pub mod csp;
use csp::Csp;

fn main() {
    let mut csp = Csp::default();
    csp.add_interface("udp");
    loop {}
}
