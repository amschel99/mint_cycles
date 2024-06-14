use candid::Nat;
use ic_cdk::id;
use ic_ledger_types::MAINNET_CYCLES_MINTING_CANISTER_ID;

use crate::{
    errors::CustomError,
    rust_declarations::cmc_service::{CmcService, NotifyTopUpArg, NotifyTopUpResult},
};

pub struct CMC {}

impl CMC {
    pub async fn top_up_self(block_index: u64) -> Result<Nat, CustomError> {
        match CmcService(MAINNET_CYCLES_MINTING_CANISTER_ID)
            .notify_top_up(NotifyTopUpArg {
                block_index,
                canister_id: id(),
            })
            .await
        {
            Ok((result,)) => match result {
                NotifyTopUpResult::Ok(cycles) => Ok(cycles),
                NotifyTopUpResult::Err(err) => Err(CustomError::custom(format!("{:?}", err))),
            },
            Err((_, err)) => Err(CustomError::custom(format!("{:?}", err))),
        }
    }
}
