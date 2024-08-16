use std::sync::Arc;
use zksync_ethers_rs::{
    contracts::{erc20::ERC20, l2_shared_bridge::get_l2_token_from_l1_address},
    core::{
        k256::ecdsa::SigningKey,
        utils::{format_ether, format_units},
    },
    providers::{Http, Middleware, Provider},
    signers::Wallet,
    types::Address,
    utils::L2_ETH_TOKEN_ADDRESS,
    zk_wallet::ZKWallet,
    ZKMiddleware,
};

pub(crate) async fn get_erc20_balance_decimals_symbol(
    token_address: Address,
    wallet_address: Address,
    provider: &Provider<Http>,
) -> eyre::Result<(String, i32, String)> {
    let erc20 = ERC20::new(token_address, Arc::new(provider.clone()));
    let balance = erc20.balance_of(wallet_address).await?;
    let token_symbol = erc20.symbol().await?;
    let token_decimals: i32 = erc20.decimals().await?.into();
    let parsed_balance = format_units(balance, token_decimals)?;
    Ok((parsed_balance, token_decimals, token_symbol))
}

pub(crate) async fn print_l2_base_token_balance(
    base_token_address: Address,
    wallet_address: Address,
    l2_provider: &Provider<Http>,
    l1_provider: &Provider<Http>,
) -> eyre::Result<()> {
    let balance = l2_provider.get_balance(wallet_address, None).await?;
    if base_token_address != L2_ETH_TOKEN_ADDRESS {
        let (_, token_decimals, token_symbol) =
            get_erc20_balance_decimals_symbol(base_token_address, wallet_address, l1_provider)
                .await?;
        let parsed_balance = format_units(balance, token_decimals)?;
        println!("[L2] ({wallet_address:?}) Base Token Balance: {parsed_balance} {token_symbol}");
    } else {
        let parsed_balance = format_ether(balance);
        println!("[L2] ({wallet_address:?}) Base Token Balance: {parsed_balance} ETH");
    }
    Ok(())
}

pub(crate) async fn display_l1_balance(
    args_token_address: Option<Address>,
    l1_provider: &Provider<Http>,
    wallet_address: Address,
) -> eyre::Result<()> {
    if let Some(token_address) = args_token_address {
        let (parsed_balance, _, token_symbol) =
            get_erc20_balance_decimals_symbol(token_address, wallet_address, l1_provider).await?;
        println!("[L1] ({wallet_address:?}) Balance: {parsed_balance} {token_symbol}");
    } else {
        let balance = l1_provider.get_balance(wallet_address, None).await?;
        let parsed_balance = format_ether(balance);
        println!("[L1] ({wallet_address:?}) Balance: {parsed_balance} ETH");
    }
    Ok(())
}

pub(crate) async fn display_l2_balance(
    args_token_address: Option<Address>,
    l1_provider: &Provider<Http>,
    l2_provider: &Provider<Http>,
    wallet_address: Address,
    base_token_address: Address,
    args_l1: bool,
) -> eyre::Result<()> {
    if let Some(token_address) = args_token_address {
        let l2_token_address = match args_l1 {
            true => get_l2_token_from_l1_address(token_address, l2_provider).await,
            false => token_address,
        };
        if token_address == base_token_address {
            print_l2_base_token_balance(
                base_token_address,
                wallet_address,
                l2_provider,
                l1_provider,
            )
            .await?;
        } else {
            let (parsed_balance, _, token_symbol) =
                get_erc20_balance_decimals_symbol(l2_token_address, wallet_address, l2_provider)
                    .await?;
            println!("[L2] ({wallet_address:?}) Balance: {parsed_balance} {token_symbol}");
        }
    } else {
        print_l2_base_token_balance(base_token_address, wallet_address, l2_provider, l1_provider)
            .await?;
    }
    Ok(())
}

pub(crate) async fn display_balance(
    token: Option<Address>,
    wallet: &ZKWallet<&Provider<Http>, Wallet<SigningKey>>,
    from_l1: bool,
) -> eyre::Result<()> {
    let l1_provider = wallet.l1_provider();
    let wallet_address = wallet.l2_address();
    if !from_l1 {
        let l2_provider = wallet.l2_provider();
        let base_token_address = l2_provider.get_base_token_l1_address().await?;
        display_l2_balance(
            token,
            l1_provider,
            l2_provider,
            wallet_address,
            base_token_address,
            false,
        )
        .await?;
    } else {
        display_l1_balance(token, l1_provider, wallet_address).await?;
    };
    Ok(())
}
