## CURRENT PRESSING ISSUE:
Service Handler echoing ping:
- Packet is getting echoed properly, received on the libcsp side's UDP interface
- Somehow, not getting put in queue properly or read from queue properly
    - running parallel debugger shows this
    - unit tests show that the echo function is working properly


- Sockets can have two things inside: a connection pointer or a packet pointer.
    - `csp_recvfrom` tries to read a packet from the queue of the socket
    - `csp_accept` tries to read a connection pointer from the queue of the socket

socket->dest_socket and socket->rx_queue[n] when it's a connection is a bidirectional association. However, it's a soft association because it's C, so they're really just pointers.

Currently, socket has a spot for one single connections. We can't currently queue connections into sockets. 

## TODO 
-x ] *listen on all ports*: needed for service handler
-[x] are connections just getting added? What data structures? -> switch to arrays
-[x] connections are just getting made over and over
-[ ] csp_mutex (as a trait, wrapped around std Mutex or whatever Mutex), with timeout parameter
-[ ] connection queue on a socket
    - listen -> connection socket
    - read -> packet socket
-[x] connections need to have an sport_outgoing, possibly?
-[x] implement csp_sendto
-[x] what is closing a connection supposed to do?

