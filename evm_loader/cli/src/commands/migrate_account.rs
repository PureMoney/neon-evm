#[allow(unused)]
use log::{error, info, debug};

use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    transaction::Transaction,
};

use solana_cli::{
    checks::{check_account_for_fee},
};

use evm::{H160};

use crate::{
    Config,
    NeonCliError,
    NeonCliResult,
};

/// Executes subcommand `migrate-account`.
#[allow(clippy::unnecessary_wraps)]
pub fn execute(
    config: &Config,
    ether_address: &H160,
) -> NeonCliResult {
    let (ether_pubkey, nonce) = crate::make_solana_program_address(ether_address, &config.evm_loader);

    let ether_account = config.rpc_client.get_account(&ether_pubkey)
        .map_err(|e| {
            error!("{}", e);
            NeonCliError::AccountNotFoundAtAddress(*ether_address)
        })?;
    dbg!(ether_account);

    let token_mint_id = evm_loader::config::token_mint::id();
    let ether_token_pubkey =
        spl_associated_token_account::get_associated_token_address(&ether_pubkey, &token_mint_id);

    let instructions = vec![
        migrate_account_instruction(
            config,
            ether_pubkey,
            ether_token_pubkey,
    )];

    let finalize_message = Message::new(&instructions, Some(&config.signer.pubkey()));
    let (blockhash, fee_calculator) = config.rpc_client.get_recent_blockhash()?;

    check_account_for_fee(
        &config.rpc_client,
        &config.signer.pubkey(),
        &fee_calculator,
        &finalize_message)?;

    let mut finalize_tx = Transaction::new_unsigned(finalize_message);

    finalize_tx.try_sign(&[&*config.signer], blockhash)?;
    debug!("signed: {:x?}", finalize_tx);

    config.rpc_client.send_and_confirm_transaction_with_spinner(&finalize_tx)?;

    info!("{}", serde_json::json!({
        "ether address": hex::encode(ether_address),
        "nonce": nonce,
    }));

    Ok(())
}

/// Returns instruction to migrate Ethereum account.
fn migrate_account_instruction(
    config: &Config,
    ether_pubkey: Pubkey,
    ether_token_pubkey: Pubkey,
) -> Instruction {
    Instruction::new_with_bincode(
        config.evm_loader,
        &(26_u8),
        vec![
            AccountMeta::new(config.signer.pubkey(), true),
            AccountMeta::new(ether_pubkey, false),
            AccountMeta::new(ether_token_pubkey, false),
        ],
    )
}
