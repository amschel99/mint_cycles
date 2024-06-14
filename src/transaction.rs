use super::cmc::CMC;
use crate::errors::CustomError;

use crate::ledger::Ledger;
use candid::Principal;
use ic_cdk::id;
use ic_ledger_types::{AccountIdentifier, Memo, Subaccount, Tokens, TransferArgs};

pub static MEMO_TOP_UP_CANISTER: Memo = Memo(1347768404_u64);

pub static ICP_TRANSACTION_FEE: Tokens = Tokens::from_e8s(10000);

pub async fn mint_cycles(amount: Tokens) -> Result<candid::Nat, CustomError> {
    let transfer_args = TransferArgs {
        memo: MEMO_TOP_UP_CANISTER,
        amount,
        fee: ICP_TRANSACTION_FEE,
        from_subaccount: None,
        to: AccountIdentifier::new(
            &Principal::from_text("rkp4c-7iaaa-aaaaa-aaaca-cai").unwrap(), //cycles minting canister ID
            &Subaccount::from(id()),
        ),
        created_at_time: None,
    };

    match Ledger::transfer_icp(transfer_args).await {
        // If the transaction is successfull, return the block index of the transaction
        Ok(cmc_block_index) => match CMC::top_up_self(cmc_block_index).await {
            Ok(cycles) => Ok(cycles),
            Err(err) => {
                Err(err).map_err(|e| CustomError::custom(format!("Failed to mint cycles: {:?}", e)))
            }
        },
        Err(err) => {
            Err(err).map_err(|e| CustomError::custom(format!("Transaction failed {:?}", e)))
        }
    }
}
