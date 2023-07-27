# current issue

- Sockets can have two things inside: a connection pointer or a packet pointer.
    - `csp_recvfrom` tries to read a packet from the queue of the socket
    - `csp_accept` tries to read a connection pointer from the queue of the socket
    
The router tries to queue up a connection when?

socket->dest_socket and socket->rx_queue[n] when it's a connection is a bidirectional association. However, it's a soft association because it's C, so they're really just pointers.

# 7/25

csp_accept gets a None????

Problem: csp_accept is called on a socket, which is owned by the top level memory structure. 
Solution: Port holds an Arc<Mutex<Socket>> instead of socket, can be kept on the top level. 


