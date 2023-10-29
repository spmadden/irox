// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter};
use irox_tools::arrays::max_index;
use irox_tools::u16::{FromU16Array, ToU16Array};

///
/// A Layer-2 Ethernet Media-Access-Control Address (MAC)
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MAC {
    bytes: [u8; 6],
}

///
/// A generic Internet Protocol network.  Could be either an [`IPv4Network`] or an [`IPv6Network`]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IPNetwork {
    IPv4(IPv4Network),
    IPv6(IPv6Network),
}

///
/// An error returned by the various processing functions.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NetworkError {
    /// The specified CIDR is not a valid CIDR
    InvalidCIDR(u8),
    /// The specified mask is not a valid mask of the type required.
    InvalidMask(u32),
    /// The specified number is not a power-of-two
    NotAPowerOfTwo(u32),
    /// The specified [`IPAddress`] does not represent a network ID, but is a host or a broadcast.
    NotANetworkAddress(IPAddress),
}

/// A generic Internet Protocol Address, could be a [`IPv4Address`] or a [`IPv6Address`]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IPAddress {
    IPv4(IPv4Address),
    IPv6(IPv6Address),
}

/// A 32-bit Internet Protocol Version 4 address as specified in RFC791
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IPv4Address {
    pub(crate) address: u32,
}

impl IPv4Address {
    ///
    /// Creates a new IPv4Address from Big-Endian Bytes.
    ///
    /// # Example:
    /// ```
    /// # use irox_networking::address::IPv4Address;
    /// let addr = IPv4Address::from_be_bytes(&[127,0,0,1]);
    ///
    /// assert_eq!("127.0.0.1", format!("{}", addr));
    /// ```
    pub fn from_be_bytes(bytes: &[u8; 4]) -> IPv4Address {
        bytes.into()
    }
}

impl Display for IPv4Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let [a, b, c, d] = self.address.to_be_bytes();
        f.write_fmt(format_args!("{a}.{b}.{c}.{d}"))
    }
}

impl From<u32> for IPv4Address {
    fn from(value: u32) -> Self {
        IPv4Address { address: value }
    }
}

impl From<[u8; 4]> for IPv4Address {
    fn from(value: [u8; 4]) -> Self {
        let address = u32::from_be_bytes(value);
        IPv4Address { address }
    }
}

impl From<&[u8; 4]> for IPv4Address {
    fn from(value: &[u8; 4]) -> Self {
        let address = u32::from_be_bytes(*value);
        IPv4Address { address }
    }
}

///
/// An Internet Protocol Version 4 Network, an [`IPv4Address`] and a Netmask/CIDR.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IPv4Network {
    pub(crate) network_id: IPv4Address,
    pub(crate) network_mask: u32,
    pub(crate) host_mask: u32,
    pub(crate) cidr: u32,
}

impl IPv4Network {
    ///
    /// Creates a new [`IPv4Network`] with the specified CIDR number.
    ///
    /// # Example
    /// ```
    ///
    /// # use irox_networking::address::IPv4Network;
    /// # use irox_networking::address::{IPv4Address, NetworkError};
    ///  let addr = IPv4Address::from_be_bytes(&[127,0,0,0]);
    ///  let network = IPv4Network::from_cidr(addr, 16).unwrap();
    ///
    ///  assert_eq!("127.0.0.0/16", format!("{network}"));
    ///
    /// ```
    ///
    pub fn from_cidr(network_id: IPv4Address, cidr: u8) -> Result<IPv4Network, NetworkError> {
        if cidr > 32 {
            return Err(NetworkError::InvalidCIDR(cidr));
        }
        let host_mask: u32 = (1 << (32 - cidr)) - 1;
        let network_mask = !host_mask;
        if host_mask & network_id.address > 0 {
            return Err(NetworkError::NotANetworkAddress(IPAddress::IPv4(
                network_id,
            )));
        }
        let cidr = network_mask.leading_ones();
        Ok(IPv4Network {
            network_id,
            network_mask,
            host_mask,
            cidr,
        })
    }

    ///
    /// Creates a new [`IPv4Network`] from a specified power-of-two count of network addresses.  This
    /// is semantically equivalent to a CIDR, using `2^(32-cidr)`.  For a `/24` network, use `256`.
    ///
    /// # Example
    /// ```
    /// # use irox_networking::address::IPv4Network;
    /// # use irox_networking::address::{IPv4Address, NetworkError};
    ///  let addr = IPv4Address::from_be_bytes(&[127,0,0,0]);
    ///  let network = IPv4Network::from_address_count(addr, 256).unwrap();
    ///
    ///  assert_eq!("127.0.0.0/24", format!("{network}"));
    /// ```

    pub fn from_address_count(
        network_id: IPv4Address,
        address_count: u32,
    ) -> Result<IPv4Network, NetworkError> {
        if !address_count.is_power_of_two() {
            return Err(NetworkError::NotAPowerOfTwo(address_count));
        }
        let host_mask: u32 = address_count - 1;
        let network_mask = !host_mask;
        if host_mask & network_id.address > 0 {
            return Err(NetworkError::NotANetworkAddress(IPAddress::IPv4(
                network_id,
            )));
        }
        let cidr = network_mask.leading_ones();
        Ok(IPv4Network {
            network_id,
            host_mask,
            network_mask,
            cidr,
        })
    }

    ///
    /// Creates a new [`IPv4Network`] using the specified network ID and network Mask.
    ///
    /// # Example
    /// ```
    ///
    /// # use irox_networking::address::IPv4Network;
    /// # use irox_networking::address::{IPv4Address, NetworkError};
    ///  let addr = IPv4Address::from_be_bytes(&[127,0,0,0]);
    ///  let network = IPv4Network::from_network_mask(addr, 0xFFFFFF00).unwrap();
    ///
    ///  assert_eq!("127.0.0.0/24", format!("{network}"));
    ///
    /// ```
    pub fn from_network_mask(
        network_id: IPv4Address,
        network_mask: u32,
    ) -> Result<IPv4Network, NetworkError> {
        println!(
            "{}/{}",
            network_mask.leading_ones(),
            network_mask.trailing_zeros()
        );
        if network_mask.leading_ones() + network_mask.trailing_zeros() != 32 {
            return Err(NetworkError::InvalidMask(network_mask));
        }
        let host_mask = !network_mask;
        if host_mask & network_id.address > 0 {
            return Err(NetworkError::NotANetworkAddress(IPAddress::IPv4(
                network_id,
            )));
        }
        let cidr = network_mask.leading_ones();
        Ok(IPv4Network {
            network_id,
            network_mask,
            host_mask,
            cidr,
        })
    }

    ///
    /// Creates an [`IPv4Network`] from a network ID and host mask.  A host mask is the inverted form
    /// of a network mask.  If a `/24` is represented by `0xFFFFFF00`, then the equivalent host mask
    /// is `0x000000FF`
    ///
    /// # Example
    /// ```
    ///
    /// # use irox_networking::address::IPv4Network;
    /// # use irox_networking::address::{IPv4Address, NetworkError};
    ///  let addr = IPv4Address::from_be_bytes(&[127,0,0,0]);
    ///  let network = IPv4Network::from_host_mask(addr, 0x000000FF).unwrap();
    ///
    ///  assert_eq!("127.0.0.0/24", format!("{network}"));
    ///
    /// ```
    pub fn from_host_mask(
        network_id: IPv4Address,
        host_mask: u32,
    ) -> Result<IPv4Network, NetworkError> {
        if host_mask.leading_zeros() + host_mask.trailing_ones() != 32 {
            return Err(NetworkError::InvalidMask(host_mask));
        }
        let network_mask = !host_mask;
        if host_mask & network_id.address > 0 {
            return Err(NetworkError::NotANetworkAddress(IPAddress::IPv4(
                network_id,
            )));
        }
        let cidr = network_mask.leading_ones();
        Ok(IPv4Network {
            network_id,
            network_mask,
            host_mask,
            cidr,
        })
    }

    ///
    /// Creates a [`IPv4Network`] using the specified IPv4 Network ID and the specified CIDR.
    ///
    /// # Example
    /// ```
    /// # use irox_networking::address::IPv4Network;
    /// let network = IPv4Network::from_net_and_cidr(&[127,0,0,0], 24).unwrap();
    ///
    /// assert_eq!("127.0.0.0/24", format!("{network}"));
    /// ```
    pub fn from_net_and_cidr(network_id: &[u8; 4], cidr: u8) -> Result<IPv4Network, NetworkError> {
        let network_id: IPv4Address = network_id.into();
        Self::from_cidr(network_id, cidr)
    }

    ///
    /// Returns true if the specified address is the network address for this network.
    ///
    /// # Example
    /// ```
    /// # use irox_networking::address::{IPv4Address, IPv4Network};
    /// let net_addr = IPv4Address::from(&[127,0,0,0]);
    /// let host_addr = IPv4Address::from(&[127,0,0,1]);
    /// let network = IPv4Network::from_cidr(net_addr, 24).unwrap();
    ///
    /// assert_eq!(true, network.is_network_address(net_addr));
    /// assert_eq!(false, network.is_network_address(host_addr));
    /// ```
    pub fn is_network_address(&self, address: IPv4Address) -> bool {
        address == self.network_id
    }

    ///
    /// Returns true if the specified address is the broadcast address for this network.
    ///
    /// # Example
    /// ```
    /// # use irox_networking::address::{IPv4Address, IPv4Network};
    /// let net_addr = IPv4Address::from(&[127,0,0,0]);
    /// let broadcast = IPv4Address::from(&[127,0,0,255]);
    /// let network = IPv4Network::from_cidr(net_addr, 24).unwrap();
    ///
    /// assert_eq!(true, network.is_broadcast_address(broadcast));
    /// assert_eq!(false, network.is_broadcast_address(net_addr));
    /// ```
    pub fn is_broadcast_address(&self, address: IPv4Address) -> bool {
        address.address & self.host_mask == self.host_mask
    }

    ///
    /// Returns true if this address represents a private address range, in
    /// `10.0.0.0/8` or `172.16.0.0/12` or `192.168.0.0/16`
    ///
    /// # Example
    /// ```
    /// # use irox_networking::address::IPv4Network;
    /// let home_network = IPv4Network::from_net_and_cidr(&[192,168,0,0], 24).unwrap();
    /// assert_eq!(true, home_network.is_private_address());
    ///
    /// let enterprise_network = IPv4Network::from_net_and_cidr(&[10,10,0,0], 16).unwrap();
    /// assert_eq!(true, enterprise_network.is_private_address());
    ///
    /// let hotel_network = IPv4Network::from_net_and_cidr(&[172,20,0,0], 14).unwrap();
    /// assert_eq!(true, hotel_network.is_private_address());
    ///
    /// let quad_eight = IPv4Network::from_net_and_cidr(&[8,8,8,8], 32).unwrap();
    /// assert_eq!(false, quad_eight.is_private_address());
    /// ```
    pub fn is_private_address(&self) -> bool {
        let net = self.network_id.address;
        net & 0xFF000000 == 0x0A000000
            || net & 0xFFF00000 == 0xAC100000
            || net & 0xFFFF0000 == 0xC0A80000
    }

    ///
    /// Returns true if this network address represents a link-local address, in `169.254.0.0/16`
    ///
    /// # Example
    /// ```
    /// # use irox_networking::address::IPv4Network;
    /// let link_local = IPv4Network::from_net_and_cidr(&[169,254,55,228], 32).unwrap();
    /// assert_eq!(true, link_local.is_link_local());
    ///
    /// let quad_eight = IPv4Network::from_net_and_cidr(&[8,8,8,8], 32).unwrap();
    /// assert_eq!(false, quad_eight.is_link_local());
    /// ```
    pub fn is_link_local(&self) -> bool {
        let net = self.network_id.address;
        net & 0xFFFF0000 == 0xA9FE0000
    }

    ///
    /// Returns true if this network address represents a loopback address, in `127.0.0.0/8`
    ///
    /// # Example
    /// ```
    /// # use irox_networking::address::IPv4Network;
    /// let loopback = IPv4Network::from_net_and_cidr(&[127,0,0,53], 32).unwrap();
    /// assert_eq!(true, loopback.is_loopback());
    ///
    /// let quad_eight = IPv4Network::from_net_and_cidr(&[8,8,8,8], 32).unwrap();
    /// assert_eq!(false, quad_eight.is_loopback());
    /// ```
    pub fn is_loopback(&self) -> bool {
        let net = self.network_id.address;
        net & 0xFF000000 == 0x7F000000
    }

    ///
    /// Returns true if this network represents a carrier-grade NAT address, in `100.64.0.0/10`
    ///
    /// # Example
    /// ```
    /// # use irox_networking::address::IPv4Network;
    /// let carrier_nat = IPv4Network::from_net_and_cidr(&[100,80,0,0], 12).unwrap();
    /// assert_eq!(true, carrier_nat.is_shared_carrier_nat());
    ///
    /// let quad_eight = IPv4Network::from_net_and_cidr(&[8,8,8,8], 32).unwrap();
    /// assert_eq!(false, quad_eight.is_shared_carrier_nat());
    /// ```
    pub fn is_shared_carrier_nat(&self) -> bool {
        let net = self.network_id.address;
        net & 0xFFC00000 == 0x64400000
    }

    pub fn host_in_network(&self, host: IPv4Address) -> bool {
        let lower = host.address & self.host_mask;
        let net = host.address & self.network_mask;
        lower > 0 && lower != self.host_mask && self.network_id.address == net
    }

    pub fn get_network_address(&self) -> IPv4Address {
        self.network_id
    }

    pub fn get_broadcast_address(&self) -> IPv4Address {
        IPv4Address {
            address: self.network_id.address | self.host_mask,
        }
    }
}

impl Display for IPv4Network {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}/{}", self.network_id, self.cidr))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IPv6Network {
    pub(crate) network_id: u128,
    pub(crate) network_mask: u128,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IPv6Address {
    pub(crate) address: u128
}
impl IPv6Address {
    ///
    /// # Example
    /// ```
    /// # use irox_networking::address::IPv6Address;
    /// let addr = IPv6Address::new(&[0x2001,0x0DB8,0x85A3,0x0000,0x0000,0x8A2E,0x0370,0x7334]);
    ///
    /// assert_eq!("2001:db8:85a3::8a2e:370:7334", format!("{addr}"));
    /// assert_eq!("2001:0db8:85a3:0000:0000:8a2e:0370:7334", format!("{addr:#}"));
    ///
    /// assert_eq!("::", format!("{}", IPv6Address::new(&[0,0,0,0,0,0,0,0])));
    /// assert_eq!("::1", format!("{}", IPv6Address::new(&[0,0,0,0,0,0,0,1])));
    ///
    /// ```
    pub fn new(val: &[u16;8]) -> IPv6Address{
        let address = u128::from_u16_array(val);
        IPv6Address {
            address
        }
    }
}
impl Display for IPv6Address {

    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let bytes = self.address.to_u16_array();
        if fmt.alternate() {
            // full form, no collapse.
            let [a,b,c,d,e,f,g,h] = bytes;
            return fmt.write_fmt(format_args!("{a:04x}:{b:04x}:{c:04x}:{d:04x}:{e:04x}:{f:04x}:{g:04x}:{h:04x}"));
        }
        // collapsed form.
        let mut zeroes : [u8;8] = [0;8];
        for i in 0..8 {
            for j in i..8 {
                let val = bytes[j];
                if val == 0 {
                    zeroes[i] += 1;
                } else {
                    break;
                }
            }
        }
        let longest_zeroes_point = max_index(&zeroes);
        if let Some(longest_zeroes_point) = longest_zeroes_point {
            if let Some(num_zeroes) = zeroes.get(longest_zeroes_point) {
                if longest_zeroes_point == 0 && *num_zeroes == 8 {
                    return fmt.write_str("::");
                }
                if *num_zeroes > 1 {
                    let bytes : Vec<String> = bytes.iter().enumerate().filter_map(|(idx,val)| {
                        if idx == longest_zeroes_point || (idx == 1 && longest_zeroes_point == 0){
                            return Some(String::new());
                        } else if idx > longest_zeroes_point && idx < (longest_zeroes_point + *num_zeroes as usize) {
                            return None;
                        }
                        Some(format!("{val:x}"))
                    }).collect();
                    if bytes.is_empty() {
                        return fmt.write_str("::");
                    }
                    fmt.write_fmt(format_args!("{}", bytes.join(":")))?;
                    return Ok(());
                }
            }
        }
        let [a,b,c,d,e,f,g,h] = bytes;
        fmt.write_fmt(format_args!("{a:x}:{b:x}:{c:x}:{d:x}:{e:x}:{f:x}:{g:x}:{h:x}"))
    }
}