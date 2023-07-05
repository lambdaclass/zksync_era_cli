use crate::cli::ZKSyncWeb3Config;
use clap::Args;
use zksync_web3_rs::{
    providers::{Middleware, Provider},
    types::Address,
};

#[derive(Args)]
pub(crate) struct AccountBalance {
    #[clap(short, long, name = "ACCOUNT_ADDRESS")]
    pub account: Address,
}

pub(crate) async fn run(args: AccountBalance, config: ZKSyncWeb3Config) -> eyre::Result<()> {
    let provider = Provider::try_from(format!(
        "http://{host}:{port}",
        host = config.host,
        port = config.port
    ))?
    .interval(std::time::Duration::from_millis(10));
    let balance = provider.get_balance(args.account, None).await?;
    log::info!("{:#?}", balance);
    Ok(())
}
