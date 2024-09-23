use chrono::NaiveDateTime;
use ethers::types::{Address, H256};
use intmax2_zkp::ethereum_types::u256::U256;
use mining_circuit_v1::eligible_tree::EligibleLeaf;
use num_bigint::BigUint;

use crate::{
    external_api::contracts::utils::get_address,
    state::{
        keys::Key,
        state::State,
    },
    utils::{deposit_hash_tree::DepositHashTree, eligible_tree_with_map::EligibleTreeWithMap},
};

pub async fn get_dummy_keys() -> Key {
    let deposit_private_key: H256 =
        "0xdf57089febbacf7ba0bc227dafbffa9fc08a93fdc68e1e42411a14efcf23656e"
            .parse()
            .unwrap();
    let deposit_address = get_address(deposit_private_key).await;
    let withdrawal_address: Address = "0x8626f6940E2eb28930eFb4CeF49B2d1F2C9C1199"
        .parse()
        .unwrap();

    Key {
        deposit_private_key,
        deposit_address,
        claim_private_key: Some(deposit_private_key),
        claim_address: Some(deposit_address),
        withdrawal_address: Some(withdrawal_address),
    }
}

pub async fn get_dummy_state() -> State {
    let mut eligible_tree = EligibleTreeWithMap::new();
    for i in 0..100 {
        eligible_tree.push(EligibleLeaf {
            deposit_index: i,
            amount: U256::try_from(BigUint::from(10u32).pow(18)).unwrap(),
        });
    }

    let state = State {
        deposit_hash_tree: DepositHashTree::new(),
        eligible_tree,
        last_tree_feched_at: NaiveDateTime::default(),
        last_deposit_synced_block: 0,
        prover: None,
    };
    state
}