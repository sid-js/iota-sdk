// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::client::error::{Error, Result};
use core::convert::TryFrom;
use dict_derive::{FromPyObject as DeriveFromPyObject, IntoPyObject as DeriveIntoPyObject};
use iota::{
    bee_rest_api::{
        handlers::{
            balance_ed25519::BalanceForAddressResponse as RustBalanceForAddressResponse,
            info::InfoResponse as RustInfoResponse,
            message_metadata::{
                LedgerInclusionStateDto as RustLedgerInclusionStateDto,
                MessageMetadataResponse as RustMessageMetadataResponse,
            },
            output::OutputResponse as RustOutputResponse,
        },
        types::{
            AddressDto as RustAddressDto, Ed25519AddressDto as RustEd25519AddressDto, MilestoneDto as RustMilestoneDto,
            OutputDto as RustOutputDto, SignatureLockedSingleOutputDto as RustSignatureLockedSingleOutputDto,
        },
    },
    builder::NetworkInfo as RustNetworkInfo,
    Address as RustAddress, Ed25519Address as RustEd25519Address, Ed25519Signature as RustEd25519Signature,
    IndexationPayload as RustIndexationPayload, Input as RustInput, Message as RustMessage,
    MilestonePayloadEssence as RustMilestonePayloadEssence, Output as RustOutput, Payload as RustPayload,
    ReferenceUnlock as RustReferenceUnlock, SignatureLockedSingleOutput as RustSignatureLockedSingleOutput,
    SignatureUnlock as RustSignatureUnlock, TransactionId as RustTransationId,
    TransactionPayload as RustTransactionPayload, TransactionPayloadEssence as RustTransactionPayloadEssence,
    UTXOInput as RustUTXOInput, UnlockBlock as RustUnlockBlock,
};

use std::{
    convert::{From, Into, TryInto},
    str::FromStr,
};
pub const MILESTONE_MERKLE_PROOF_LENGTH: usize = 32;
pub const MILESTONE_PUBLIC_KEY_LENGTH: usize = 32;
pub static mut BECH32_HRP: &str = "atoi1";

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct MessageMetadataResponse {
    /// Message ID
    pub message_id: String,
    /// Message ID of parent_1
    pub parent_1_message_id: String,
    /// Message ID of parent_2
    pub parent_2_message_id: String,
    /// Solid status
    pub is_solid: bool,
    pub referenced_by_milestone_index: Option<u32>,
    pub milestone_index: Option<u32>,
    pub ledger_inclusion_state: Option<LedgerInclusionStateDto>,
    pub conflict_reason: Option<u8>,
    pub should_promote: Option<bool>,
    pub should_reattach: Option<bool>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct BalanceForAddressResponse {
    // The type of the address (1=Ed25519).
    pub address_type: u8,
    // hex encoded address
    pub address: String,
    pub max_results: usize,
    pub count: usize,
    pub balance: u64,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct AddressBalancePair {
    /// Address
    pub address: String,
    /// Balance in the address
    pub balance: u64,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct MilestoneDto {
    pub kind: u32,
    pub index: u32,
    pub timestamp: u64,
    pub parent_1_message_id: String,
    pub parent_2_message_id: String,
    pub inclusion_merkle_proof: String,
    pub public_keys: Vec<String>,
    pub signatures: Vec<String>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct UTXOInput {
    pub transaction_id: Vec<u8>,
    pub index: u16,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct OutputResponse {
    pub message_id: String,
    pub transaction_id: String,
    pub output_index: u16,
    pub is_spent: bool,
    pub output: OutputDto,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct OutputDto {
    signature_locked_single: SignatureLockedSingleOutputDto,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct SignatureLockedSingleOutputDto {
    pub kind: u32,
    pub address: AddressDto,
    pub amount: u64,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct AddressDto {
    ed25519: Ed25519AddressDto,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Ed25519AddressDto {
    pub kind: u32,
    pub address: String,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Message {
    pub message_id: String,
    pub network_id: u64,
    pub parent1: String,
    pub parent2: String,
    pub payload: Option<Payload>,
    pub nonce: u64,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Payload {
    pub transaction: Option<Vec<Transaction>>,
    pub milestone: Option<Vec<Milestone>>,
    pub indexation: Option<Vec<Indexation>>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Transaction {
    pub essence: TransactionPayloadEssence,
    pub unlock_blocks: Vec<UnlockBlock>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Milestone {
    pub essence: MilestonePayloadEssence,
    pub signatures: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct MilestonePayloadEssence {
    pub index: u32,
    pub timestamp: u64,
    pub parent1: String,
    pub parent2: String,
    pub merkle_proof: [u8; MILESTONE_MERKLE_PROOF_LENGTH],
    pub public_keys: Vec<[u8; MILESTONE_PUBLIC_KEY_LENGTH]>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Indexation {
    pub index: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct TransactionPayloadEssence {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub payload: Option<Payload>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Output {
    pub address: String,
    pub amount: u64,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Input {
    pub transaction_id: String,
    pub index: u16,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct UnlockBlock {
    pub signature: Option<Ed25519Signature>,
    pub reference: Option<u16>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Ed25519Signature {
    pub public_key: [u8; 32],
    pub signature: Vec<u8>,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct BrokerOptions {
    /// automatic disconnect or not
    pub automatic_disconnect: bool,
    /// broker timeout in secs
    pub timeout: u64,
    /// use websockets or not
    pub use_ws: bool,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct LedgerInclusionStateDto {
    pub state: String,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct InfoResponse {
    pub name: String,
    pub version: String,
    pub is_healthy: bool,
    pub network_id: String,
    pub bech32_hrp: String,
    pub latest_milestone_index: u32,
    pub solid_milestone_index: u32,
    pub pruning_index: u32,
    pub features: Vec<String>,
    pub min_pow_score: f64,
}

pub struct NetworkInfo {
    /// Network of the Iota nodes belong to
    pub network: String,
    /// Network ID
    pub network_id: u64,
    /// Bech32 HRP
    pub bech32_hrp: String,
    /// Mininum proof of work score
    pub min_pow_score: f64,
    /// Local proof of work
    pub local_pow: bool,
}

impl From<RustOutputResponse> for OutputResponse {
    fn from(output: RustOutputResponse) -> Self {
        Self {
            message_id: output.message_id,
            transaction_id: output.transaction_id,
            output_index: output.output_index,
            is_spent: output.is_spent,
            output: output.output.into(),
        }
    }
}

impl From<RustOutputDto> for OutputDto {
    fn from(output: RustOutputDto) -> Self {
        Self {
            signature_locked_single: match output {
                RustOutputDto::SignatureLockedSingle(signature) => signature.into(),
            },
        }
    }
}

impl From<RustEd25519AddressDto> for Ed25519AddressDto {
    fn from(address: RustEd25519AddressDto) -> Self {
        Self {
            kind: address.kind,
            address: address.address,
        }
    }
}

impl From<RustSignatureLockedSingleOutputDto> for SignatureLockedSingleOutputDto {
    fn from(address: RustSignatureLockedSingleOutputDto) -> Self {
        Self {
            kind: address.kind,
            address: address.address.into(),
            amount: address.amount,
        }
    }
}

impl From<RustAddressDto> for AddressDto {
    fn from(address: RustAddressDto) -> Self {
        Self {
            ed25519: match address {
                RustAddressDto::Ed25519(ed25519) => ed25519.into(),
            },
        }
    }
}

impl From<RustBalanceForAddressResponse> for BalanceForAddressResponse {
    fn from(balance_for_address_response: RustBalanceForAddressResponse) -> Self {
        BalanceForAddressResponse {
            address_type: balance_for_address_response.address_type,
            address: balance_for_address_response.address,
            max_results: balance_for_address_response.max_results,
            count: balance_for_address_response.count,
            balance: balance_for_address_response.balance,
        }
    }
}

impl From<RustMessageMetadataResponse> for MessageMetadataResponse {
    fn from(message_metadata_response: RustMessageMetadataResponse) -> Self {
        Self {
            message_id: message_metadata_response.message_id,
            parent_1_message_id: message_metadata_response.parent_1_message_id,
            parent_2_message_id: message_metadata_response.parent_2_message_id,
            is_solid: message_metadata_response.is_solid,
            referenced_by_milestone_index: message_metadata_response.referenced_by_milestone_index,
            milestone_index: message_metadata_response.milestone_index,
            ledger_inclusion_state: {
                if let Some(state) = message_metadata_response.ledger_inclusion_state {
                    Some(state.into())
                } else {
                    None
                }
            },
            conflict_reason: message_metadata_response.conflict_reason,
            should_promote: message_metadata_response.should_promote,
            should_reattach: message_metadata_response.should_reattach,
        }
    }
}

impl From<RustInfoResponse> for InfoResponse {
    fn from(info: RustInfoResponse) -> Self {
        InfoResponse {
            name: info.name,
            version: info.version,
            is_healthy: info.is_healthy,
            network_id: info.network_id,
            bech32_hrp: info.bech32_hrp,
            latest_milestone_index: info.latest_milestone_index,
            solid_milestone_index: info.solid_milestone_index,
            pruning_index: info.pruning_index,
            features: info.features,
            min_pow_score: info.min_pow_score,
        }
    }
}

impl From<RustNetworkInfo> for NetworkInfo {
    fn from(network_info: RustNetworkInfo) -> Self {
        NetworkInfo {
            network: network_info.network,
            network_id: network_info.network_id,
            bech32_hrp: network_info.bech32_hrp,
            min_pow_score: network_info.min_pow_score,
            local_pow: network_info.local_pow,
        }
    }
}

impl From<RustMilestoneDto> for MilestoneDto {
    fn from(milestone_dto: RustMilestoneDto) -> Self {
        Self {
            kind: milestone_dto.kind,
            index: milestone_dto.index,
            timestamp: milestone_dto.timestamp,
            parent_1_message_id: milestone_dto.parent_1_message_id,
            parent_2_message_id: milestone_dto.parent_2_message_id,
            inclusion_merkle_proof: milestone_dto.inclusion_merkle_proof,
            public_keys: milestone_dto.public_keys,
            signatures: milestone_dto.signatures,
        }
    }
}

impl From<RustLedgerInclusionStateDto> for LedgerInclusionStateDto {
    fn from(state: RustLedgerInclusionStateDto) -> Self {
        match state {
            RustLedgerInclusionStateDto::Conflicting => Self {
                state: "Conflicting".to_string(),
            },
            RustLedgerInclusionStateDto::Included => Self {
                state: "Included".to_string(),
            },
            RustLedgerInclusionStateDto::NoTransaction => Self {
                state: "NoTransaction".to_string(),
            },
        }
    }
}

impl TryFrom<RustTransactionPayloadEssence> for TransactionPayloadEssence {
    type Error = Error;
    fn try_from(essence: RustTransactionPayloadEssence) -> Result<Self> {
        Ok(TransactionPayloadEssence {
            inputs: essence
                .inputs()
                .iter()
                .cloned()
                .map(|input| {
                    if let RustInput::UTXO(input) = input {
                        Input {
                            transaction_id: input.output_id().transaction_id().to_string(),
                            index: input.output_id().index(),
                        }
                    } else {
                        unreachable!()
                    }
                })
                .collect(),
            outputs: essence
                .outputs()
                .iter()
                .cloned()
                .map(|output| {
                    if let RustOutput::SignatureLockedSingle(output) = output {
                        Output {
                            address: unsafe { output.address().to_bech32(BECH32_HRP) },
                            amount: output.amount(),
                        }
                    } else {
                        unreachable!()
                    }
                })
                .collect(),
            payload: if essence.payload().is_some() {
                if let Some(RustPayload::Indexation(payload)) = essence.payload() {
                    Some(Payload {
                        transaction: None,
                        milestone: None,
                        indexation: Some(vec![Indexation {
                            index: payload.index().to_string(),
                            data: payload.data().try_into().unwrap_or_else(|_| {
                                panic!(
                                    "invalid Indexation Payload {:?} with data: {:?}",
                                    essence,
                                    payload.data()
                                )
                            }),
                        }]),
                    })
                } else {
                    unreachable!()
                }
            } else {
                None
            },
        })
    }
}

impl TryFrom<RustMilestonePayloadEssence> for MilestonePayloadEssence {
    type Error = Error;
    fn try_from(essence: RustMilestonePayloadEssence) -> Result<Self> {
        Ok(MilestonePayloadEssence {
            index: essence.index(),
            timestamp: essence.timestamp(),
            parent1: essence.parent1().to_string(),
            parent2: essence.parent2().to_string(),
            merkle_proof: essence.merkle_proof().try_into()?,
            public_keys: essence
                .public_keys()
                .iter()
                .map(|public_key| {
                    public_key.to_vec().try_into().unwrap_or_else(|_| {
                        panic!(
                            "invalid MilestonePayloadEssence {:?} with public key: {:?}",
                            essence,
                            essence.public_keys()
                        )
                    })
                })
                .collect(),
        })
    }
}

impl TryFrom<RustUnlockBlock> for UnlockBlock {
    type Error = Error;
    fn try_from(unlock_block: RustUnlockBlock) -> Result<Self> {
        if let RustUnlockBlock::Signature(RustSignatureUnlock::Ed25519(signature)) = unlock_block {
            Ok(UnlockBlock {
                signature: Some(Ed25519Signature {
                    public_key: signature.public_key().to_vec().try_into().unwrap_or_else(|_| {
                        panic!(
                            "invalid Ed25519Signature {:?} with public key: {:?}",
                            signature,
                            signature.public_key()
                        )
                    }),
                    signature: signature.signature().to_vec(),
                }),
                reference: None,
            })
        } else if let RustUnlockBlock::Reference(signature) = unlock_block {
            Ok(UnlockBlock {
                signature: None,
                reference: Some(signature.index()),
            })
        } else {
            unreachable!()
        }
    }
}

impl TryFrom<RustMessage> for Message {
    type Error = Error;
    fn try_from(msg: RustMessage) -> Result<Self> {
        let payload = msg.payload().as_ref();
        let payload = match payload {
            Some(RustPayload::Transaction(payload)) => Some(Payload {
                transaction: Some(vec![Transaction {
                    essence: payload.essence().to_owned().try_into()?,
                    unlock_blocks: payload
                        .unlock_blocks()
                        .iter()
                        .cloned()
                        .map(|unlock_block| unlock_block.try_into().expect("Invalid UnlockBlock"))
                        .collect(),
                }]),
                milestone: None,
                indexation: None,
            }),
            Some(RustPayload::Indexation(payload)) => Some(Payload {
                transaction: None,
                milestone: None,
                indexation: Some(vec![Indexation {
                    index: payload.index().to_string(),
                    data: payload.data().try_into().unwrap_or_else(|_| {
                        panic!(
                            "invalid Indexation Payload {:?} with data: {:?}",
                            payload,
                            payload.data()
                        )
                    }),
                }]),
            }),
            Some(RustPayload::Milestone(payload)) => Some(Payload {
                transaction: None,
                milestone: Some(vec![Milestone {
                    essence: payload.essence().to_owned().try_into()?,
                    signatures: payload
                        .signatures()
                        .iter()
                        .map(|signature| (*signature).to_vec())
                        .collect(),
                }]),
                indexation: None,
            }),
            _ => None,
        };

        Ok(Message {
            message_id: msg.id().0.to_string(),
            network_id: msg.network_id(),
            parent1: msg.parent1().to_string(),
            parent2: msg.parent2().to_string(),
            payload,
            nonce: msg.nonce(),
        })
    }
}

impl TryFrom<TransactionPayloadEssence> for RustTransactionPayloadEssence {
    type Error = Error;
    fn try_from(essence: TransactionPayloadEssence) -> Result<Self> {
        let mut builder = RustTransactionPayloadEssence::builder();
        let inputs: Vec<RustInput> = essence
            .inputs
            .iter()
            .map(|input| {
                RustUTXOInput::new(
                    RustTransationId::from_str(&input.transaction_id[..]).unwrap_or_else(|_| {
                        panic!(
                            "invalid UTXOInput transaction_id: {} with input index {}",
                            input.transaction_id, input.index
                        )
                    }),
                    input.index,
                )
                .unwrap_or_else(|_| {
                    panic!(
                        "invalid UTXOInput transaction_id: {} with input index {}",
                        input.transaction_id, input.index
                    )
                })
                .into()
            })
            .collect();
        for input in inputs {
            builder = builder.add_input(input);
        }

        let outputs: Vec<RustOutput> = essence
            .outputs
            .iter()
            .map(|output| {
                RustSignatureLockedSingleOutput::new(
                    RustAddress::from(RustEd25519Address::from_str(&output.address[..]).unwrap_or_else(|_| {
                        panic!(
                            "invalid SignatureLockedSingleOutput with output address: {}",
                            output.address
                        )
                    })),
                    output.amount,
                )
                .unwrap_or_else(|_| {
                    panic!(
                        "invalid SignatureLockedSingleOutput with output address: {}",
                        output.address
                    )
                })
                .into()
            })
            .collect();
        for output in outputs {
            builder = builder.add_output(output);
        }
        if let Some(indexation_payload) = &essence.payload {
            let index = RustIndexationPayload::new(
                indexation_payload
                    .indexation
                    .as_ref()
                    .unwrap_or_else(|| panic!("Invalid IndexationPayload: {:?}", indexation_payload))[0]
                    .index
                    .clone(),
                &(indexation_payload
                    .indexation
                    .as_ref()
                    .unwrap_or_else(|| panic!("Invalid IndexationPayload: {:?}", indexation_payload))[0]
                    .data)
                    .clone(),
            )
            .unwrap();
            builder = builder.with_payload(RustPayload::from(index));
        }
        Ok(builder.finish()?)
    }
}

impl TryFrom<Ed25519Signature> for RustSignatureUnlock {
    type Error = Error;
    fn try_from(signature: Ed25519Signature) -> Result<Self> {
        let mut public_key = [0u8; 32];
        hex::decode_to_slice(signature.public_key, &mut public_key)?;
        let signature = hex::decode(signature.signature)?.into_boxed_slice();
        Ok(RustEd25519Signature::new(public_key, signature).into())
    }
}

impl TryFrom<UnlockBlock> for RustUnlockBlock {
    type Error = Error;
    fn try_from(block: UnlockBlock) -> Result<Self> {
        if let Some(signature) = block.signature {
            let sig: RustSignatureUnlock = signature.try_into()?;
            Ok(sig.into())
        } else {
            let reference: RustReferenceUnlock = block
                .reference
                .unwrap()
                .try_into()
                .unwrap_or_else(|_| panic!("Invalid ReferenceUnlock: {:?}", block.reference));
            Ok(reference.into())
        }
    }
}

impl TryFrom<Payload> for RustPayload {
    type Error = Error;
    fn try_from(payload: Payload) -> Result<Self> {
        if let Some(transaction_payload) = &payload.transaction {
            let mut transaction = RustTransactionPayload::builder();
            transaction = transaction.with_essence(transaction_payload[0].essence.clone().try_into()?);

            let unlock_blocks = transaction_payload[0].unlock_blocks.clone();
            for unlock_block in unlock_blocks {
                transaction = transaction.add_unlock_block(unlock_block.try_into()?);
            }

            Ok(RustPayload::Transaction(Box::new(transaction.finish()?)))
        } else {
            let indexation = RustIndexationPayload::new(
                (&payload
                    .indexation
                    .as_ref()
                    .unwrap_or_else(|| panic!("Invalid Payload: {:?}", payload))[0]
                    .index
                    .clone())
                    .to_owned(),
                &payload
                    .indexation
                    .as_ref()
                    .unwrap_or_else(|| panic!("Invalid Payload: {:?}", payload))[0]
                    .data,
            )?;
            Ok(RustPayload::Indexation(Box::new(indexation)))
        }
    }
}
