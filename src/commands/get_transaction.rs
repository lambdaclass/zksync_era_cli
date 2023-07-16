use crate::cli::ZKSyncConfig;
use clap::Args as ClapArgs;
use eyre::ContextCompat;
use zksync_web3_rs::{
    providers::{Middleware, Provider},
    types::H256,
};

#[derive(ClapArgs)]
pub(crate) struct Args {
    #[clap(short, long, name = "TRANSACTION_HASH")]
    pub transaction: H256,
}

pub(crate) async fn run(args: Args, config: ZKSyncConfig) -> eyre::Result<()> {
    let provider = Provider::try_from(format!(
        "http://{host}:{port}",
        host = config.host,
        port = config.port
    ))?
    .interval(std::time::Duration::from_millis(10));
    let transaction = provider
        .get_transaction(args.transaction)
        .await?
        .context("No pending transaction")?;
    log::info!("{:#?}", transaction);
    Ok(())
}
