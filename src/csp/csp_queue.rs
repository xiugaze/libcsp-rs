use std::collections::VecDeque;
use crate::csp::types;

//
// csp_queue_handle_t csp_queue_create_static(int length, size_t item_size, char * buffer, csp_static_queue_t * queue) {
// 	/* We ignore static allocation for posix for now */
// 	return pthread_queue_create(length, item_size);
// }
//
// pthread_queue_create seems to just create a queue. 

fn csp_queue_create_static(size: usize) -> VecDeque<types::CspPacket> {
    VecDeque::with_capacity(size)
}

