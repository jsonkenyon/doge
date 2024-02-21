//! Creating DNS transports based on the user’s input arguments.

use doge_transport::*;
use log::debug;


/// A **transport type** creates a `Transport` that determines which protocols
/// should be used to send and receive DNS wire data over the network.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TransportType {

    /// Send packets over UDP or TCP.
    /// UDP is used by default. If the request packet would be too large, send
    /// a TCP packet instead; if a UDP _response_ packet is truncated, try
    /// again with TCP.
    Automatic,

    /// Send packets over UDP only.
    /// If the request packet is too large or the response packet is
    /// truncated, fail with an error.
    UDP,

    /// Send packets over TCP only.
    TCP,

    /// Send encrypted DNS-over-TLS packets.
    TLS,

    /// Send encrypted DNS-over-HTTPS packets.
    HTTPS,
}

/// **port number** creates a `PortNumber` that specifies what [port](https://datatracker.ietf.org/doc/html/rfc1340)
/// to send the dns query to.
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct PortNumber {
    /// A port number to send a query to. If '**None**' is given we will assume the defaults for the protocol
    /// as per [RFC 1340](https://datatracker.ietf.org/doc/html/rfc1340)
    pub port: u16,
}

impl PortNumber {

    /// Returns the specified port number given or 'zero', if 'zero' is returned the transport will use the default port
    pub fn new(port_string: Vec<String>) -> Self {
        
        if port_string.len() == 1 {
            Self { port : port_string[0].parse::<u16>().unwrap_or(0) }
        } else  {
            debug!("Multiple ports or no port should/n't be specified, assuming defaults");
            Self { port : 0  }
        }
    }
    
    
}

impl TransportType {

    /// Creates a boxed `Transport` depending on the transport type. The
    /// parameter will be a URL for the HTTPS transport type, and a
    /// stringified address for the others.
    pub fn make_transport(self, param: GenericTransport) -> Box<dyn Transport> {
        match self {
            Self::Automatic  => Box::new(AutoTransport::new(param)),
            Self::UDP        => Box::new(UdpTransport::new(param)),
            Self::TCP        => Box::new(TcpTransport::new(param)),
            Self::TLS        => Box::new(TlsTransport::new(param)),
            Self::HTTPS      => Box::new(HttpsTransport::new(param)),
        }
    }
}
