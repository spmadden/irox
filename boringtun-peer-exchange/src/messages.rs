use std::fmt::{Debug};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use serde::{Deserialize, Serialize};
use crate::error::Error;


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Peer {
    public_key : [u8;32],
    public_ipv4 : Ipv4Addr,
    public_ipv6 : Ipv6Addr,
    public_port : u16,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
enum PacketType {
    Invalid,
    GetPeers,
    Peers,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Packet {
    packet_type : PacketType,
    data_length : u16,
    data : Vec<u8>
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct GetPeersPacket {
    my_info : Peer
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PeersPacket {
    num_peers : u8,
    peers : Vec<Peer>
}

// macro_rules! impl_serialize {
//     () => {
//         pub fn serialize(&self) -> Result<Vec<u8>, Error> {
//             let mut buf : Vec<u8> = Vec::new();
//             return match ciborium::ser::into_writer(self, &mut buf) {
//                 Ok(_) => Ok(buf),
//                 Err(t) => Err(t.into())
//             }
//         }
//     };
// }

impl PeerUpdate {
    // impl_serialize!();

    pub fn parse(input: &[u8]) -> Result<PeerUpdate, Error> {
        ciborium::de::from_reader(input).map_err(Error::from)
    }
    
    pub fn validate(&self) -> Result<(), Error> {
        todo!()
    }
}

impl PeerInfo {
    // impl_serialize!();

    pub fn parse(input: &[u8]) -> Result<PeerInfo, Error> {
        ciborium::de::from_reader(input).map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use std::{net::{IpAddr, Ipv4Addr}, env::consts::OS};
    use chacha20poly1305::{KeyInit, AeadCore, aead::Aead};
    use poly1305::{universal_hash::UniversalHash};
    use rand_core::RngCore;
    use x25519_dalek::{PublicKey, StaticSecret};

    use crate::messages::{PeerInfo, PeerUpdate};

    #[test]
    pub fn test_serialize() {
        
    }
}