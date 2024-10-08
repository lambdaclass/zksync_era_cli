use crate::config::ZKSyncConfig;
use clap::Subcommand;
use std::path::PathBuf;
use zksync_ethers_rs::types::zksync::inputs::WitnessInputData;

#[derive(Subcommand)]
pub(crate) enum Command {
    #[clap(
        about = "Prover - Debug Witness Inputs",
        visible_alias = "debug-proof-gen-data"
    )]
    DebugWitnessInputs {
        file_path: PathBuf,
        #[arg(long, default_value = "false", requires = "file_path")]
        vm_run_data: bool,
        #[arg(long, default_value = "false", requires = "file_path")]
        merkle_paths: bool,
        #[arg(long, default_value = "false", requires = "file_path")]
        previous_batch_metadata: bool,
        #[arg(long, default_value = "false", requires = "file_path")]
        eip_4844_blobs: bool,
    },
}

impl Command {
    pub async fn run(self, _cfg: ZKSyncConfig) -> eyre::Result<()> {
        match self {
            Command::DebugWitnessInputs {
                file_path,
                vm_run_data,
                merkle_paths,
                previous_batch_metadata,
                eip_4844_blobs,
            } => {
                let witness_inputs_bytes = std::fs::read(file_path)?;
                let witness_input_data: WitnessInputData =
                    bincode::deserialize(&witness_inputs_bytes)?;
                if vm_run_data && merkle_paths && previous_batch_metadata && eip_4844_blobs {
                    println!("{witness_input_data:?}");
                } else {
                    if vm_run_data {
                        println!("{:?}", witness_input_data.vm_run_data);
                    }
                    if merkle_paths {
                        println!("{:?}", witness_input_data.merkle_paths);
                    }
                    if previous_batch_metadata {
                        println!("{:?}", witness_input_data.previous_batch_metadata);
                    }
                    if eip_4844_blobs {
                        println!("{:?}", witness_input_data.eip_4844_blobs);
                    }
                }
            }
        }
        Ok(())
    }
}
