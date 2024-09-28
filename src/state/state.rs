use chrono::NaiveDateTime;

use super::{keys::Key, prover::Prover};
use crate::{
    services::{
        assets_status::{fetch_assets_status, AssetsStatus},
        sync::sync_trees,
    },
    utils::{deposit_hash_tree::DepositHashTree, eligible_tree_with_map::EligibleTreeWithMap},
};

pub struct State {
    pub deposit_hash_tree: DepositHashTree,
    pub eligible_tree: EligibleTreeWithMap,
    pub last_tree_feched_at: NaiveDateTime,
    pub last_deposit_synced_block: u64,
    pub prover: Option<Prover>,
}

impl State {
    pub fn new() -> Self {
        Self {
            deposit_hash_tree: DepositHashTree::new(),
            eligible_tree: EligibleTreeWithMap::new(),
            last_tree_feched_at: NaiveDateTime::default(),
            last_deposit_synced_block: 0,
            prover: None,
        }
    }

    pub fn build_circuit(&mut self) -> anyhow::Result<()> {
        self.prover = Some(Prover::new());
        Ok(())
    }

    pub async fn sync_trees(&mut self) -> anyhow::Result<()> {
        sync_trees(
            &mut self.last_deposit_synced_block,
            &mut self.last_tree_feched_at,
            &mut self.deposit_hash_tree,
            &mut self.eligible_tree,
        )
        .await?;
        Ok(())
    }

    pub async fn sync_and_fetch_assets(&mut self, key: &Key) -> anyhow::Result<AssetsStatus> {
        self.sync_trees().await?;
        fetch_assets_status(&self, key.deposit_address, key.deposit_private_key).await
    }
}
