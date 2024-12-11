mod errors;
mod structs;

use anchor_lang::{prelude::*, solana_program::sysvar::clock::Clock};
use errors::*;
use structs::*;

declare_id!("2LnQChqZcx8NKpNn3L5SmPmtz4tt6B2Wo31bY25sgRGs");

#[program]
pub mod token_faucet {
    use anchor_lang::solana_program;

    use super::*;

    pub fn initialize_faucet_pda(ctx: Context<InitFaucetPDA>) -> Result<()> {
        let faucet_pda = &mut ctx.accounts.faucet_pda;

        faucet_pda.max_distribution_amount = 2_000_000_000;
        faucet_pda.cooldown_time = 600;
        faucet_pda.last_request_time = 0;
        faucet_pda.total_dispensed = 0;
        faucet_pda.is_active = true;

        msg!("Faucet PDA initialized successfully");
        Ok(())
    }

    pub fn initialize_recipient_pda(ctx: Context<InitRecipientPDA>) -> Result<()> {
        let recipient_pda = &mut ctx.accounts.recipient_pda;

        recipient_pda.total_dispensed = 0;
        recipient_pda.last_request_time = 0;

        msg!("Recipient PDA initialized successfully");
        Ok(())
    }

    pub fn transfer_sol(ctx: Context<TransferSol>, amount: u64) -> Result<()> {
        let faucet_pda = &mut ctx.accounts.faucet_pda;
        let recipient_pda = &mut ctx.accounts.recipient_pda;

        // ensure faucet is active
        require!(faucet_pda.is_active, Errors::FaucetInactive);

        // ensure requested amount doesn't exceed distribution limit
        require!(
            amount <= faucet_pda.max_distribution_amount,
            Errors::AmountExceedsDistributionLimit
        );

        // fetch the current time using clock sysvar
        let current_time = Clock::get()?.unix_timestamp as u64;

        // check the cooldown
        require!(
            current_time >= recipient_pda.last_request_time + faucet_pda.cooldown_time,
            Errors::CooldownTimeNotOver
        );

        // update the faucet pda's state
        faucet_pda.total_dispensed += amount;
        faucet_pda.last_request_time = current_time;

        // update the reciepient pda's state
        recipient_pda.last_request_time = current_time;
        recipient_pda.total_dispensed += amount;

        // instruction to perform sol transfer
        let instruction = solana_program::system_instruction::transfer(
            &ctx.accounts.faucet_account.to_account_info().key,
            &ctx.accounts.recipient.to_account_info().key,
            amount,
        );

        // invoke instruction
        solana_program::program::invoke(
            &instruction,
            &[
                ctx.accounts.faucet_account.to_account_info(),
                ctx.accounts.recipient.to_account_info(),
            ],
        )?;

        msg!("Faucet transfer successful. PDAs state updated successfully");
        Ok(())
    }
}
