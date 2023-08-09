- Sockets can have two things inside: a connection pointer or a packet pointer.
    - `csp_recvfrom` tries to read a packet from the queue of the socket
    - `csp_accept` tries to read a connection pointer from the queue of the socket

socket->dest_socket and socket->rx_queue[n] when it's a connection is a bidirectional association. However, it's a soft association because it's C, so they're really just pointers.

Currently, socket has a spot for one single connections. We can't currently queue connections into sockets. 


## Packet ID's

On csp connect: 
```c
conn {
    incoming_id {
        src: 0, src_port: dport_parameter, 
        dest: 0, dport: 0
    }
    outgoing_id {
        src: 0, src_port: 0, 
        dest: 0, dport: dport_parameter,
    }
}
```
Then: 
```c
incoming_id.dport = sport_outgoing;
outgoing_id.sport = sport_outgoing;
```
`sport_outgoing` is set in `csp_conn_init()` for each connection.

Packet IDs are NOT SET at all until the send step, ie not set by user. 
