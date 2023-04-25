static mut ports: [CspPort; 200] = [CspPort::default(); 200];

enum CspPortState {
    Closed, 
    Open, 
    OpenCallback,
}

enum CspSocket {
    Some(u32), 
    Default,
}

#[derive(Debug, Copy)]
struct CspPort {
    state: CspPortState,
    // union {
    //  csp_socket_t * socket,
    //  csp_callback_t callback;
    // }
    socket: CspSocket,
} 

impl CspPort {
    fn default() -> Self {
        CspPort {
            state: CspPortState::Closed,
            socket: CspSocket::Default, 
        }
    }

}

fn csp_bind(socket: CspSocket, port: u8) -> i32 {
    let port = CspPort {
        state: CspPortState::Open,
        socket: socket,
    };

    0
}
