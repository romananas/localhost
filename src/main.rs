use std::collections::HashMap;
use std::mem;
use std::net::TcpListener;
use std::os::fd::{AsRawFd, RawFd};
use std::fs;
use libc::{epoll_create1, epoll_ctl, epoll_event, epoll_wait, EPOLLIN, EPOLL_CTL_ADD};

mod connection;
mod ip;
mod args;
mod options;
mod config;
mod files;
mod utils;
mod interface;

use options::Opts;

fn set_nonblocking(fd: RawFd) {
    unsafe {
        let flags = libc::fcntl(fd, libc::F_GETFL, 0);
        libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
    }
}

/// Parse all options when using arguments or config file
fn options() -> options::Opts {
    let args = args::parse();
    let content = String::from_utf8(fs::read(args.config).unwrap()).unwrap();
    let cfg = config::get(&content).unwrap();
    match Opts::from_config(cfg) {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("config error : {}",e);
            std::process::exit(1);
        },
    }
}

fn get_listeners(epfd:i32, addrs: Vec<String>) -> HashMap<i32,TcpListener> {
        // HashMap to keep track of listeners and clients
        let mut listeners: HashMap<i32, TcpListener> = HashMap::new();
        for addr in &addrs {
            let listener = TcpListener::bind(addr).unwrap();
            // println!("Serveur en écoute sur {}", addr);
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

        listeners
}

fn event_loop(epfd:i32,addrs: Vec<String>,opts: &mut Opts) {
    let mut events: [epoll_event; 10] = unsafe { mem::zeroed() };
    // Bind to each address and add the listener to epoll
    let mut listeners = get_listeners(epfd, addrs.clone());
    loop {
        println!("Waiting for events...");

        let nfds = unsafe { epoll_wait(epfd, events.as_mut_ptr(), events.len() as i32, -1) };
        // dbg!(nfds);
        if nfds == -1 {
            panic!("Error with epoll_wait");
        }

        for i in 0..nfds as usize {
            opts.links = files::parse_files(opts.index.clone());
            let event_fd = events[i].u64 as RawFd;
            set_nonblocking(event_fd);

            // Check if this event is from a listener
            if listeners.contains_key(&event_fd) {
                if let Some(listener) = listeners.get_mut(&event_fd) {
                    // Accept the incoming connection
                    if let Ok((stream, addr)) = listener.accept() {
                        println!("New connection from: {}", addr);

                        // Handle the new connection in a separate thread or add it to epoll
                        connection::handle(stream,opts.clone());
                    }
                }
            }
        }
    }
}

fn main() {
    let mut opts = options();
    let addrs = opts.clone().address_combinations();

    // sanaitizing files paths for later use
    opts.links = files::parse_files(opts.index.clone());
    // println!("{:#?}",opts.links);

    // Create the epoll instance
    let epfd = unsafe { epoll_create1(0) };
    if epfd == -1 {
        panic!("Error creating epoll instance");
    }

    // Handle events with epoll
    event_loop(epfd, addrs, &mut opts);

}
