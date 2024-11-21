use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer as SplTransfer},
};
use std::mem::size_of;

use crate::state::job_contract::*;
use crate::constants::{
    CONTRACT_SEED,
    PAY_TOKEN_MINT_ADDRESS,
};
use crate::errors::{
    GigContractError,
};

pub fn job_listing_with_fees_employer(
    ctx: Context<JobListingWithFeesEmployerContext>,
    contract_id: String,
    with_dispute: bool,
) -> Result<()> {
    msg!("Listing Job with $1 fee on employer side!");

    let contract = &mut ctx.accounts.contract;

    // Check if the signer is the correct employer
    require_keys_eq!(ctx.accounts.employer.key(), contract.employer, GigContractError::InvalidActivator);

    // Check if the contract is not ended.
    // require!(contract.status != JobContractStatus::Created, GigContractError::HourlyContractEnded);

    // Define the fees
    let listing_fee = 1_000_000; // Assuming fees are in lamports (1 SOL = 1_000_000 lamports)
    let dispute_fee = 1_000_000; // Same assumption for dispute fee

    // Transfer listing fee from employer to the program's account
    let cpi_accounts = SplTransfer {
        from: ctx.accounts.employer_token_account.to_account_info(),
        to: ctx.accounts.program_token_account.to_account_info(),
        authority: ctx.accounts.employer.to_account_info(),
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    
    // Transfer listing fee
    let transfer_listing_fee_ctx = CpiContext::new(cpi_program.clone(), cpi_accounts);
    SplTransfer::transfer(transfer_listing_fee_ctx, listing_fee)?;

    msg!("Transferred listing fee of 1 SOL!");

    if with_dispute {
        // Transfer dispute fee if applicable
        let dispute_cpi_accounts = SplTransfer {
            from: ctx.accounts.employer_token_account.to_account_info(),
            to: ctx.accounts.program_token_account.to_account_info(),
            authority: ctx.accounts.employer.to_account_info(),
        };

        let dispute_transfer_ctx = CpiContext::new(cpi_program.clone(), dispute_cpi_accounts);
        SplTransfer::transfer(dispute_transfer_ctx, dispute_fee)?;

        msg!("Transferred dispute fee of 1 SOL!");
    }

    // Update contract status or any other necessary fields
    contract.status = JobContractStatus::Listed; // Example status update

    msg!("Job listed successfully!");
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(contract_id: String)]
pub struct JobListingWithFeesEmployerContext<'info> {
    #[account(mut)]
    pub employer: Signer<'info>,

    #[account(
        mut, 
        seeds = [
            CONTRACT_SEED.as_bytes(), 
            &contract_id.as_bytes()
        ], 
        bump, 
    )]
    pub contract: Account<'info, HourlyContract>,

    #[account(mut)]
    pub employer_token_account: Account<'info, TokenAccount>, // Employer's token account

    #[account(mut)]
    pub program_token_account: Account<'info, TokenAccount>, // Program's token account to receive fees

    pub token_program: Program<'info, Token>,
    
    pub system_program: Program<'info, System>,
}