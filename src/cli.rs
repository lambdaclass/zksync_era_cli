use crate::{
    commands::{autocomplete, chain, config, contract, contracts, db, prover, test, utils, wallet},
    config::load_selected_config,
};
use clap::{Parser, Subcommand};

pub const VERSION_STRING: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name="zks", author, version=VERSION_STRING, about, long_about = None)]
pub struct ZKSyncCLI {
    #[command(subcommand)]
    command: ZKSyncCommand,
}

#[derive(Subcommand)]
enum ZKSyncCommand {
    #[clap(
        subcommand,
        about = "Wallet interaction commands. The configured wallet could operate both with the L1 and L2 networks.",
        visible_alias = "w"
    )]
    Wallet(wallet::Command),
    #[clap(
        subcommand,
        about = "Chain interaction commands. These make use of the JSON-RPC API."
    )]
    Chain(chain::Command),
    #[clap(subcommand, about = "Prover commands. TODO.")]
    Prover(prover::Command),
    #[clap(subcommand, about = "Contract interaction commands.")]
    Contract(contract::Command),
    #[clap(subcommand, about = "CLI config commands.")]
    Config(config::Command),
    #[clap(
        subcommand,
        about = "L1 Contracts interaction commands. For the chain owner.",
        visible_alias = "l1"
    )]
    Contracts(contracts::Command),
    #[clap(
        subcommand,
        about = "Test commands, LoadTests, Benchmarks, etc.",
        visible_alias = "t"
    )]
    Test(test::Command),
    #[clap(subcommand, about = "Generate shell completion scripts.")]
    Autocomplete(autocomplete::Command),
    #[clap(subcommand, about = "Utility commands.")]
    Utils(utils::Command),
    #[clap(
        subcommand,
        about = "Commands for interacting with the server and prover databases."
    )]
    Db(db::Command),
}

pub async fn start() -> eyre::Result<()> {
    let ZKSyncCLI { command } = ZKSyncCLI::parse();
    if let ZKSyncCommand::Config(cmd) = command {
        return cmd.run().await;
    }
    let cfg = load_selected_config().await?;
    match command {
        ZKSyncCommand::Wallet(cmd) => cmd.run(cfg).await?,
        ZKSyncCommand::Chain(cmd) => cmd.run(cfg).await?,
        ZKSyncCommand::Prover(cmd) => cmd.run(cfg).await?,
        ZKSyncCommand::Contract(cmd) => cmd.run(cfg)?,
        ZKSyncCommand::Contracts(cmd) => cmd.run(cfg).await?,
        ZKSyncCommand::Autocomplete(cmd) => cmd.run()?,
        ZKSyncCommand::Utils(cmd) => cmd.run(cfg)?,
        ZKSyncCommand::Config(_) => unreachable!(),
        ZKSyncCommand::Test(cmd) => cmd.run(cfg).await?,
        ZKSyncCommand::Db(cmd) => cmd.run(cfg).await?,
    };
    Ok(())
}
