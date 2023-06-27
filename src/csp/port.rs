use std::collections::VecDeque;

use super::types::CspPacket;

pub enum CspPortState {
    Closed,
    Open,
    // Callback,
}

pub struct CspPort {
    pub state: CspPortState,
    pub socket: CspSocket,
}

impl CspPort {
    pub fn get_socket(&mut self) -> &mut CspSocket {
        &mut self.socket
    }
}
// /**
//    @defgroup CSP_SOCKET_OPTIONS CSP Socket options.
//    @{
// */
// #define CSP_SO_NONE			0x0000 //!< No socket options
// #define CSP_SO_RDPREQ			0x0001 //!< Require RDP
// #define CSP_SO_RDPPROHIB		0x0002 //!< Prohibit RDP
// #define CSP_SO_HMACREQ			0x0004 //!< Require HMAC
// #define CSP_SO_HMACPROHIB		0x0008 //!< Prohibit HMAC
// #define CSP_SO_CRC32REQ			0x0040 //!< Require CRC32
// #define CSP_SO_CRC32PROHIB		0x0080 //!< Prohibit CRC32
// #define CSP_SO_CONN_LESS		0x0100 //!< Enable Connection Less mode
// #define CSP_SO_SAME			0x8000 // Copy opts from incoming packet only apllies to csp_sendto_reply()
pub struct CspSocket {
    rx_queue: VecDeque<CspPacket>,
    conn_less: bool,
}
impl CspSocket {
    pub fn conn_less() -> CspSocket {
        CspSocket {
            rx_queue: VecDeque::new(),
            conn_less: true,
        }
    }

    pub fn conn() -> CspSocket {
        CspSocket {
            rx_queue: VecDeque::new(),
            conn_less: false,
        }
    }

    pub fn is_conn_less(&self) -> bool {
        self.conn_less
    }

    pub fn push(&mut self, packet: CspPacket) {
        self.rx_queue.push_back(packet);
    }
    pub fn pop(&mut self) -> Option<CspPacket> {
        self.rx_queue.pop_front()
    }

    // TODO: This is just a print function, should probably do something more useful
    pub fn flush_rx_queue(&mut self) {
        for packet in self.rx_queue.drain(0..) {
            print!("{packet}");
        }
    }
}

// pub struct
