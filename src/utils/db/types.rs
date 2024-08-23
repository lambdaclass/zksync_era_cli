use chrono::{NaiveDateTime, NaiveTime};
use sqlx::{postgres::PgRow, FromRow, Row};
use std::str::FromStr;
use zksync_ethers_rs::types::{
    zksync::{
        basic_fri_types::AggregationRound,
        protocol_version::VersionPatch,
        prover_dal::{ProverJobStatus, WitnessJobStatus},
        L1BatchNumber, ProtocolVersionId,
    },
    U256,
};

#[derive(Debug, Clone)]
pub struct BasicWitnessGeneratorJobInfo {
    pub l1_batch_number: L1BatchNumber,
    pub _attempts: u32,
    pub _status: WitnessJobStatus,
    pub _error: Option<String>,
    pub _created_at: NaiveDateTime,
    pub _updated_at: NaiveDateTime,
    pub _processing_started_at: Option<NaiveDateTime>,
    pub _time_taken: Option<NaiveTime>,
    pub _protocol_version: Option<ProtocolVersionId>,
    pub _picked_by: Option<String>,
    pub _protocol_version_patch: Option<VersionPatch>,
    pub _witness_inputs_blob_url: Option<String>,
}

impl FromRow<'_, PgRow> for BasicWitnessGeneratorJobInfo {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            l1_batch_number: L1BatchNumber::from(get_u32_from_pg_row(row, "l1_batch_number")?),
            _attempts: get_u32_from_pg_row(row, "attempts")?,
            _status: get_witness_job_status_from_pg_row(row)?,
            _error: row.get("error"),
            _created_at: row.get("created_at"),
            _updated_at: row.get("updated_at"),
            _processing_started_at: row.get("processing_started_at"),
            _time_taken: row.get("time_taken"),
            _protocol_version: {
                let raw_protocol_version_id = row.get::<i16, &str>("protocol_version");
                ProtocolVersionId::try_from(U256::from(raw_protocol_version_id))
                    .map_err(|e| sqlx::Error::Decode(e.into()))
                    .ok()
            },
            _picked_by: row.get("picked_by"),
            _protocol_version_patch: get_version_path_from_pg_row(row).ok(),
            _witness_inputs_blob_url: row.get("witness_inputs_blob_url"),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ProverJobFriInfo {
    pub _id: u32,
    pub l1_batch_number: L1BatchNumber,
    pub _circuit_id: u32,
    pub _circuit_blob_url: String,
    pub _aggregation_round: AggregationRound,
    pub _sequence_number: u32,
    pub _status: ProverJobStatus,
    pub _error: Option<String>,
    pub _attempts: u8,
    pub _processing_started_at: Option<NaiveDateTime>,
    pub _created_at: NaiveDateTime,
    pub _updated_at: NaiveDateTime,
    pub _time_taken: Option<NaiveTime>,
    pub _depth: u32,
    pub _is_node_final_proof: bool,
    pub _proof_blob_url: Option<String>,
    pub _protocol_version: Option<ProtocolVersionId>,
    pub _picked_by: Option<String>,
    pub _protocol_version_patch: Option<VersionPatch>,
}

impl FromRow<'_, PgRow> for ProverJobFriInfo {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            _id: get_u32_from_pg_row(row, "id")?,
            l1_batch_number: L1BatchNumber::from(get_u32_from_pg_row(row, "l1_batch_number")?),
            _circuit_id: get_u32_from_pg_row(row, "circuit_id")?,
            _circuit_blob_url: row.get("circuit_blob_url"),
            _aggregation_round: {
                let raw_aggregation_round = row.get::<&str, &str>("aggregation_round");
                AggregationRound::from_str(raw_aggregation_round)
                    .map_err(|e| sqlx::Error::Decode(e.into()))?
            },
            _sequence_number: get_u32_from_pg_row(row, "sequence_number")?,
            _status: {
                let raw_status = row.get::<&str, &str>("status");
                ProverJobStatus::from_str(raw_status).map_err(|e| sqlx::Error::Decode(e.into()))?
            },
            _error: row.get("error"),
            _attempts: get_u8_from_pg_row(row, "attempts")?,
            _processing_started_at: row.get("processing_started_at"),
            _created_at: row.get("created_at"),
            _updated_at: row.get("updated_at"),
            _time_taken: row.get("time_taken"),
            _depth: get_u32_from_pg_row(row, "depth")?,
            _is_node_final_proof: row.get("is_node_final_proof"),
            _proof_blob_url: row.get("proof_blob_url"),
            _protocol_version: {
                let raw_protocol_version_id = row.get::<i16, &str>("protocol_version");
                ProtocolVersionId::try_from(U256::from(raw_protocol_version_id))
                    .map_err(|e| sqlx::Error::Decode(e.into()))
                    .ok()
            },
            _picked_by: row.get("picked_by"),
            _protocol_version_patch: get_version_path_from_pg_row(row).ok(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct LeafWitnessGeneratorJobInfo {
    pub _id: u32,
    pub l1_batch_number: L1BatchNumber,
    pub _circuit_id: u32,
    pub _closed_form_inputs_blob_url: Option<String>,
    pub _attempts: u32,
    pub _status: WitnessJobStatus,
    pub _error: Option<String>,
    pub _created_at: NaiveDateTime,
    pub _updated_at: NaiveDateTime,
    pub _processing_started_at: Option<NaiveDateTime>,
    pub _time_taken: Option<NaiveTime>,
    pub _number_of_basic_circuits: Option<i32>,
    pub _protocol_version: Option<i32>,
    pub _picked_by: Option<String>,
    pub _protocol_version_patch: Option<VersionPatch>,
}

impl FromRow<'_, PgRow> for LeafWitnessGeneratorJobInfo {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            _id: get_u32_from_pg_row(row, "id")?,
            l1_batch_number: L1BatchNumber::from(get_u32_from_pg_row(row, "l1_batch_number")?),
            _circuit_id: get_u32_from_pg_row(row, "circuit_id")?,
            _closed_form_inputs_blob_url: row.get("closed_form_inputs_blob_url"),
            _attempts: get_u32_from_pg_row(row, "attempts")?,
            _status: get_witness_job_status_from_pg_row(row)?,
            _error: row.get("error"),
            _created_at: row.get("created_at"),
            _updated_at: row.get("updated_at"),
            _processing_started_at: row.get("processing_started_at"),
            _time_taken: row.get("time_taken"),
            _number_of_basic_circuits: row.get("number_of_basic_circuits"),
            _protocol_version: row.get("protocol_version"),
            _picked_by: row.get("picked_by"),
            _protocol_version_patch: get_version_path_from_pg_row(row).ok(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct NodeWitnessGeneratorJobInfo {
    pub _id: u32,
    pub l1_batch_number: L1BatchNumber,
    pub _circuit_id: u32,
    pub _depth: u32,
    pub _status: WitnessJobStatus,
    pub _attempts: u32,
    pub _aggregations_url: Option<String>,
    pub _processing_started_at: Option<NaiveDateTime>,
    pub _time_taken: Option<NaiveTime>,
    pub _error: Option<String>,
    pub _created_at: NaiveDateTime,
    pub _updated_at: NaiveDateTime,
    pub _number_of_dependent_jobs: Option<i32>,
    pub _protocol_version: Option<i32>,
    pub _picked_by: Option<String>,
    pub _protocol_version_patch: Option<VersionPatch>,
}

impl FromRow<'_, PgRow> for NodeWitnessGeneratorJobInfo {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            _id: get_u32_from_pg_row(row, "id")?,
            l1_batch_number: L1BatchNumber::from(get_u32_from_pg_row(row, "l1_batch_number")?),
            _circuit_id: get_u32_from_pg_row(row, "circuit_id")?,
            _depth: get_u32_from_pg_row(row, "depth")?,
            _status: get_witness_job_status_from_pg_row(row)?,
            _attempts: get_u32_from_pg_row(row, "attempts")?,
            _aggregations_url: row.get("aggregations_url"),
            _processing_started_at: row.get("processing_started_at"),
            _time_taken: row.get("time_taken"),
            _error: row.get("error"),
            _created_at: row.get("created_at"),
            _updated_at: row.get("updated_at"),
            _number_of_dependent_jobs: row.get("number_of_dependent_jobs"),
            _protocol_version: row.get("protocol_version"),
            _picked_by: row.get("picked_by"),
            _protocol_version_patch: get_version_path_from_pg_row(row).ok(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RecursionTipWitnessGeneratorJobInfo {
    pub l1_batch_number: L1BatchNumber,
    pub _status: WitnessJobStatus,
    pub _attempts: u32,
    pub _processing_started_at: Option<NaiveDateTime>,
    pub _time_taken: Option<NaiveTime>,
    pub _error: Option<String>,
    pub _created_at: NaiveDateTime,
    pub _updated_at: NaiveDateTime,
    pub _number_of_final_node_jobs: Option<i32>,
    pub _protocol_version: Option<i32>,
    pub _picked_by: Option<String>,
    pub _protocol_version_patch: Option<VersionPatch>,
}

impl FromRow<'_, PgRow> for RecursionTipWitnessGeneratorJobInfo {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            l1_batch_number: L1BatchNumber::from(get_u32_from_pg_row(row, "l1_batch_number")?),
            _status: get_witness_job_status_from_pg_row(row)?,
            _attempts: get_u32_from_pg_row(row, "attempts")?,
            _processing_started_at: row.get("processing_started_at"),
            _time_taken: row.get("time_taken"),
            _error: row.get("error"),
            _created_at: row.get("created_at"),
            _updated_at: row.get("updated_at"),
            _number_of_final_node_jobs: row.get("number_of_final_node_jobs"),
            _protocol_version: row.get("protocol_version"),
            _picked_by: row.get("picked_by"),
            _protocol_version_patch: get_version_path_from_pg_row(row).ok(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SchedulerWitnessGeneratorJobInfo {
    pub l1_batch_number: L1BatchNumber,
    pub _scheduler_partial_input_blob_url: String,
    pub _status: WitnessJobStatus,
    pub _processing_started_at: Option<NaiveDateTime>,
    pub _time_taken: Option<NaiveTime>,
    pub _error: Option<String>,
    pub _created_at: NaiveDateTime,
    pub _updated_at: NaiveDateTime,
    pub _attempts: u32,
    pub _protocol_version: Option<i32>,
    pub _picked_by: Option<String>,
    pub _protocol_version_patch: Option<VersionPatch>,
}

impl FromRow<'_, PgRow> for SchedulerWitnessGeneratorJobInfo {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            l1_batch_number: L1BatchNumber::from(get_u32_from_pg_row(row, "l1_batch_number")?),
            _scheduler_partial_input_blob_url: row.get("scheduler_partial_input_blob_url"),
            _status: get_witness_job_status_from_pg_row(row)?,
            _processing_started_at: row.get("processing_started_at"),
            _time_taken: row.get("time_taken"),
            _error: row.get("error"),
            _created_at: row.get("created_at"),
            _updated_at: row.get("updated_at"),
            _attempts: get_u32_from_pg_row(row, "attempts")?,
            _protocol_version: row.get("protocol_version"),
            _picked_by: row.get("picked_by"),
            _protocol_version_patch: get_version_path_from_pg_row(row).ok(),
        })
    }
}

fn get_u32_from_pg_row(row: &PgRow, index: &str) -> Result<u32, sqlx::Error> {
    let raw_u32: Result<u32, _> = row.get::<i16, &str>(index).try_into();
    raw_u32.map_err(|e| sqlx::Error::Decode(e.into()))
}

fn get_u8_from_pg_row(row: &PgRow, index: &str) -> Result<u8, sqlx::Error> {
    let raw_u8: Result<u8, _> = row.get::<i8, &str>(index).try_into();
    raw_u8.map_err(|e| sqlx::Error::Decode(e.into()))
}

fn get_version_path_from_pg_row(row: &PgRow) -> Result<VersionPatch, sqlx::Error> {
    let raw_version_path = row.get::<&str, &str>("protocol_version_patch");
    VersionPatch::from_str(raw_version_path).map_err(|e| sqlx::Error::Decode(e.into()))
}

fn get_witness_job_status_from_pg_row(row: &PgRow) -> Result<WitnessJobStatus, sqlx::Error> {
    let raw_status = row.get::<&str, &str>("status");
    WitnessJobStatus::from_str(raw_status).map_err(|e| sqlx::Error::Decode(e.into()))
}