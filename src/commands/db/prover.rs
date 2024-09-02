use crate::{
    config::ZKSyncConfig,
    utils::{
        config::{
            confirm,
            default_values::{
                DEFAULT_PROTOCOL_VERSION, DEFAULT_RECURSION_CIRCUITS_SET_VK_HASH,
                DEFAULT_RECURSION_LEAF_VK_HASH, DEFAULT_RECURSION_NODE_VK_HASH,
                DEFAULT_RECURSION_SCHEDULER_VK_HASH, DEFAULT_VERSION_PATCH,
            },
            prompt,
        },
        db::{
            prover::{
                find_map_stuck_wg_jobs_in_aggregation_round,
                find_stuck_prover_jobs_in_aggregation_round, map_bwg_info, map_leaf_wg_info,
                map_node_wg_info, map_recursion_tip_wg_info, map_scheduler_wg_info,
            },
            queries::{
                get_basic_witness_job_status, get_compressor_job_status,
                insert_prover_protocol_version, insert_witness_inputs, restart_batch_proof,
            },
        },
        messages::{
            DATABASE_PROVER_PROTOCOL_VERSION_PATCH_PROMPT_MSG,
            DATABASE_PROVER_PROTOCOL_VERSION_PROMPT_MSG,
            DATABASE_PROVER_RECURSION_CIRCUITS_SET_PROMPT_MSG,
            DATABASE_PROVER_RECURSION_LEAF_VK_HASH_PROMPT_MSG,
            DATABASE_PROVER_RECURSION_NODE_VK_HASH_PROMPT_MSG,
            DATABASE_PROVER_RECURSION_SCHEDULER_VK_HASH_PROMPT_MSG,
            DATABASE_PROVER_RESTART_ALREADY_PROVED_BATCH_PROOF_CONFIRMATION_MSG,
            DATABASE_PROVER_RESTART_BATCH_PROOF_CONFIRMATION_MSG,
        },
        prover_status::{get_batches_data, BatchData, StageInfo, Status},
    },
};
use clap::Subcommand;
use colored::Colorize;
use eyre::ContextCompat;
use spinoff::{spinners::Dots, Color, Spinner};
use zksync_ethers_rs::types::{
    zksync::{
        basic_fri_types::AggregationRound, protocol_version::VersionPatch,
        prover_dal::ProofCompressionJobStatus, L1BatchNumber, ProtocolVersionId,
    },
    TryFromPrimitive,
};

#[derive(Subcommand)]
pub(crate) enum Command {
    #[clap(about = "List all the stuck batch proofs.", visible_alias = "stuck")]
    StuckBatchProofs,
    #[clap(about = "Restart a batch proof.")]
    RestartBatchProof { batch_number: L1BatchNumber },
    #[clap(about = "Insert a batch proof.", visible_aliases = ["insert-witness", "insert-witness-inputs"])]
    InsertBatchWitnessInput {
        #[clap(index = 1)]
        batch_number: L1BatchNumber,
        #[clap(value_parser = |v: &str| ProtocolVersionId::try_from_primitive(v.parse::<u16>().expect("Invalid Protocol Version")), index = 2, default_value_t = ProtocolVersionId::default())]
        protocol_version: ProtocolVersionId,
        #[clap(index = 3)]
        protocol_version_patch: VersionPatch,
    },
    #[clap(
        about = "Insert a protocol version.",
        visible_alias = "protocol-version"
    )]
    InsertProtocolVersion {
        #[arg(short = 'd')]
        default_values: bool,
    },
    Status {
        #[clap(short = 'n', num_args = 1.., required = true)]
        batches: Vec<L1BatchNumber>,
        #[clap(short = 'v', long, default_value("false"))]
        verbose: bool,
    },
}

#[allow(unused, reason = "not used atm")]
fn protocol_version_from_str(s: &str) -> eyre::Result<ProtocolVersionId> {
    Ok(ProtocolVersionId::try_from_primitive(s.parse()?)?)
}

impl Command {
    pub async fn run(self, cfg: ZKSyncConfig) -> eyre::Result<()> {
        let mut prover_db = cfg
            .db
            .clone()
            .context("DB config missing")?
            .prover
            .acquire()
            .await?;
        match self {
            Command::StuckBatchProofs => {
                let mut aggregation_round = AggregationRound::BasicCircuits;
                find_map_stuck_wg_jobs_in_aggregation_round(
                    aggregation_round,
                    map_bwg_info,
                    &mut prover_db,
                )
                .await?;
                find_stuck_prover_jobs_in_aggregation_round(aggregation_round, &mut prover_db)
                    .await?;

                aggregation_round = AggregationRound::LeafAggregation;
                find_map_stuck_wg_jobs_in_aggregation_round(
                    aggregation_round,
                    map_leaf_wg_info,
                    &mut prover_db,
                )
                .await?;
                find_stuck_prover_jobs_in_aggregation_round(aggregation_round, &mut prover_db)
                    .await?;

                aggregation_round = AggregationRound::NodeAggregation;
                find_map_stuck_wg_jobs_in_aggregation_round(
                    aggregation_round,
                    map_node_wg_info,
                    &mut prover_db,
                )
                .await?;
                find_stuck_prover_jobs_in_aggregation_round(aggregation_round, &mut prover_db)
                    .await?;

                aggregation_round = AggregationRound::RecursionTip;
                find_map_stuck_wg_jobs_in_aggregation_round(
                    aggregation_round,
                    map_recursion_tip_wg_info,
                    &mut prover_db,
                )
                .await?;
                find_stuck_prover_jobs_in_aggregation_round(aggregation_round, &mut prover_db)
                    .await?;

                aggregation_round = AggregationRound::Scheduler;
                find_map_stuck_wg_jobs_in_aggregation_round(
                    aggregation_round,
                    map_scheduler_wg_info,
                    &mut prover_db,
                )
                .await?;
                find_stuck_prover_jobs_in_aggregation_round(aggregation_round, &mut prover_db)
                    .await?;
            }
            Command::RestartBatchProof { batch_number } => {
                let mut spinner = Spinner::new(Dots, "Checking batch proof status", Color::Blue);
                let compressor_job_status_for_batch =
                    get_compressor_job_status(batch_number, &mut prover_db).await?;
                if (matches!(
                    compressor_job_status_for_batch,
                    Some(ProofCompressionJobStatus::SentToServer)
                ) && confirm(
                    DATABASE_PROVER_RESTART_ALREADY_PROVED_BATCH_PROOF_CONFIRMATION_MSG,
                )?) || confirm(DATABASE_PROVER_RESTART_BATCH_PROOF_CONFIRMATION_MSG)?
                {
                    spinner.update_text("Restarting batch proof");
                    restart_batch_proof(batch_number, &mut prover_db).await?;
                    spinner.success("Batch proof restarted");
                } else {
                    spinner.info("Batch proof restart aborted");
                }
            }
            Command::InsertBatchWitnessInput {
                batch_number,
                protocol_version,
                protocol_version_patch,
            } => {
                let mut spinner = Spinner::new(
                    Dots,
                    "Checking batch proof basic witness generation status",
                    Color::Blue,
                );
                let basic_witness_job_status_for_batch =
                    get_basic_witness_job_status(batch_number, &mut prover_db).await?;
                if basic_witness_job_status_for_batch.is_some() {
                    spinner.warn(
                        "Batch proof already exists, you need to restart the batch proof to insert new witness inputs",
                    );
                    let mut spinner =
                        Spinner::new(Dots, "Checking batch proof compression status", Color::Blue);
                    let compressor_job_status_for_batch =
                        get_compressor_job_status(batch_number, &mut prover_db).await?;
                    let proof_was_sent_to_server = matches!(
                        compressor_job_status_for_batch,
                        Some(ProofCompressionJobStatus::SentToServer)
                    );
                    if proof_was_sent_to_server {
                        spinner.info("Batch proof is already sent to the server.");
                    } else {
                        spinner.success("Batch proof is not sent to the server.");
                    }
                    if (proof_was_sent_to_server
                        && confirm(
                            DATABASE_PROVER_RESTART_ALREADY_PROVED_BATCH_PROOF_CONFIRMATION_MSG,
                        )?)
                        || confirm(DATABASE_PROVER_RESTART_BATCH_PROOF_CONFIRMATION_MSG)?
                    {
                        let mut spinner = Spinner::new(Dots, "Restarting batch proof", Color::Blue);
                        restart_batch_proof(batch_number, &mut prover_db).await?;
                        spinner.success("Batch proof restarted");
                    } else {
                        spinner.info("Batch proof restart aborted");
                        return Ok(());
                    }
                }

                let mut spinner = Spinner::new(Dots, "Inserting witness inputs", Color::Blue);
                let witness_inputs_blob_url = format!("witness_inputs_{batch_number}.bin");
                match insert_witness_inputs(
                    batch_number,
                    &witness_inputs_blob_url,
                    protocol_version,
                    protocol_version_patch,
                    &mut prover_db,
                )
                .await
                {
                    Ok(_) => spinner.success("Batch proof inserted"),
                    Err(e) => {
                        spinner.fail("Batch proof insertion failed");
                        return Err(e);
                    }
                }
                return Ok(());
            }
            Command::InsertProtocolVersion { default_values } => {
                let protocol_version = if default_values {
                    ProtocolVersionId::default()
                } else {
                    ProtocolVersionId::try_from_primitive(prompt(
                        DATABASE_PROVER_PROTOCOL_VERSION_PROMPT_MSG,
                        DEFAULT_PROTOCOL_VERSION,
                    )?)?
                };
                let recursion_scheduler_vk_hash = if default_values {
                    DEFAULT_RECURSION_SCHEDULER_VK_HASH
                } else {
                    prompt(
                        DATABASE_PROVER_RECURSION_SCHEDULER_VK_HASH_PROMPT_MSG,
                        DEFAULT_RECURSION_SCHEDULER_VK_HASH,
                    )?
                };
                let recursion_node_vk_hash = if default_values {
                    DEFAULT_RECURSION_NODE_VK_HASH
                } else {
                    prompt(
                        DATABASE_PROVER_RECURSION_NODE_VK_HASH_PROMPT_MSG,
                        DEFAULT_RECURSION_NODE_VK_HASH,
                    )?
                };
                let recursion_leaf_vk_hash = if default_values {
                    DEFAULT_RECURSION_LEAF_VK_HASH
                } else {
                    prompt(
                        DATABASE_PROVER_RECURSION_LEAF_VK_HASH_PROMPT_MSG,
                        DEFAULT_RECURSION_LEAF_VK_HASH,
                    )?
                };
                let recursion_circuits_set_vk_hash = if default_values {
                    DEFAULT_RECURSION_CIRCUITS_SET_VK_HASH
                } else {
                    prompt(
                        DATABASE_PROVER_RECURSION_CIRCUITS_SET_PROMPT_MSG,
                        DEFAULT_RECURSION_CIRCUITS_SET_VK_HASH,
                    )?
                };
                let protocol_version_patch = if default_values {
                    DEFAULT_VERSION_PATCH
                } else {
                    prompt(
                        DATABASE_PROVER_PROTOCOL_VERSION_PATCH_PROMPT_MSG,
                        DEFAULT_VERSION_PATCH,
                    )?
                };

                let mut spinner = Spinner::new(Dots, "Inserting protocol version", Color::Blue);
                match insert_prover_protocol_version(
                    protocol_version,
                    recursion_scheduler_vk_hash,
                    recursion_node_vk_hash,
                    recursion_leaf_vk_hash,
                    recursion_circuits_set_vk_hash,
                    protocol_version_patch,
                    &mut prover_db,
                )
                .await
                {
                    Ok(_) => spinner.success("Protocol version inserted"),
                    Err(e) => {
                        spinner.fail("Protocol version insertion failed");
                        return Err(e);
                    }
                };
            }
            Command::Status { batches, verbose } => {
                let mut spinner = Spinner::new(Dots, "Getting Batch(es)", Color::Blue);
                let batches_data = get_batches_data(batches, &mut prover_db).await?;
                spinner.success("Data Retrieved from DB");

                for batch_data in batches_data {
                    println!(
                        "{} {} {}",
                        "=".repeat(8),
                        format!("Batch {:0>5} Status", batch_data.batch_number.0)
                            .bold()
                            .bright_cyan()
                            .on_black(),
                        "=".repeat(8)
                    );

                    if let Status::Custom(msg) =
                        batch_data.compressor.witness_generator_jobs_status(10)
                    {
                        if msg.contains("Sent to server") {
                            println!("> Proof sent to server ✅");
                            continue;
                        }
                    }

                    let basic_witness_generator_status = batch_data
                        .basic_witness_generator
                        .witness_generator_jobs_status(10);
                    if matches!(basic_witness_generator_status, Status::JobsNotFound) {
                        println!("> No batch found. 🚫");
                        continue;
                    }

                    if !verbose {
                        display_batch_status(batch_data);
                    } else {
                        //display_batch_info(batch_data);
                    }
                }
            }
        };

        Ok(())
    }
}

fn display_batch_status(batch_data: BatchData) {
    display_status_for_stage(batch_data.basic_witness_generator);
    display_status_for_stage(batch_data.leaf_witness_generator);
    display_status_for_stage(batch_data.node_witness_generator);
    display_status_for_stage(batch_data.recursion_tip_witness_generator);
    display_status_for_stage(batch_data.scheduler_witness_generator);
    display_status_for_stage(batch_data.compressor);
}

fn display_status_for_stage(stage_info: StageInfo) {
    let max_attempts = 10;
    display_aggregation_round(&stage_info);
    let status = stage_info.witness_generator_jobs_status(max_attempts);
    match status {
        Status::Custom(msg) => {
            println!("{}: {} \n", stage_info.to_string().bold(), msg);
        }
        Status::Queued | Status::WaitingForProofs | Status::Stuck | Status::JobsNotFound => {
            println!("{}: {}", stage_info.to_string().bold(), status)
        }
        Status::InProgress | Status::Successful => {
            println!("{}: {}", stage_info.to_string().bold(), status);
            if let Some(job_status) = stage_info.prover_jobs_status(max_attempts) {
                println!("> {}: {}", "Prover Jobs".to_owned().bold(), job_status);
            }
        }
    }
}

#[allow(clippy::as_conversions, reason = "AggregationRound is an enum of u8s")]
fn display_aggregation_round(stage_info: &StageInfo) {
    if let Some(aggregation_round) = stage_info.aggregation_round() {
        println!(
            "\n-- {} --",
            format!("Aggregation Round {}", aggregation_round as u8).bold()
        );
    } else {
        println!("\n-- {} --", "Proof Compression".to_owned().bold());
    };
}
