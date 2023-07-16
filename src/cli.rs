use crate::commands::{
    account_balance, call, compile, deploy, encode, get_bridge_contracts, get_bytecode_by_hash,
    get_contract, get_transaction, pay, selector, AccountBalance, Call, CompileArgs, Deploy,
    EncodeArgs, GetBytecodeByHashArgs, GetContract, GetTransaction, Pay, SelectorArgs,
};
use clap::{command, Args, Parser, Subcommand};

pub const VERSION_STRING: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name="zksync-web3-rs", author, version=VERSION_STRING, about, long_about = None)]
struct ZKSyncWeb3 {
    #[command(subcommand)]
    command: ZKSyncWeb3Command,
    #[clap(flatten)]
    config: ZKSyncWeb3Config,
}

#[derive(Args)]
pub struct ZKSyncWeb3Config {
    #[clap(long, default_value = "localhost")]
    pub host: String,
    #[clap(short, long, default_value = "3050")]
    pub port: u16,
}

#[derive(Subcommand)]
enum ZKSyncWeb3Command {
    Deploy(Deploy),
    Call(Call),
    GetContract(GetContract),
    GetTransaction(GetTransaction),
    Balance(AccountBalance),
    Pay(Pay),
    Compile(CompileArgs),
    Encode(EncodeArgs),
    Selector(SelectorArgs),
    GetBridgeContracts,
    GetBytecodeByHash(GetBytecodeByHashArgs),
}

pub async fn start() -> eyre::Result<()> {
    let ZKSyncWeb3 { command, config } = ZKSyncWeb3::parse();
    match command {
        ZKSyncWeb3Command::Deploy(args) => deploy::run(args, config).await?,
        ZKSyncWeb3Command::Call(args) => call::run(args, config).await?,
        ZKSyncWeb3Command::GetContract(args) => get_contract::run(args, config).await?,
        ZKSyncWeb3Command::GetTransaction(args) => get_transaction::run(args, config).await?,
        ZKSyncWeb3Command::Balance(args) => account_balance::run(args, config).await?,
        ZKSyncWeb3Command::Pay(args) => pay::run(args, config).await?,
        ZKSyncWeb3Command::Compile(args) => {
            let _ = compile::run(args)?;
        }
        ZKSyncWeb3Command::Encode(args) => encode::run(args).await?,
        ZKSyncWeb3Command::Selector(args) => selector::run(args).await?,
        ZKSyncWeb3Command::GetBridgeContracts => get_bridge_contracts::run(config).await?,
        ZKSyncWeb3Command::GetBytecodeByHash(args) => {
            get_bytecode_by_hash::run(args, config).await?
        }
    };

    Ok(())
}
