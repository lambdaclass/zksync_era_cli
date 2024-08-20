use clap::Parser;
use eyre::ContextCompat;
use zksync_ethers_rs::{abi::Hash, contracts::governance::Governance, providers::Middleware};

#[derive(Parser, PartialEq)]
pub(crate) struct Args {
    pub operation_id: Hash,
}

pub(crate) async fn run(
    args: Args,
    governance: Governance<impl Middleware + 'static>,
) -> eyre::Result<()> {
    let transaction_receipt = governance
        .cancel(args.operation_id.into())
        .send()
        .await?
        .await?
        .context("No transaction receipt for operation cancel")?;
    println!(
        "Operation canceled: {:?}",
        transaction_receipt.transaction_hash
    );
    Ok(())
}