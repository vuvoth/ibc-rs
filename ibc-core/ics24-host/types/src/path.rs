//! Defines all store paths used by IBC

/// Path-space as listed in ICS-024
/// https://github.com/cosmos/ibc/tree/master/spec/core/ics-024-host-requirements#path-space
/// Some of these are implemented in other ICSs, but ICS-024 has a nice summary table.
///
use core::str::FromStr;

use derive_more::{Display, From};
use ibc_primitives::prelude::*;

use crate::identifiers::{ChannelId, ClientId, ConnectionId, PortId, Sequence};

/// ABCI client upgrade keys
/// - The key identifying the upgraded IBC state within the upgrade sub-store
const UPGRADED_IBC_STATE: &str = "upgradedIBCState";
///- The key identifying the upgraded client state
const UPGRADED_CLIENT_STATE: &str = "upgradedClient";
/// - The key identifying the upgraded consensus state
const UPGRADED_CLIENT_CONSENSUS_STATE: &str = "upgradedConsState";

/// The Path enum abstracts out the different sub-paths.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Display)]
pub enum Path {
    NextClientSequence(NextClientSequencePath),
    NextConnectionSequence(NextConnectionSequencePath),
    NextChannelSequence(NextChannelSequencePath),
    ClientState(ClientStatePath),
    ClientConsensusState(ClientConsensusStatePath),
    ClientUpdateTime(ClientUpdateTimePath),
    ClientUpdateHeight(ClientUpdateHeightPath),
    ClientConnection(ClientConnectionPath),
    Connection(ConnectionPath),
    Ports(PortPath),
    ChannelEnd(ChannelEndPath),
    SeqSend(SeqSendPath),
    SeqRecv(SeqRecvPath),
    SeqAck(SeqAckPath),
    Commitment(CommitmentPath),
    Ack(AckPath),
    Receipt(ReceiptPath),
    UpgradeClient(UpgradeClientPath),
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(fmt = "nextClientSequence")]
pub struct NextClientSequencePath;

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(fmt = "nextConnectionSequence")]
pub struct NextConnectionSequencePath;

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(fmt = "nextChannelSequence")]
pub struct NextChannelSequencePath;

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(fmt = "clients/{_0}/clientState")]
pub struct ClientStatePath(pub ClientId);

impl ClientStatePath {
    pub fn new(client_id: &ClientId) -> ClientStatePath {
        ClientStatePath(client_id.clone())
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(fmt = "clients/{client_id}/consensusStates/{revision_number}-{revision_height}")]
pub struct ClientConsensusStatePath {
    pub client_id: ClientId,
    pub revision_number: u64,
    pub revision_height: u64,
}

impl ClientConsensusStatePath {
    pub fn new(
        client_id: ClientId,
        revision_number: u64,
        revision_height: u64,
    ) -> ClientConsensusStatePath {
        ClientConsensusStatePath {
            client_id,
            revision_number,
            revision_height,
        }
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(
    fmt = "clients/{client_id}/consensusStates/{revision_number}-{revision_height}/processedTime"
)]
pub struct ClientUpdateTimePath {
    pub client_id: ClientId,
    pub revision_number: u64,
    pub revision_height: u64,
}

impl ClientUpdateTimePath {
    pub fn new(client_id: ClientId, revision_number: u64, revision_height: u64) -> Self {
        Self {
            client_id,
            revision_number,
            revision_height,
        }
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(
    fmt = "clients/{client_id}/consensusStates/{revision_number}-{revision_height}/processedHeight"
)]
pub struct ClientUpdateHeightPath {
    pub client_id: ClientId,
    pub revision_number: u64,
    pub revision_height: u64,
}

impl ClientUpdateHeightPath {
    pub fn new(client_id: ClientId, revision_number: u64, revision_height: u64) -> Self {
        Self {
            client_id,
            revision_number,
            revision_height,
        }
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(fmt = "clients/{_0}/connections")]
pub struct ClientConnectionPath(pub ClientId);

impl ClientConnectionPath {
    pub fn new(client_id: &ClientId) -> ClientConnectionPath {
        ClientConnectionPath(client_id.clone())
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(fmt = "connections/{_0}")]
pub struct ConnectionPath(pub ConnectionId);

impl ConnectionPath {
    pub fn new(connection_id: &ConnectionId) -> ConnectionPath {
        ConnectionPath(connection_id.clone())
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(fmt = "ports/{_0}")]
pub struct PortPath(pub PortId);

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(fmt = "channelEnds/ports/{_0}/channels/{_1}")]
pub struct ChannelEndPath(pub PortId, pub ChannelId);

impl ChannelEndPath {
    pub fn new(port_id: &PortId, channel_id: &ChannelId) -> ChannelEndPath {
        ChannelEndPath(port_id.clone(), channel_id.clone())
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(fmt = "nextSequenceSend/ports/{_0}/channels/{_1}")]
pub struct SeqSendPath(pub PortId, pub ChannelId);

impl SeqSendPath {
    pub fn new(port_id: &PortId, channel_id: &ChannelId) -> SeqSendPath {
        SeqSendPath(port_id.clone(), channel_id.clone())
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(fmt = "nextSequenceRecv/ports/{_0}/channels/{_1}")]
pub struct SeqRecvPath(pub PortId, pub ChannelId);

impl SeqRecvPath {
    pub fn new(port_id: &PortId, channel_id: &ChannelId) -> SeqRecvPath {
        SeqRecvPath(port_id.clone(), channel_id.clone())
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(fmt = "nextSequenceAck/ports/{_0}/channels/{_1}")]
pub struct SeqAckPath(pub PortId, pub ChannelId);

impl SeqAckPath {
    pub fn new(port_id: &PortId, channel_id: &ChannelId) -> SeqAckPath {
        SeqAckPath(port_id.clone(), channel_id.clone())
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(fmt = "commitments/ports/{port_id}/channels/{channel_id}/sequences/{sequence}")]
pub struct CommitmentPath {
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub sequence: Sequence,
}

impl CommitmentPath {
    pub fn new(port_id: &PortId, channel_id: &ChannelId, sequence: Sequence) -> CommitmentPath {
        CommitmentPath {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            sequence,
        }
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(fmt = "acks/ports/{port_id}/channels/{channel_id}/sequences/{sequence}")]
pub struct AckPath {
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub sequence: Sequence,
}

impl AckPath {
    pub fn new(port_id: &PortId, channel_id: &ChannelId, sequence: Sequence) -> AckPath {
        AckPath {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            sequence,
        }
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display(fmt = "receipts/ports/{port_id}/channels/{channel_id}/sequences/{sequence}")]
pub struct ReceiptPath {
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub sequence: Sequence,
}

impl ReceiptPath {
    pub fn new(port_id: &PortId, channel_id: &ChannelId, sequence: Sequence) -> ReceiptPath {
        ReceiptPath {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            sequence,
        }
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Paths that are specific for client upgrades.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub enum UpgradeClientPath {
    #[display(fmt = "{UPGRADED_IBC_STATE}/{_0}/{UPGRADED_CLIENT_STATE}")]
    UpgradedClientState(u64),
    #[display(fmt = "{UPGRADED_IBC_STATE}/{_0}/{UPGRADED_CLIENT_CONSENSUS_STATE}")]
    UpgradedClientConsensusState(u64),
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Sub-paths which are not part of the specification, but are still
/// useful to represent for parsing purposes.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum SubPath {
    Channels(ChannelId),
    Sequences(Sequence),
}

impl Path {
    /// Indication if the path is provable.
    pub fn is_provable(&self) -> bool {
        !matches!(&self, Path::ClientConnection(_) | Path::Ports(_))
    }

    /// into_bytes implementation
    pub fn into_bytes(self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

#[derive(Debug, displaydoc::Display)]
pub enum PathError {
    /// `{path}` could not be parsed into a Path
    ParseFailure { path: String },
}

#[cfg(feature = "std")]
impl std::error::Error for PathError {}

/// The FromStr trait allows paths encoded as strings to be parsed into Paths.
impl FromStr for Path {
    type Err = PathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<&str> = s.split('/').collect();

        parse_next_sequence(&components)
            .or_else(|| parse_client_paths(&components))
            .or_else(|| parse_connections(&components))
            .or_else(|| parse_ports(&components))
            .or_else(|| parse_channel_ends(&components))
            .or_else(|| parse_seqs(&components))
            .or_else(|| parse_commitments(&components))
            .or_else(|| parse_acks(&components))
            .or_else(|| parse_receipts(&components))
            .or_else(|| parse_upgrades(&components))
            .ok_or(PathError::ParseFailure {
                path: s.to_string(),
            })
    }
}

fn parse_next_sequence(components: &[&str]) -> Option<Path> {
    if components.len() != 1 {
        return None;
    }

    match *components.first()? {
        "nextClientSequence" => Some(NextClientSequencePath.into()),
        "nextConnectionSequence" => Some(NextConnectionSequencePath.into()),
        "nextChannelSequence" => Some(NextChannelSequencePath.into()),
        _ => None,
    }
}

fn parse_client_paths(components: &[&str]) -> Option<Path> {
    let first = match components.first() {
        Some(f) => *f,
        None => return None,
    };

    if first != "clients" {
        return None;
    }

    let client_id = match ClientId::from_str(components[1]) {
        Ok(s) => s,
        Err(_) => return None,
    };

    if components.len() == 3 {
        match components[2] {
            "clientState" => Some(ClientStatePath(client_id).into()),
            "connections" => Some(ClientConnectionPath(client_id).into()),
            _ => None,
        }
    } else if components.len() == 4 || components.len() == 5 {
        match components[2] {
            "consensusStates" => {}
            _ => return None,
        }

        let epoch_height: Vec<&str> = components[3].split('-').collect();

        if epoch_height.len() != 2 {
            return None;
        }

        let revision_number = epoch_height[0];
        let revision_height = epoch_height[1];

        let revision_number = match revision_number.parse::<u64>() {
            Ok(ep) => ep,
            Err(_) => return None,
        };

        let revision_height = match revision_height.parse::<u64>() {
            Ok(h) => h,
            Err(_) => return None,
        };

        match components.len() {
            4 => Some(
                ClientConsensusStatePath {
                    client_id,
                    revision_number,
                    revision_height,
                }
                .into(),
            ),
            5 => match components[4] {
                "processedTime" => Some(
                    ClientUpdateTimePath {
                        client_id,
                        revision_number,
                        revision_height,
                    }
                    .into(),
                ),
                "processedHeight" => Some(
                    ClientUpdateHeightPath {
                        client_id,
                        revision_number,
                        revision_height,
                    }
                    .into(),
                ),
                _ => None,
            },
            _ => None,
        }
    } else {
        None
    }
}

fn parse_connections(components: &[&str]) -> Option<Path> {
    if components.len() != 2 {
        return None;
    }

    let first = match components.first() {
        Some(f) => *f,
        None => return None,
    };

    if first != "connections" {
        return None;
    }

    let connection_id = match components.last() {
        Some(c) => *c,
        None => return None,
    };

    let connection_id = match ConnectionId::from_str(connection_id) {
        Ok(c) => c,
        Err(_) => return None,
    };

    Some(ConnectionPath(connection_id).into())
}

fn parse_ports(components: &[&str]) -> Option<Path> {
    if components.len() != 2 {
        return None;
    }

    let first = match components.first() {
        Some(f) => *f,
        None => return None,
    };

    if first != "ports" {
        return None;
    }

    let port_id = match components.last() {
        Some(p) => *p,
        None => return None,
    };

    let port_id = match PortId::from_str(port_id) {
        Ok(p) => p,
        Err(_) => return None,
    };

    Some(PortPath(port_id).into())
}

fn parse_channels(components: &[&str]) -> Option<SubPath> {
    if components.len() != 2 {
        return None;
    }

    let first = match components.first() {
        Some(f) => *f,
        None => return None,
    };

    if first != "channels" {
        return None;
    }

    let channel_id = match components.last() {
        Some(c) => *c,
        None => return None,
    };

    let channel_id = match ChannelId::from_str(channel_id) {
        Ok(c) => c,
        Err(_) => return None,
    };

    Some(SubPath::Channels(channel_id))
}

fn parse_sequences(components: &[&str]) -> Option<SubPath> {
    if components.len() != 2 {
        return None;
    }

    let first = match components.first() {
        Some(f) => *f,
        None => return None,
    };

    if first != "sequences" {
        return None;
    }

    let sequence_number = match components.last() {
        Some(s) => *s,
        None => return None,
    };

    match Sequence::from_str(sequence_number) {
        Ok(seq) => Some(SubPath::Sequences(seq)),
        Err(_) => None,
    }
}

fn parse_channel_ends(components: &[&str]) -> Option<Path> {
    if components.len() != 5 {
        return None;
    }

    let first = match components.first() {
        Some(f) => *f,
        None => return None,
    };

    if first != "channelEnds" {
        return None;
    }

    let port = parse_ports(&components[1..=2]);
    let channel = parse_channels(&components[3..=4]);

    let port_id = if let Some(Path::Ports(PortPath(port_id))) = port {
        port_id
    } else {
        return None;
    };

    let channel_id = if let Some(SubPath::Channels(channel_id)) = channel {
        channel_id
    } else {
        return None;
    };

    Some(ChannelEndPath(port_id, channel_id).into())
}

fn parse_seqs(components: &[&str]) -> Option<Path> {
    if components.len() != 5 {
        return None;
    }

    let first = match components.first() {
        Some(f) => *f,
        None => return None,
    };

    let port = parse_ports(&components[1..=2]);
    let channel = parse_channels(&components[3..=4]);

    let port_id = if let Some(Path::Ports(PortPath(port_id))) = port {
        port_id
    } else {
        return None;
    };

    let channel_id = if let Some(SubPath::Channels(channel_id)) = channel {
        channel_id
    } else {
        return None;
    };

    match first {
        "nextSequenceSend" => Some(SeqSendPath(port_id, channel_id).into()),
        "nextSequenceRecv" => Some(SeqRecvPath(port_id, channel_id).into()),
        "nextSequenceAck" => Some(SeqAckPath(port_id, channel_id).into()),
        _ => None,
    }
}

fn parse_commitments(components: &[&str]) -> Option<Path> {
    if components.len() != 7 {
        return None;
    }

    let first = match components.first() {
        Some(f) => *f,
        None => return None,
    };

    if first != "commitments" {
        return None;
    }

    let port = parse_ports(&components[1..=2]);
    let channel = parse_channels(&components[3..=4]);
    let sequence = parse_sequences(&components[5..]);

    let port_id = if let Some(Path::Ports(PortPath(port_id))) = port {
        port_id
    } else {
        return None;
    };

    let channel_id = if let Some(SubPath::Channels(channel_id)) = channel {
        channel_id
    } else {
        return None;
    };

    let sequence = if let Some(SubPath::Sequences(seq)) = sequence {
        seq
    } else {
        return None;
    };

    Some(
        CommitmentPath {
            port_id,
            channel_id,
            sequence,
        }
        .into(),
    )
}

fn parse_acks(components: &[&str]) -> Option<Path> {
    if components.len() != 7 {
        return None;
    }

    let first = match components.first() {
        Some(f) => *f,
        None => return None,
    };

    if first != "acks" {
        return None;
    }

    let port = parse_ports(&components[1..=2]);
    let channel = parse_channels(&components[3..=4]);
    let sequence = parse_sequences(&components[5..]);

    let port_id = if let Some(Path::Ports(PortPath(port_id))) = port {
        port_id
    } else {
        return None;
    };

    let channel_id = if let Some(SubPath::Channels(channel_id)) = channel {
        channel_id
    } else {
        return None;
    };

    let sequence = if let Some(SubPath::Sequences(seq)) = sequence {
        seq
    } else {
        return None;
    };

    Some(
        AckPath {
            port_id,
            channel_id,
            sequence,
        }
        .into(),
    )
}

fn parse_receipts(components: &[&str]) -> Option<Path> {
    if components.len() != 7 {
        return None;
    }

    let first = match components.first() {
        Some(f) => *f,
        None => return None,
    };

    if first != "receipts" {
        return None;
    }

    let port = parse_ports(&components[1..=2]);
    let channel = parse_channels(&components[3..=4]);
    let sequence = parse_sequences(&components[5..]);

    let port_id = if let Some(Path::Ports(PortPath(port_id))) = port {
        port_id
    } else {
        return None;
    };

    let channel_id = if let Some(SubPath::Channels(channel_id)) = channel {
        channel_id
    } else {
        return None;
    };

    let sequence = if let Some(SubPath::Sequences(seq)) = sequence {
        seq
    } else {
        return None;
    };

    Some(
        ReceiptPath {
            port_id,
            channel_id,
            sequence,
        }
        .into(),
    )
}

fn parse_upgrades(components: &[&str]) -> Option<Path> {
    if components.len() != 3 {
        return None;
    }

    let first = match components.first() {
        Some(f) => *f,
        None => return None,
    };

    if first != UPGRADED_IBC_STATE {
        return None;
    }

    let last = match components.last() {
        Some(l) => *l,
        None => return None,
    };

    let height = match components[1].parse::<u64>() {
        Ok(h) => h,
        Err(_) => return None,
    };

    match last {
        UPGRADED_CLIENT_STATE => Some(UpgradeClientPath::UpgradedClientState(height).into()),
        UPGRADED_CLIENT_CONSENSUS_STATE => {
            Some(UpgradeClientPath::UpgradedClientConsensusState(height).into())
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use super::*;

    #[rstest::rstest]
    #[case("nextClientSequence", Path::NextClientSequence(NextClientSequencePath))]
    #[case(
        "nextConnectionSequence",
        Path::NextConnectionSequence(NextConnectionSequencePath)
    )]
    #[case(
        "nextChannelSequence",
        Path::NextChannelSequence(NextChannelSequencePath)
    )]
    #[case(
        "clients/07-tendermint-0/clientState",
        Path::ClientState(ClientStatePath(ClientId::default()))
    )]
    #[case(
        "clients/07-tendermint-0/consensusStates/15-31",
        Path::ClientConsensusState(ClientConsensusStatePath {
            client_id: ClientId::default(),
            revision_number: 15,
            revision_height: 31,
        })
    )]
    #[case(
        "clients/07-tendermint-0/consensusStates/15-31/processedTime",
        Path::ClientUpdateTime(ClientUpdateTimePath {
            client_id: ClientId::default(),
            revision_number: 15,
            revision_height: 31,
        })
    )]
    #[case(
        "clients/07-tendermint-0/consensusStates/15-31/processedHeight",
        Path::ClientUpdateHeight(ClientUpdateHeightPath {
            client_id: ClientId::default(),
            revision_number: 15,
            revision_height: 31,
        })
    )]
    #[case(
        "clients/07-tendermint-0/connections",
        Path::ClientConnection(ClientConnectionPath(ClientId::default()))
    )]
    #[case(
        "connections/connection-0",
        Path::Connection(ConnectionPath(ConnectionId::new(0)))
    )]
    #[case("ports/transfer", Path::Ports(PortPath(PortId::transfer())))]
    #[case(
        "channelEnds/ports/transfer/channels/channel-0",
        Path::ChannelEnd(ChannelEndPath(PortId::transfer(), ChannelId::default()))
    )]
    #[case(
        "nextSequenceSend/ports/transfer/channels/channel-0",
        Path::SeqSend(SeqSendPath(PortId::transfer(), ChannelId::default()))
    )]
    #[case(
        "nextSequenceRecv/ports/transfer/channels/channel-0",
        Path::SeqRecv(SeqRecvPath(PortId::transfer(), ChannelId::default()))
    )]
    #[case(
        "nextSequenceAck/ports/transfer/channels/channel-0",
        Path::SeqAck(SeqAckPath(PortId::transfer(), ChannelId::default()))
    )]
    #[case(
        "commitments/ports/transfer/channels/channel-0/sequences/0",
        Path::Commitment(CommitmentPath {
            port_id: PortId::transfer(),
            channel_id: ChannelId::default(),
            sequence: Sequence::default(),
        })
    )]
    #[case(
        "acks/ports/transfer/channels/channel-0/sequences/0",
        Path::Ack(AckPath {
            port_id: PortId::transfer(),
            channel_id: ChannelId::default(),
            sequence: Sequence::default(),
        })
    )]
    #[case(
        "receipts/ports/transfer/channels/channel-0/sequences/0",
        Path::Receipt(ReceiptPath {
            port_id: PortId::transfer(),
            channel_id: ChannelId::default(),
            sequence: Sequence::default(),
        })
    )]
    #[case(
        "upgradedIBCState/0/upgradedClient",
        Path::UpgradeClient(UpgradeClientPath::UpgradedClientState(0))
    )]
    #[case(
        "upgradedIBCState/0/upgradedConsState",
        Path::UpgradeClient(UpgradeClientPath::UpgradedClientConsensusState(0))
    )]
    fn test_successful_parsing(#[case] path_str: &str, #[case] path: Path) {
        // can be parsed into Path
        assert_eq!(Path::from_str(path_str).expect("no error"), path);
        // can be converted back to string
        assert_eq!(path_str, path.to_string());
    }

    #[rstest::rstest]
    #[case("clients/clientType")]
    #[case("channels/channel-0")]
    #[case("sequences/0")]
    fn test_failure_parsing(#[case] path_str: &str) {
        // cannot be parsed into Path
        assert!(Path::from_str(path_str).is_err());
    }

    #[test]
    fn test_parse_client_paths_fn() {
        let path = "clients/07-tendermint-0/clientState";
        let components: Vec<&str> = path.split('/').collect();

        assert_eq!(
            parse_client_paths(&components),
            Some(Path::ClientState(ClientStatePath(ClientId::default())))
        );

        let path = "clients/07-tendermint-0/consensusStates/15-31";
        let components: Vec<&str> = path.split('/').collect();

        assert_eq!(
            parse_client_paths(&components),
            Some(Path::ClientConsensusState(ClientConsensusStatePath {
                client_id: ClientId::default(),
                revision_number: 15,
                revision_height: 31,
            }))
        );
    }

    #[test]
    fn test_parse_client_update_paths_fn() {
        let path = "clients/07-tendermint-0/consensusStates/15-31/processedTime";
        let components: Vec<&str> = path.split('/').collect();

        assert_eq!(
            parse_client_paths(&components),
            Some(Path::ClientUpdateTime(ClientUpdateTimePath {
                client_id: ClientId::default(),
                revision_number: 15,
                revision_height: 31,
            }))
        );

        let path = "clients/07-tendermint-0/consensusStates/15-31/processedHeight";
        let components: Vec<&str> = path.split('/').collect();

        assert_eq!(
            parse_client_paths(&components),
            Some(Path::ClientUpdateHeight(ClientUpdateHeightPath {
                client_id: ClientId::default(),
                revision_number: 15,
                revision_height: 31,
            }))
        );
    }

    #[test]
    fn test_parse_connections_fn() {
        let path = "connections/connection-0";
        let components: Vec<&str> = path.split('/').collect();

        assert_eq!(
            parse_connections(&components),
            Some(Path::Connection(ConnectionPath(ConnectionId::new(0)))),
        );
    }

    #[test]
    fn test_parse_ports_fn() {
        let path = "ports/transfer";
        let components: Vec<&str> = path.split('/').collect();

        assert_eq!(
            parse_ports(&components),
            Some(Path::Ports(PortPath(PortId::transfer()))),
        );
    }

    #[test]
    fn test_parse_channels_fn() {
        let path = "channels/channel-0";
        let components: Vec<&str> = path.split('/').collect();

        assert_eq!(
            parse_channels(&components),
            Some(SubPath::Channels(ChannelId::default())),
        );
    }

    #[test]
    fn test_parse_sequences_fn() {
        let path = "sequences/0";
        let components: Vec<&str> = path.split('/').collect();

        assert_eq!(
            parse_sequences(&components),
            Some(SubPath::Sequences(Sequence::default()))
        );
    }

    #[test]
    fn test_parse_channel_ends_fn() {
        let path = "channelEnds/ports/transfer/channels/channel-0";
        let components: Vec<&str> = path.split('/').collect();

        assert_eq!(
            parse_channel_ends(&components),
            Some(Path::ChannelEnd(ChannelEndPath(
                PortId::transfer(),
                ChannelId::default()
            ))),
        );
    }

    #[test]
    fn test_parse_seqs_fn() {
        let path = "nextSequenceSend/ports/transfer/channels/channel-0";
        let components: Vec<&str> = path.split('/').collect();

        assert_eq!(
            parse_seqs(&components),
            Some(Path::SeqSend(SeqSendPath(
                PortId::transfer(),
                ChannelId::default()
            ))),
        );

        let path = "nextSequenceRecv/ports/transfer/channels/channel-0";
        let components: Vec<&str> = path.split('/').collect();

        assert_eq!(
            parse_seqs(&components),
            Some(Path::SeqRecv(SeqRecvPath(
                PortId::transfer(),
                ChannelId::default()
            ))),
        );

        let path = "nextSequenceAck/ports/transfer/channels/channel-0";
        let components: Vec<&str> = path.split('/').collect();

        assert_eq!(
            parse_seqs(&components),
            Some(Path::SeqAck(SeqAckPath(
                PortId::transfer(),
                ChannelId::default()
            ))),
        );
    }

    #[test]
    fn test_parse_commitments_fn() {
        let path = "commitments/ports/transfer/channels/channel-0/sequences/0";
        let components: Vec<&str> = path.split('/').collect();

        assert_eq!(
            parse_commitments(&components),
            Some(Path::Commitment(CommitmentPath {
                port_id: PortId::transfer(),
                channel_id: ChannelId::default(),
                sequence: Sequence::default(),
            })),
        );
    }

    #[test]
    fn test_parse_acks_fn() {
        let path = "acks/ports/transfer/channels/channel-0/sequences/0";
        let components: Vec<&str> = path.split('/').collect();

        assert_eq!(
            parse_acks(&components),
            Some(Path::Ack(AckPath {
                port_id: PortId::transfer(),
                channel_id: ChannelId::default(),
                sequence: Sequence::default(),
            })),
        );
    }

    #[test]
    fn test_parse_receipts_fn() {
        let path = "receipts/ports/transfer/channels/channel-0/sequences/0";
        let components: Vec<&str> = path.split('/').collect();

        assert_eq!(
            parse_receipts(&components),
            Some(Path::Receipt(ReceiptPath {
                port_id: PortId::transfer(),
                channel_id: ChannelId::default(),
                sequence: Sequence::default(),
            })),
        );
    }

    #[test]
    fn test_parse_upgrades_fn() {
        let path = "upgradedIBCState/0/upgradedClient";
        let components: Vec<&str> = path.split('/').collect();

        assert_eq!(
            parse_upgrades(&components),
            Some(Path::UpgradeClient(UpgradeClientPath::UpgradedClientState(
                0
            ))),
        );

        let path = "upgradedIBCState/0/upgradedConsState";
        let components: Vec<&str> = path.split('/').collect();

        assert_eq!(
            parse_upgrades(&components),
            Some(Path::UpgradeClient(
                UpgradeClientPath::UpgradedClientConsensusState(0)
            )),
        )
    }
}