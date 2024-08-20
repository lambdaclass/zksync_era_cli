use clap::Parser;
use zksync_ethers_rs::{abi::Hash, contracts::governance::Governance, providers::Middleware};

#[derive(Parser, PartialEq)]
pub(crate) struct Args {
    pub operation_id: Hash,
}

pub(crate) async fn run(
    args: Args,
    governance: Governance<impl Middleware + 'static>,
) -> eyre::Result<()> {
    let is_operation_done = governance
        .is_operation_done(args.operation_id.into())
        .call()
        .await?;
    println!(
        "Operation is {:?}",
        if is_operation_done { "done" } else { "pending" }
    );
    Ok(())
}