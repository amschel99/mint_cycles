// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
use candid::{self, CandidType, Deserialize, Principal};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub struct Account {
    owner: Principal,
    subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize)]
pub struct UpgradeArgs {
    maximum_number_of_accounts: Option<u64>,
    icrc1_minting_account: Option<Account>,
}

#[derive(CandidType, Deserialize)]
pub struct Tokens {
    e8s: u64,
}

#[derive(CandidType, Deserialize)]
pub struct Duration {
    secs: u64,
    nanos: u32,
}

#[derive(CandidType, Deserialize)]
pub struct ArchiveOptions {
    num_blocks_to_archive: u64,
    max_transactions_per_response: Option<u64>,
    trigger_threshold: u64,
    max_message_size_bytes: Option<u64>,
    cycles_for_archive_creation: Option<u64>,
    node_max_memory_size_bytes: Option<u64>,
    controller_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    send_whitelist: Vec<Principal>,
    token_symbol: Option<String>,
    transfer_fee: Option<Tokens>,
    minting_account: String,
    transaction_window: Option<Duration>,
    max_message_size_bytes: Option<u64>,
    icrc1_minting_account: Option<Account>,
    archive_options: Option<ArchiveOptions>,
    initial_values: Vec<(String, Tokens)>,
    token_name: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub enum LedgerCanisterPayload {
    Upgrade(Option<UpgradeArgs>),
    Init(InitArgs),
}

#[derive(CandidType, Deserialize)]
pub struct BinaryAccountBalanceArgs {
    account: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct AccountBalanceArgs {
    account: String,
}

#[derive(CandidType, Deserialize)]
pub struct ArchiveInfo {
    canister_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct Archives {
    archives: Vec<ArchiveInfo>,
}

#[derive(CandidType, Deserialize)]
pub struct Decimals {
    decimals: u32,
}

#[derive(CandidType, Deserialize)]
pub enum MetadataValue {
    Int(candid::Int),
    Nat(candid::Nat),
    Blob(Vec<u8>),
    Text(String),
}

#[derive(CandidType, Deserialize)]
pub struct StandardRecord {
    url: String,
    name: String,
}

#[derive(CandidType, Deserialize)]
pub struct TransferArg {
    to: Account,
    fee: Option<candid::Nat>,
    memo: Option<Vec<u8>>,
    from_subaccount: Option<Vec<u8>>,
    created_at_time: Option<u64>,
    amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum TransferError {
    GenericError {
        message: String,
        error_code: candid::Nat,
    },
    TemporarilyUnavailable,
    BadBurn {
        min_burn_amount: candid::Nat,
    },
    Duplicate {
        duplicate_of: candid::Nat,
    },
    BadFee {
        expected_fee: candid::Nat,
    },
    CreatedInFuture {
        ledger_time: u64,
    },
    TooOld,
    InsufficientFunds {
        balance: candid::Nat,
    },
}

#[derive(CandidType, Deserialize)]
pub struct Name {
    name: String,
}

#[derive(CandidType, Deserialize)]
pub struct GetBlocksArgs {
    start: u64,
    length: u64,
}

#[derive(CandidType, Deserialize)]
pub struct TimeStamp {
    timestamp_nanos: u64,
}

#[derive(CandidType, Deserialize)]
pub enum CandidOperation {
    Approve {
        fee: Tokens,
        from: Vec<u8>,
        allowance_e8s: candid::Int,
        allowance: Tokens,
        expires_at: Option<TimeStamp>,
        spender: Vec<u8>,
    },
    Burn {
        from: Vec<u8>,
        amount: Tokens,
    },
    Mint {
        to: Vec<u8>,
        amount: Tokens,
    },
    Transfer {
        to: Vec<u8>,
        fee: Tokens,
        from: Vec<u8>,
        amount: Tokens,
    },
    TransferFrom {
        to: Vec<u8>,
        fee: Tokens,
        from: Vec<u8>,
        amount: Tokens,
        spender: Vec<u8>,
    },
}

#[derive(CandidType, Deserialize)]
pub struct CandidTransaction {
    memo: u64,
    icrc1_memo: Option<Vec<u8>>,
    operation: Option<CandidOperation>,
    created_at_time: TimeStamp,
}

#[derive(CandidType, Deserialize)]
pub struct CandidBlock {
    transaction: CandidTransaction,
    timestamp: TimeStamp,
    parent_hash: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize)]
pub struct BlockRange {
    blocks: Vec<CandidBlock>,
}

#[derive(CandidType, Deserialize)]
pub enum GetBlocksError {
    BadFirstBlockIndex {
        requested_index: u64,
        first_valid_index: u64,
    },
    Other {
        error_message: String,
        error_code: u64,
    },
}

#[derive(CandidType, Deserialize)]
pub enum ArchivedBlocksRangeCallbackRet0 {
    Ok(BlockRange),
    Err(GetBlocksError),
}

candid::define_function!(pub ArchivedBlocksRangeCallback : (GetBlocksArgs) -> (
    ArchivedBlocksRangeCallbackRet0,
  ) query);
#[derive(CandidType, Deserialize)]
pub struct ArchivedBlocksRange {
    callback: ArchivedBlocksRangeCallback,
    start: u64,
    length: u64,
}

#[derive(CandidType, Deserialize)]
pub struct QueryBlocksResponse {
    certificate: Option<Vec<u8>>,
    blocks: Vec<CandidBlock>,
    chain_length: u64,
    first_block_index: u64,
    archived_blocks: Vec<ArchivedBlocksRange>,
}

#[derive(CandidType, Deserialize)]
pub enum ArchivedEncodedBlocksRangeCallbackRet0 {
    Ok(Vec<Vec<u8>>),
    Err(GetBlocksError),
}

candid::define_function!(pub ArchivedEncodedBlocksRangeCallback : (
    GetBlocksArgs,
  ) -> (ArchivedEncodedBlocksRangeCallbackRet0) query);
#[derive(CandidType, Deserialize)]
pub struct ArchivedEncodedBlocksRange {
    callback: ArchivedEncodedBlocksRangeCallback,
    start: u64,
    length: u64,
}

#[derive(CandidType, Deserialize)]
pub struct QueryEncodedBlocksResponse {
    certificate: Option<Vec<u8>>,
    blocks: Vec<Vec<u8>>,
    chain_length: u64,
    first_block_index: u64,
    archived_blocks: Vec<ArchivedEncodedBlocksRange>,
}

#[derive(CandidType, Deserialize)]
pub struct SendArgs {
    to: String,
    fee: Tokens,
    memo: u64,
    from_subaccount: Option<Vec<u8>>,
    created_at_time: Option<TimeStamp>,
    amount: Tokens,
}

#[derive(CandidType, Deserialize)]
pub struct Symbol {
    symbol: String,
}

#[derive(CandidType, Deserialize)]
pub struct TransferArgs {
    to: Vec<u8>,
    fee: Tokens,
    memo: u64,
    from_subaccount: Option<Vec<u8>>,
    created_at_time: Option<TimeStamp>,
    amount: Tokens,
}

#[derive(CandidType, Deserialize)]
pub enum TransferError1 {
    TxTooOld { allowed_window_nanos: u64 },
    BadFee { expected_fee: Tokens },
    TxDuplicate { duplicate_of: u64 },
    TxCreatedInFuture,
    InsufficientFunds { balance: Tokens },
}

#[derive(CandidType, Deserialize)]
pub enum Result1 {
    Ok(u64),
    Err(TransferError1),
}

#[derive(CandidType, Deserialize)]
pub struct TransferFeeArg0 {}

#[derive(CandidType, Deserialize)]
pub struct TransferFee {
    transfer_fee: Tokens,
}

#[derive(CandidType, Deserialize)]
pub enum Result2 {
    Ok(candid::Nat),
    Err(TransferError),
}

pub struct IcpLedgerService(pub Principal);
impl IcpLedgerService {
    pub async fn account_balance(&self, arg0: BinaryAccountBalanceArgs) -> Result<(Tokens,)> {
        ic_cdk::call(self.0, "account_balance", (arg0,)).await
    }
    pub async fn account_balance_dfx(&self, arg0: AccountBalanceArgs) -> Result<(Tokens,)> {
        ic_cdk::call(self.0, "account_balance_dfx", (arg0,)).await
    }
    pub async fn archives(&self) -> Result<(Archives,)> {
        ic_cdk::call(self.0, "archives", ()).await
    }
    pub async fn decimals(&self) -> Result<(Decimals,)> {
        ic_cdk::call(self.0, "decimals", ()).await
    }
    pub async fn icrc1_balance_of(&self, arg0: Account) -> Result<(candid::Nat,)> {
        ic_cdk::call(self.0, "icrc1_balance_of", (arg0,)).await
    }
    pub async fn icrc1_decimals(&self) -> Result<(u8,)> {
        ic_cdk::call(self.0, "icrc1_decimals", ()).await
    }
    pub async fn icrc1_fee(&self) -> Result<(candid::Nat,)> {
        ic_cdk::call(self.0, "icrc1_fee", ()).await
    }
    pub async fn icrc1_metadata(&self) -> Result<(Vec<(String, MetadataValue)>,)> {
        ic_cdk::call(self.0, "icrc1_metadata", ()).await
    }
    pub async fn icrc1_minting_account(&self) -> Result<(Option<Account>,)> {
        ic_cdk::call(self.0, "icrc1_minting_account", ()).await
    }
    pub async fn icrc1_name(&self) -> Result<(String,)> {
        ic_cdk::call(self.0, "icrc1_name", ()).await
    }
    pub async fn icrc1_supported_standards(&self) -> Result<(Vec<StandardRecord>,)> {
        ic_cdk::call(self.0, "icrc1_supported_standards", ()).await
    }
    pub async fn icrc1_symbol(&self) -> Result<(String,)> {
        ic_cdk::call(self.0, "icrc1_symbol", ()).await
    }
    pub async fn icrc1_total_supply(&self) -> Result<(candid::Nat,)> {
        ic_cdk::call(self.0, "icrc1_total_supply", ()).await
    }
    pub async fn icrc1_transfer(&self, arg0: TransferArg) -> Result<(Result2,)> {
        ic_cdk::call(self.0, "icrc1_transfer", (arg0,)).await
    }
    pub async fn name(&self) -> Result<(Name,)> {
        ic_cdk::call(self.0, "name", ()).await
    }
    pub async fn query_blocks(&self, arg0: GetBlocksArgs) -> Result<(QueryBlocksResponse,)> {
        ic_cdk::call(self.0, "query_blocks", (arg0,)).await
    }
    pub async fn query_encoded_blocks(
        &self,
        arg0: GetBlocksArgs,
    ) -> Result<(QueryEncodedBlocksResponse,)> {
        ic_cdk::call(self.0, "query_encoded_blocks", (arg0,)).await
    }
    pub async fn send_dfx(&self, arg0: SendArgs) -> Result<(u64,)> {
        ic_cdk::call(self.0, "send_dfx", (arg0,)).await
    }
    pub async fn symbol(&self) -> Result<(Symbol,)> {
        ic_cdk::call(self.0, "symbol", ()).await
    }
    pub async fn transfer(&self, arg0: TransferArgs) -> Result<(Result1,)> {
        ic_cdk::call(self.0, "transfer", (arg0,)).await
    }
    pub async fn transfer_fee(&self, arg0: TransferFeeArg0) -> Result<(TransferFee,)> {
        ic_cdk::call(self.0, "transfer_fee", (arg0,)).await
    }
}
