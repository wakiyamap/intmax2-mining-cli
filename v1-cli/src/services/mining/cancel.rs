use intmax2_zkp::ethereum_types::u32limb_trait::U32LimbTrait as _;

use crate::{
    external_api::contracts::{
        events::Deposited,
        int1::{get_int1_contract_with_signer, int_1},
        utils::handle_contract_call,
    },
    state::state::State,
};

pub async fn cancel_task(state: &State, event: Deposited) -> anyhow::Result<()> {
    let deposit = int_1::Deposit {
        recipient_salt_hash: event.recipient_salt_hash.to_bytes_be().try_into().unwrap(),
        token_index: event.token_index,
        amount: ethers::types::U256::from_big_endian(&event.amount.to_bytes_be()),
    };
    let deposit_address = state.private_data.deposit_address;
    let int1 = get_int1_contract_with_signer(state.private_data.deposit_private_key).await?;
    let tx = int1.cancel_deposit(event.deposit_id.into(), deposit.clone());
    handle_contract_call(tx, deposit_address, "deposit", "cancel").await?;
    Ok(())
}
