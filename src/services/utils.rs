use anyhow::ensure;
use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::Wallet,
    types::{Address, U256},
};

use crate::{
    cli::console::{print_status, print_warning},
    external_api::contracts::utils::{get_account_nonce, get_balance, get_client, get_gas_price},
    utils::{config::Settings, env_config::EnvConfig},
};

pub async fn handle_contract_call<S: ToString>(
    tx: ethers::contract::builders::ContractCall<
        SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
        (),
    >,
    from_address: Address,
    from_name: S,
    tx_name: S,
) -> anyhow::Result<()> {
    loop {
        let result = tx.send().await;
        match result {
            Ok(tx) => {
                let pending_tx = tx;
                print_status(format!(
                    "{} tx hash: {:?}",
                    tx_name.to_string(),
                    pending_tx.tx_hash()
                ));
                let tx_receipt = pending_tx.await?.unwrap();
                ensure!(
                    tx_receipt.status.unwrap() == 1.into(),
                    "{} tx failed",
                    from_name.to_string()
                );
                return Ok(());
            }
            Err(e) => {
                let error_message = e.to_string();
                // insufficient balance
                if error_message.contains("-32000") {
                    let estimate_gas = tx.estimate_gas().await?;
                    let gas_price = get_client().await?.get_gas_price().await?;
                    let value = tx.tx.value().cloned().unwrap_or_default();
                    let necessary_balance = estimate_gas * gas_price + value;
                    insuffient_balance_instruction(
                        from_address,
                        necessary_balance,
                        &from_name.to_string(),
                    )
                    .await?;
                    print_status(format!("Retrying {} transaction...", tx_name.to_string()));
                } else {
                    return Err(anyhow::anyhow!("Error sending transaction: {:?}", e));
                }
            }
        }
    }
}

pub async fn insuffient_balance_instruction(
    address: Address,
    required_balance: U256,
    name: &str,
) -> anyhow::Result<()> {
    let balance = get_balance(address).await?;
    if required_balance <= balance {
        return Ok(());
    }
    print_warning(format!(
        r"{} address {:?} has insufficient balance {} ETH < {} ETH. Waiting for your deposit...",
        name,
        address,
        pretty_format_u256(balance),
        pretty_format_u256(required_balance)
    ));
    loop {
        let new_balance = get_balance(address).await?;
        if new_balance > required_balance {
            print_status("Balance updated");
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
    Ok(())
}

pub async fn await_until_low_gas_price() -> anyhow::Result<()> {
    let max_gas_price = EnvConfig::import_from_env()?.max_gas_price;
    let settings = Settings::load()?;
    let high_gas_retry_inverval_in_sec = settings.service.high_gas_retry_inverval_in_sec;
    let _url = settings.service.repository_url;
    loop {
        let current_gas_price = get_gas_price().await?;
        if current_gas_price <= max_gas_price {
            log::info!(
                "Current gas price: {} GWei is lower than max gas price: {} GWei",
                ethers::utils::format_units(current_gas_price.clone(), "gwei").unwrap(),
                ethers::utils::format_units(max_gas_price.clone(), "gwei").unwrap(),
            );
            break;
        }
        print_warning(format!(
            "Current gas price: {} Gwei > max gas price: {} Gwei. Waiting for gas price to drop...",
            ethers::utils::format_units(current_gas_price.clone(), "gwei").unwrap(),
            ethers::utils::format_units(max_gas_price.clone(), "gwei").unwrap(),
        ));
        tokio::time::sleep(std::time::Duration::from_secs(
            high_gas_retry_inverval_in_sec,
        ))
        .await;
    }
    Ok(())
}

pub async fn is_address_used(deposit_address: Address) -> bool {
    get_account_nonce(deposit_address).await.unwrap() > 0
        || get_balance(deposit_address).await.unwrap() > 0.into()
}

pub fn pretty_format_u256(value: U256) -> String {
    let s = ethers::utils::format_units(value, "ether").unwrap();
    let s = s.trim_end_matches('0').trim_end_matches('.');
    s.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pretty_format() {
        let value = ethers::utils::parse_ether("1.01000000000000000").unwrap();
        let pretty = super::pretty_format_u256(value);
        assert_eq!(pretty, "1.01");

        let value = ethers::utils::parse_ether("1.00000000000000000").unwrap();
        let pretty = super::pretty_format_u256(value);
        assert_eq!(pretty, "1");
    }
}