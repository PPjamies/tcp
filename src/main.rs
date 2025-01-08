use socket2;
use socket2::{Domain, Protocol, Socket, Type};
use std::io::{Error, Read, Result, Write};
use std::net::SocketAddr;

// connections are added to a queue on the OS - typically the queue size is 128
// the number of sockets a server can open depends on:
// OS file descriptor limit, memory, server configurations, OS network stack
// total # of buffers are limited to RAM
// local IP + local port + remote IP + remote port = socket address

fn main() -> Result<()> {
    // creates new socket
    let socket: Socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;

    // define server address
    let addr: SocketAddr = "127.0.0.1:8080"
        .parse()
        .map_err(|e| Error::new(std::io::ErrorKind::InvalidInput, e))?;

    // bind server address to socket
    socket.bind(&addr.into())?;

    // listening for incoming connections
    socket.listen(128)?;

    loop {
        // accepts new connection
        let (mut client_socket, client_addr) = socket.accept()?;

        std::thread::spawn(move || {
            let mut buf = [0u8; 1024];
            let size = client_socket.read(&mut buf)?;
            let message = String::from_utf8_lossy(&buf[..size]);

            if message.trim() == "shutdown" {
                client_socket.shutdown(std::net::Shutdown::Both)?;
            } else {
                println!("Server received: {}", message);
                client_socket.write_all(b"Message from the server!")?;
            }

            Ok(())
        });
    }
}

fn tcp_client() -> Result<()> {
    // creates new socket
    let mut socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;

    // server address to connect to
    let addr: SocketAddr = "127.0.0.1:8080"
        .parse()
        .map_err(|e| Error::new(std::io::ErrorKind::InvalidInput, e))?;

    // connect
    socket.connect(&addr.into())?;

    // sends message to server
    socket.write_all(b"Message from the client!")?;

    // reads message from server
    let mut buf = [0u8; 1024];
    let size = socket.read(&mut buf)?;
    println!("Client received: {}", String::from_utf8_lossy(&buf[..size]));

    // gracefully shuts down
    socket.write_all(b"shutdown")?;

    Ok(())
}
