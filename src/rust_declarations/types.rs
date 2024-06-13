use candid::{CandidType, Nat, Principal};
use ic_ledger_types::Tokens;
use serde::Deserialize;
use std::borrow::Cow;

use candid::{Decode, Encode};
use ic_stable_structures::{BoundedStorable, Storable};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MultisigData {
    pub canister_id: Principal,
    pub group_identifier: Option<Principal>,
    pub created_by: Principal,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Storable for MultisigData {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for MultisigData {
    const MAX_SIZE: u32 = 102400;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum TransactionStatus {
    IcpToIndexFailed,
    IcpToCmcFailed,
    CyclesToIndexFailed,
    InsufficientIcp,
    Success,
    Pending,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransactionData {
    pub icp_transfer_block_index: u64,
    pub cmc_transfer_block_index: Option<u64>,
    pub icp_amount: Option<Tokens>,
    pub cycles_amount: Option<Nat>,
    pub initialized_by: Principal,
    pub created_at: u64,
    pub status: TransactionStatus,
    pub error_message: Option<String>,
}

impl Storable for TransactionData {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for TransactionData {
    const MAX_SIZE: u32 = 102400;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum UpdateCycleBalanceArgs {
    Add(Nat),
    Subtract(Nat),
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum UpdateIcpBalanceArgs {
    Add(Tokens),
    Subtract(Tokens),
}
