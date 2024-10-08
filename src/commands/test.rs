use crate::config::ZKSyncConfig;
use crate::utils::{
    balance::display_balance, balance::get_erc20_decimals_symbol, gas_tracker::GasTracker, test::*,
    wallet::*,
};
use clap::Subcommand;
use colored::*;
use core::time;
use eyre::ContextCompat;
use spinoff::{spinners, Color, Spinner};
use std::{
    ops::{Add, Div},
    sync::Arc,
    thread::sleep,
};
use zksync_ethers_rs::{
    core::utils::parse_units, providers::Middleware, types::Address, types::U256,
    wait_for_finalize_withdrawal, ZKMiddleware,
};

#[derive(Subcommand)]
pub(crate) enum Command {
    #[clap(about = "LoadTest the zkStack Chain.", visible_alias = "lt")]
    LoadTest {
        #[clap(long = "wallets", short = 'w', required = true)]
        number_of_wallets: u16,
        #[clap(
            long = "amount",
            short = 'a',
            required = true,
            help = "Amount of BaseToken to deposit, 20% more will be deposited.\nThat extra 20% will remain in the main wallet,\nthe rest will be redistributed to random wallets"
        )]
        amount: f32,
        #[clap(
            long = "reruns",
            short = 'r',
            help = "If set to 0 it will run indefinitely, If not set defaults to 1 run."
        )]
        reruns_wanted: Option<u8>,
        #[arg(
            long = "withdraw",
            default_value_t = false,
            help = "If set, the funds will be withdrawn after each run."
        )]
        withdraw: bool,
        #[arg(
            long = "sleep",
            short = 's',
            default_value_t = 1,
            help = "Sleep interval between each rerun"
        )]
        sleep_secs: u64,
    },
    #[clap(
        about = "Gas Measurements for the zkStack Chain.",
        visible_alias = "gs"
    )]
    GasScenario {
        #[clap(long = "tpr", required = true, help = "Transactions per run")]
        tpr: u64,
        #[clap(
            long = "amount",
            short = 'a',
            required = true,
            help = "Amount of BaseToken to deposit, 20% more will be deposited.\nThat extra 20% will remain in the main wallet,\nthe rest will be redistributed to random wallets"
        )]
        amount: f32,
        #[arg(
            long = "reruns",
            short = 'r',
            default_value_t = 1,
            help = "Amount of times to run the program in a loop, it defaults to 1. Max is 255"
        )]
        reruns_wanted: u8,
    },
    #[clap(
        about = "LoadTest with contract interactions for the zkStack Chain.\nCustom command, the contract performs a fibonacci calculation and stores the value.",
        visible_alias = "ci"
    )]
    ContractInteraction {
        #[clap(long = "tpr", required = true, help = "Transactions per run")]
        tpr: u64,
        #[arg(
            long = "contract",
            short = 'c',
            required = true,
            help = "Contract Address, Make sure it follows the Custom Fibonacci Contract"
        )]
        contract_address: Address,
        #[arg(
            long = "reruns",
            short = 'r',
            default_value_t = 1,
            help = "Amount of times to run the program in a loop, it defaults to 1. Max is 255"
        )]
        reruns_wanted: u8,
    },
}

impl Command {
    pub async fn run(self, cfg: ZKSyncConfig) -> eyre::Result<()> {
        let (zk_wallet, l1_provider, l2_provider) = get_wallet_l1_l2_providers(cfg)?;
        let base_token_address = l2_provider.get_base_token_l1_address().await?;
        let (base_token_decimals, base_token_symbol) =
            get_erc20_decimals_symbol(base_token_address, &l1_provider).await?;

        let mut reruns = 0;
        let mut current_reruns: u32 = 1;

        let arc_zk_wallet = Arc::new(zk_wallet);

        match self {
            Command::LoadTest {
                number_of_wallets,
                amount,
                reruns_wanted,
                withdraw,
                sleep_secs,
            } => {
                let wallets =
                    get_n_random_wallets(number_of_wallets, &l1_provider, &l2_provider).await?;
                // ideally it should be the amount transferred, the gas + fees have to be deducted automatically
                let parsed_amount_to_deposit: U256 =
                    parse_units(amount, base_token_decimals)?.into();
                let parsed_amount_to_withdraw = parsed_amount_to_deposit;
                let parsed_amount_to_deposit = parsed_amount_to_deposit
                    .div(10_u32)
                    .saturating_mul(U256::from(12_u32)); // 20% of headroom
                let float_wallets: f32 = number_of_wallets.into();
                let amount_of_bt_to_transfer_for_each: f32 = amount / float_wallets;
                let parsed_amount_of_bt_to_transfer_for_each: U256 =
                    parse_units(amount_of_bt_to_transfer_for_each, base_token_decimals)?.into();

                // Begin Display L1 Balance and BaseToken Addr
                println!("{}", "#".repeat(64));
                println!(
                    "{}: {base_token_address:?}",
                    "Base Token Address".bold().green().on_black()
                );
                display_balance(None, &arc_zk_wallet, true, false).await?;
                display_balance(Some(base_token_address), &arc_zk_wallet, true, false).await?;

                println!("{}", "#".repeat(64));
                // End Display L1 Balance and BaseToken Addr

                let reruns_wanted = reruns_wanted.unwrap_or(1);
                let reruns_to_complete = if reruns_wanted == 0 { 1 } else { reruns_wanted };

                println!(
                    "Number of reruns {}",
                    if reruns_wanted == 0 {
                        "∞".to_owned().red()
                    } else {
                        reruns_wanted.to_string().red()
                    }
                );

                while reruns < reruns_to_complete {
                    check_balance_and_deposit_or_mint(
                        Arc::clone(&arc_zk_wallet),
                        base_token_address,
                        parsed_amount_to_deposit,
                    )
                    .await?;

                    println!(
                        "\n{} N: {}\n",
                        "Run".red().on_black(),
                        (current_reruns).to_string().yellow().on_black()
                    );

                    // Begin Transfer from rich wallet to each wallet

                    send_transactions(
                        &arc_zk_wallet,
                        &wallets,
                        parsed_amount_of_bt_to_transfer_for_each,
                    )
                    .await?;

                    display_balances(&wallets).await?;

                    // End Transfer from rich wallet to each wallet
                    println!("{}", "#".repeat(64));
                    // Begin Transfer from each wallet to rich wallet

                    display_balance(None, &arc_zk_wallet, false, true).await?;

                    send_transactions_back(&wallets, &arc_zk_wallet).await?;

                    display_balance(None, &arc_zk_wallet, false, true).await?;

                    // End Transfer from each wallet to rich wallet
                    println!("{}", "#".repeat(64));

                    if withdraw {
                        // Begin Withdrawal
                        println!(
                            "{} Withdraw basetoken from {} wallet.",
                            "[L2->L1]".bold().bright_cyan().on_black(),
                            "rich".bold().red().on_black(),
                        );

                        display_balance(Some(base_token_address), &arc_zk_wallet, true, true)
                            .await?;
                        let withdraw_hash = arc_zk_wallet
                            .withdraw_base_token(parsed_amount_to_withdraw)
                            .await?;
                        println!("Withdraw hash: {withdraw_hash:?}");
                        let base_token_address =
                            Some(l2_provider.get_base_token_l1_address().await?);
                        println!("finalize withdrawal");
                        wait_for_finalize_withdrawal(withdraw_hash, &l2_provider).await;
                        arc_zk_wallet.finalize_withdraw(withdraw_hash).await?;
                        display_balance(base_token_address, &arc_zk_wallet, true, true).await?;
                        println!("{}", "#".repeat(64));
                        // End Withdrawal
                    }

                    if reruns_wanted != 0 {
                        reruns += 1;
                    }
                    current_reruns += 1;

                    let mut spinner = Spinner::new(
                        spinners::Dots,
                        format!("Waiting for {sleep_secs} second(s)"),
                        Color::Blue,
                    );
                    sleep(time::Duration::from_secs(sleep_secs));
                    spinner.success(&format!("Rerun {current_reruns} finished"));
                }
                Ok(())
            }
            Command::GasScenario {
                tpr,
                amount,
                reruns_wanted,
            } => {
                // Calculations are performed with the following conditions
                // - Don't take deposits into account
                // - 1 transaction from the rich wallet to each random wallet
                // - 1 transaction from each random wallet to the rich wallet
                // - sleep 300 miliseconds == 0.3[s] . Between each run.
                //  - the only variable is the amount of random wallets
                //  - taking into account we will have 2*number_of_wallets transactions
                //  - the number_of_wallets is calculated as follows: 2*number_of_wallets [txs] = tpr [tx/run]
                //  - so number_of_wallets = (tpr+1)/2. The +1 is to have an even number from an odd number. An even number of tps is preffered.
                let number_of_wallets = ((tpr + 1) / 2).try_into()?;

                let mut gas_tracker = GasTracker::new()
                    .set_token_decimals(base_token_decimals)
                    .set_token_symbol(base_token_symbol);

                let mut txs_per_run;
                let mut fees_ra_per_run;
                let mut gas_ra_per_run;
                let mut gas_ra_price_per_run;

                let wallets =
                    get_n_random_wallets(number_of_wallets, &l1_provider, &l2_provider).await?;
                // ideally it should be the amount transferred, the gas + fees have to be deducted automatically
                let parsed_amount_to_deposit: U256 =
                    parse_units(amount, base_token_decimals)?.into();
                let parsed_amount_to_deposit = parsed_amount_to_deposit
                    .div(10_u32)
                    .saturating_mul(U256::from(12_u32)); // 20% of headroom
                let float_wallets: f32 = number_of_wallets.into();
                let amount_of_bt_to_transfer_for_each: f32 = amount / float_wallets;

                let parsed_amount_of_bt_to_transfer_for_each =
                    parse_units(amount_of_bt_to_transfer_for_each, base_token_decimals)?;

                let reruns_to_complete: u8 = if reruns_wanted == 0 { 1 } else { reruns_wanted };

                while reruns < reruns_to_complete {
                    fees_ra_per_run = U256::zero();
                    gas_ra_per_run = U256::zero();
                    gas_ra_price_per_run = U256::zero();

                    check_balance_and_deposit_or_mint(
                        Arc::clone(&arc_zk_wallet),
                        base_token_address,
                        parsed_amount_to_deposit,
                    )
                    .await?;

                    println!(
                        "\n{} N: {}\n",
                        "Run".red().on_black(),
                        (current_reruns).to_string().yellow().on_black()
                    );

                    // Begin Transfer from rich wallet to each wallet
                    let tx_hashes_forwards = send_transactions(
                        &arc_zk_wallet,
                        &wallets,
                        parsed_amount_of_bt_to_transfer_for_each.into(),
                    )
                    .await?;

                    display_balances(&wallets).await?;

                    // End Transfer from rich wallet to each wallet
                    println!("{}", "#".repeat(64));
                    // Begin Transfer from each wallet to rich wallet

                    display_balance(None, &arc_zk_wallet, false, true).await?;

                    let tx_hashes_backwards =
                        send_transactions_back(&wallets, &arc_zk_wallet).await?;

                    // End Transfer from each wallet to rich wallet
                    println!("{}", "#".repeat(64));

                    let mut tx_hashes = tx_hashes_forwards.clone();
                    tx_hashes.extend(&tx_hashes_backwards);
                    txs_per_run = tx_hashes.len().try_into()?;

                    for h in tx_hashes {
                        let receipt = l2_provider
                            .get_transaction_receipt(h)
                            .await?
                            .context("Error unwrapping tx_receipt")?;

                        let gas_used = receipt.gas_used.context("Error unwrapping gas_used")?;
                        let receipt_gas_price = receipt
                            .effective_gas_price
                            .context("Error unwrapping gas price")?;

                        let details = l2_provider
                            .get_transaction_details(h)
                            .await?
                            .context("Error unwrapping tx_details")?;

                        // Implementing running average calculation
                        if gas_ra_per_run.is_zero() {
                            gas_ra_per_run = gas_used;
                        }
                        gas_ra_per_run = gas_ra_per_run.add(gas_used) / 2_u32;
                        if fees_ra_per_run.is_zero() {
                            fees_ra_per_run = details.fee;
                        }
                        fees_ra_per_run = fees_ra_per_run.add(details.fee) / 2_u32;
                        if gas_ra_price_per_run.is_zero() {
                            gas_ra_price_per_run = receipt_gas_price;
                        }
                        gas_ra_price_per_run = (receipt_gas_price + gas_ra_price_per_run) / 2_u32;
                    }

                    gas_tracker.add_run(
                        gas_ra_per_run,
                        fees_ra_per_run,
                        gas_ra_price_per_run,
                        txs_per_run,
                    );

                    if reruns_wanted != 0 {
                        reruns += 1;
                    }
                    current_reruns += 1;

                    sleep(time::Duration::from_millis(300));
                }
                println!("{gas_tracker}");
                Ok(())
            }
            Command::ContractInteraction {
                tpr,
                contract_address,
                reruns_wanted,
            } => {
                let reruns_to_complete: u8 = if reruns_wanted == 0 { 1 } else { reruns_wanted };

                // The amount is a high value to avoid gas calculations and keep the test simple enough.
                let parsed_amount_to_deposit: U256 = parse_units("50", base_token_decimals)?.into();
                while reruns < reruns_to_complete {
                    check_balance_and_deposit_or_mint(
                        Arc::clone(&arc_zk_wallet),
                        base_token_address,
                        parsed_amount_to_deposit,
                    )
                    .await?;

                    println!(
                        "\n{} N: {}\n",
                        "Run".red().on_black(),
                        (current_reruns).to_string().yellow().on_black()
                    );

                    // Begin Contract Interaction
                    send_contract_transactions_for_test(&arc_zk_wallet, contract_address, tpr)
                        .await?;
                    // End Contract Interaction
                    println!("{}", "#".repeat(64));

                    if reruns_wanted != 0 {
                        reruns += 1;
                    }
                    current_reruns += 1;

                    sleep(time::Duration::from_millis(300));
                }
                Ok(())
            }
        }
    }
}
