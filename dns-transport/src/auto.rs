use log::*;

use dns::{Request, Response};
use crate::GenericTransport;

use super::{Transport, Error, UdpTransport, TcpTransport};


/// The **automatic transport**, which sends DNS wire data using the UDP
/// transport, then tries using the TCP transport if the first one fails
/// because the response wouldn’t fit in a single UDP packet.
///
/// This is the default behaviour for many DNS clients.
pub struct AutoTransport {
    addr: String,
    port: u16
}

impl AutoTransport {

    /// Creates a new automatic transport that connects to the given host.
    pub fn new(addr: GenericTransport) -> Self {
        if addr.port_num != 0 {
            Self {
            addr : addr.address,
            port : addr.port_num,
            }
        } else {
            Self {
                addr : addr.address,
                port : 53
            }
        }
    }
}


impl Transport for AutoTransport {
    fn send(&self, request: &Request) -> Result<Response, Error> {
        let udp_transport = UdpTransport::new(GenericTransport {
            address: self.addr.clone(),
            port_num: self.port.clone(),
        });
        let udp_response = udp_transport.send(&request)?;

        if ! udp_response.flags.truncated {
            return Ok(udp_response);
        }

        debug!("Truncated flag set, so switching to TCP");

        let tcp_transport = TcpTransport::new(GenericTransport {
            address: self.addr.clone(),
            port_num: self.port.clone(),
        });
        let tcp_response = tcp_transport.send(&request)?;
        Ok(tcp_response)
    }
}
