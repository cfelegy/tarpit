use std::net::{SocketAddr, TcpListener};
use log::{info, error};

const PORT: u16 = 8080;

fn main() {
    env_logger::init();

    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], PORT)))
        .expect(format!("Failed to bind to 0.0.0.0:{}", PORT).as_str());

    info!("Listening for connections on 0.0.0.0:{}", PORT);

    for stream in listener.incoming() {
        match stream {
            Ok(tcp) => {
                let peer_addr = tcp.peer_addr().expect("Failed to unwrap peer address");
                info!("Peer connected: {}", peer_addr);
            },
            Err(err) => error!("Failed to handle incoming connection: {:?}", err)
        }
    }
}
