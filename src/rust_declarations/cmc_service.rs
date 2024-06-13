// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
use candid::{self, CandidType, Deserialize, Principal};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub enum ExchangeRateCanister {
    Set(Principal),
    Unset,
}

#[derive(CandidType, Deserialize)]
pub struct AccountIdentifier {
    bytes: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct CyclesCanisterInitPayload {
    exchange_rate_canister: Option<ExchangeRateCanister>,
    last_purged_notification: Option<u64>,
    governance_canister_id: Option<Principal>,
    minting_account_id: Option<AccountIdentifier>,
    ledger_canister_id: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct IcpXdrConversionRate {
    xdr_permyriad_per_icp: u64,
    timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize)]
pub struct IcpXdrConversionRateResponse {
    certificate: Vec<u8>,
    data: IcpXdrConversionRate,
    hash_tree: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct SubnetTypesToSubnetsResponse {
    data: Vec<(String, Vec<Principal>)>,
}

pub type BlockIndex = u64;
#[derive(CandidType, Deserialize)]
pub struct NotifyCreateCanisterArg {
    pub controller: Principal,
    pub block_index: BlockIndex,
    pub subnet_type: Option<String>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum NotifyError {
    Refunded {
        block_index: Option<BlockIndex>,
        reason: String,
    },
    InvalidTransaction(String),
    Other {
        error_message: String,
        error_code: u64,
    },
    Processing,
    TransactionTooOld(BlockIndex),
}

#[derive(CandidType, Deserialize)]
pub enum NotifyCreateCanisterResult {
    Ok(Principal),
    Err(NotifyError),
}

#[derive(CandidType, Deserialize)]
pub struct NotifyTopUpArg {
    pub block_index: BlockIndex,
    pub canister_id: Principal,
}

pub type Cycles = candid::Nat;
#[derive(CandidType, Deserialize)]
pub enum NotifyTopUpResult {
    Ok(Cycles),
    Err(NotifyError),
}

pub struct CmcService(pub Principal);
impl CmcService {
    pub async fn get_icp_xdr_conversion_rate(&self) -> Result<(IcpXdrConversionRateResponse,)> {
        ic_cdk::call(self.0, "get_icp_xdr_conversion_rate", ()).await
    }
    pub async fn get_subnet_types_to_subnets(&self) -> Result<(SubnetTypesToSubnetsResponse,)> {
        ic_cdk::call(self.0, "get_subnet_types_to_subnets", ()).await
    }
    pub async fn notify_create_canister(
        &self,
        arg0: NotifyCreateCanisterArg,
    ) -> Result<(NotifyCreateCanisterResult,)> {
        ic_cdk::call(self.0, "notify_create_canister", (arg0,)).await
    }
    pub async fn notify_top_up(&self, arg0: NotifyTopUpArg) -> Result<(NotifyTopUpResult,)> {
        ic_cdk::call(self.0, "notify_top_up", (arg0,)).await
    }
}
