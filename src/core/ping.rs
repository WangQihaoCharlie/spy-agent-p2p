use std::net::{SocketAddr, UdpSocket};

use std::result::Result;


pub fn ping(ip: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let destination = format!("{}:{}", ip, port);
    let destination: SocketAddr = destination.parse()?;
    socket.send_to(b"Ping", &destination)?;
    Ok(())
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping() {
        assert!(ping("127.0.0.1", 12345).is_ok());
    }
}


