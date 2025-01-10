use std::net::SocketAddr;
use bitcoin::network::{message::NetworkMessage, message_network::VersionMessage, constants::ServiceFlags, Address, Magic };
use chrono::Utc;
use rand::random;
use bitcoin::network::message::RawNetworkMessage;
use crate::consts::{MAGIC_MAINNET, MAGIC_TESTNET};
use bitcoin::consensus::encode::serialize;
use crate::config::CliNetwork;

pub fn get_version_message(local_address: SocketAddr, remote_address: SocketAddr, network: &CliNetwork) -> Vec<u8> {
    let network_message = NetworkMessage::Version(
        VersionMessage::new(
            ServiceFlags::NONE,
            Utc::now().timestamp(),
            Address::new(&remote_address, ServiceFlags::NONE),
            Address::new(&local_address, ServiceFlags::NONE),
            random(),
            "handshake-node".to_string(),
            0,
        )
    );

    let magic_number = match network {
        CliNetwork::Mainnet => MAGIC_MAINNET,
        CliNetwork::Testnet => MAGIC_TESTNET,
    };

    let version_message = RawNetworkMessage {
        magic: Magic::from_bytes(magic_number.to_le_bytes()),
        payload: network_message,
    };

    serialize(&version_message)
}

pub fn get_verack_message(network: &CliNetwork) -> Vec<u8> {
    let network_message = NetworkMessage::Verack;

    let magic_number = match network {
        CliNetwork::Mainnet => MAGIC_MAINNET,
        CliNetwork::Testnet => MAGIC_TESTNET,
    };

    let verack_message = RawNetworkMessage {
        magic: Magic::from_bytes(magic_number.to_le_bytes()),
        payload: network_message,
    };

    serialize(&verack_message)
}