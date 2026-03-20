use std::net::{SocketAddrV4, SocketAddrV6};

use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

/// Node address
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NodeAddress {
    /// IPv4 address
    V4(SocketAddrV4),

    /// IPv6 address
    V6(SocketAddrV6),

    /// Domain name address
    Domain(#[dyn_length] String, u16),
}
