use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

use binary_codec::{BinaryDeserializer, BinarySerializer};
use serde::{Deserialize, Serialize};

/// Node address
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NodeAddress {
    /// IPv4 address
    V4(SocketAddrV4),

    /// IPv6 address
    V6(SocketAddrV6),

    /// Domain name address
    Domain(String, u16),
}

impl<T: Clone> BinarySerializer<T> for NodeAddress {
    fn write_bytes(
        &self,
        stream: &mut binary_codec::BitStreamWriter,
        _config: Option<&mut binary_codec::SerializerConfig<T>>,
    ) -> Result<(), binary_codec::SerializationError> {
        match self {
            NodeAddress::V4(addr) => {
                stream.write_bit(false); // is domain
                stream.write_bit(false); // is ipv6
                stream.write_bytes(&addr.ip().octets());
                stream.write_fixed_int(addr.port());
            }
            NodeAddress::V6(addr) => {
                stream.write_bit(false); // is domain
                stream.write_bit(true); // is ipv6
                stream.write_bytes(&addr.ip().octets());
                stream.write_fixed_int(addr.port());
            }
            NodeAddress::Domain(domain, port) => {
                stream.write_bit(true); // is domain
                stream.write_dyn_int(domain.len() as u128);
                stream.write_bytes(domain.as_bytes());
                stream.write_fixed_int(*port);
            }
        }

        Ok(())
    }
}

impl<T: Clone> BinaryDeserializer<T> for NodeAddress {
    fn read_bytes(
        stream: &mut binary_codec::BitStreamReader,
        _config: Option<&mut binary_codec::SerializerConfig<T>>,
    ) -> Result<Self, binary_codec::DeserializationError> {
        let is_domain = stream.read_bit()?;
        let is_ipv6 = stream.read_bit()?;

        if is_domain {
            let len = stream.read_dyn_int()?;
            let bytes = stream.read_bytes(len as usize)?;
            let domain = String::from_utf8_lossy(bytes).to_string();
            let port = stream.read_fixed_int()?;
            Ok(NodeAddress::Domain(domain, port))
        } else {
            if is_ipv6 {
                let octets: [u8; 16] = stream.read_bytes(16)?.try_into().unwrap();
                let port = stream.read_fixed_int()?;
                let ip = Ipv6Addr::from_octets(octets);
                Ok(NodeAddress::V6(SocketAddrV6::new(ip, port, 0, 0)))
            } else {
                let octets: [u8; 4] = stream.read_bytes(4)?.try_into().unwrap();
                let port = stream.read_fixed_int()?;
                let ip = Ipv4Addr::from_octets(octets);
                Ok(NodeAddress::V4(SocketAddrV4::new(ip, port)))
            }
        }
    }
}
