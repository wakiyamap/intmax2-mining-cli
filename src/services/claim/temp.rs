use std::path::Path;

use intmax2_zkp::wrapper_config::plonky2_config::PoseidonBN128GoldilocksConfig;
use mining_circuit_v1::claim::claim_inner_circuit::ClaimInnerValue;
use plonky2::{field::goldilocks_field::GoldilocksField, plonk::proof::ProofWithPublicInputs};
use serde::{Deserialize, Serialize};

use crate::utils::file::create_file_with_content;

const CLAIM_TEMP_PATH: &str = "data/temp/claim_temp.json";

type F = GoldilocksField;
type C = PoseidonBN128GoldilocksConfig;
const D: usize = 2;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ClaimStep {
    Plonky2Prove,
    GnarkStart,
    GnarkGetProof,
    ContractCall,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimStatus {
    pub next_step: ClaimStep,
    pub witness: Vec<ClaimInnerValue>,
    pub plonlky2_proof: Option<ProofWithPublicInputs<F, C, D>>,
    pub job_id: Option<String>,
    pub start_query_time: Option<u64>, // unix timestamp
    pub gnark_proof: Option<String>,
}

impl ClaimStatus {
    pub fn new() -> anyhow::Result<Self> {
        let file = std::fs::read(CLAIM_TEMP_PATH)?;
        let status: Self = serde_json::from_slice(&file)?;
        Ok(status)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let input = serde_json::to_vec_pretty(&self)?;
        create_file_with_content(Path::new(CLAIM_TEMP_PATH), &input)?;
        Ok(())
    }

    pub fn delete() -> anyhow::Result<()> {
        std::fs::remove_file(CLAIM_TEMP_PATH)?;
        Ok(())
    }
}