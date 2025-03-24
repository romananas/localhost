use std::collections::HashMap;
// use std::io::{Read, Write};
use std::mem;
use std::net::TcpListener;
use std::os::fd::{AsRawFd, RawFd};
// use std::time::Duration;

use libc::{epoll_create1, epoll_ctl, epoll_wait, epoll_event, EPOLLIN, EPOLL_CTL_ADD};

mod connection;
mod ip;
mod args;
mod options;

mod config;

use ip::IPv4;

// fn set_nonblocking(fd: RawFd) {
//     unsafe {
//         let flags = libc::fcntl(fd, libc::F_GETFL, 0);
//         libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
//     }
// }


fn main() {
    // Parse the command line arguments
    let args = args::parse();

    dbg!("{}",args.config.clone());

    // Collect all the IP addresses and ports to bind to
    let addrs = args.addr.iter().map(|f| {
        IPv4::from(f.as_str()).unwrap()
    }).collect::<Vec<IPv4>>();

    // Create the epoll instance
    let epfd = unsafe { epoll_create1(0) };
    if epfd == -1 {
        panic!("Error creating epoll instance");
    }

    // HashMap to keep track of listeners and clients
    let mut listeners = HashMap::new();

    // 1️⃣ Bind to each address and add the listener to epoll
    for addr in &addrs {
        let listener = TcpListener::bind(addr.full()).unwrap();
        listener.set_nonblocking(true).unwrap();
        let listener_fd = listener.as_raw_fd();

        // Add listener to epoll
        let mut event: epoll_event = unsafe { mem::zeroed() };
        event.events = EPOLLIN as u32;
        event.u64 = listener_fd as u64;
        
        unsafe {
            if epoll_ctl(epfd, EPOLL_CTL_ADD, listener_fd, &mut event) == -1 {
                panic!("Error adding listener to epoll");
            }
        }

        listeners.insert(listener_fd, listener);
    }

    // 2️⃣ Handle events with epoll
    let mut events: [epoll_event; 10] = unsafe { mem::zeroed() };
    loop {
        println!("Waiting for events...");

        let nfds = unsafe { epoll_wait(epfd, events.as_mut_ptr(), events.len() as i32, -1) };
        if nfds == -1 {
            panic!("Error with epoll_wait");
        }

        for i in 0..nfds as usize {
            let event_fd = events[i].u64 as RawFd;

            // Check if this event is from a listener
            if listeners.contains_key(&event_fd) {
                if let Some(listener) = listeners.get_mut(&event_fd) {
                    // Accept the incoming connection
                    if let Ok((stream, addr)) = listener.accept() {
                        println!("New connection from: {}", addr);

                        // Handle the new connection in a separate thread or add it to epoll
                        connection::handle(stream);
                    }
                }
            }
        }
    }
}
