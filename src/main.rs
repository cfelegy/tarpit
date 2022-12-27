use std::{net::{SocketAddr, TcpListener, TcpStream}, io::{Write, ErrorKind}, thread, time::Duration};
use log::{info, error};

const PORT: u16 = 8080;

fn main() {
    env_logger::init();

    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], PORT)))
        .expect(format!("Failed to bind to 0.0.0.0:{}", PORT).as_str());

    info!("Listening for connections on 0.0.0.0:{}", PORT);

    for stream in listener.incoming() {
        match stream {
            Ok(mut tcp) => {
                handle_stream(&mut tcp).expect("Failed to handle stream");
            },
            Err(err) => error!("Failed to handle incoming connection: {:?}", err)
        }
    }
}

fn handle_stream(stream: &mut TcpStream) -> Result<(), std::io::Error> {
    let peer_addr = stream.peer_addr()?;
    info!("Peer connected: {}", peer_addr);

    stream.write("HTTP/1.1 200 OK\r\n".as_bytes())?;

    let mut cloned_stream = stream.try_clone()?;

    thread::spawn(move || {
        match tar_stream(&mut cloned_stream) {
            Err(err) => {
                if err.kind() == ErrorKind::BrokenPipe {
                    info!("Peer gave up: {}", peer_addr);
                } else {
                    panic!("Failed to tar stream: {:?}", err);
                }
            }
            Ok(_) => {}
        }
    });

    return Ok(());
}

fn tar_stream(stream: &mut TcpStream) -> Result<(), std::io::Error> {
    loop {
        let header = format!("X-{}: {}\r\n", random_string(), random_string());
        stream.write(header.as_bytes())?;
        stream.flush()?;

        thread::sleep(Duration::from_millis(500));
    }
}

fn random_string() -> String {
    let length = fastrand::usize(5..40);
    let mut chars: Vec<char> = Vec::with_capacity(length);

    for _ in 0..length {
        chars.push(fastrand::alphabetic());
    }

    chars.iter().collect()
}