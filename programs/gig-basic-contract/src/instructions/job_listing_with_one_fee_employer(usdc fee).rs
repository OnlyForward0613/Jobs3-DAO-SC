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
    let listing_fee = 1_000_000; // 1 USDC = 0.000001 micro USDC
    let dispute_fee = 1_000_000; // Same assumption for dispute fee
    
    // Define source address and destination address
    let employer_destination = &ctx.accounts.employer_ata;
    let contract_destination = &ctx.accounts.contract_ata;
    let authority = &ctx.accounts.employer;

    // Transfer listing fee from employer to the program's account
    let cpi_accounts = SplTransfer {
        from: employer_destination.to_account_info().clone(),
        to: contract_destination.to_account_info().clone(),
        authority: authority.to_account_info().clone(),
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

    #[account(
        mut,
        associated_token::mint = PAY_TOKEN_MINT_ADDRESS,
        associated_token::authority = contract.employer,
    )]
    pub employer_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = PAY_TOKEN_MINT_ADDRESS,
        associated_token::authority = contract,
    )]
    pub contract_ata: Account<'info, TokenAccount>,

    pub employer_token_account: Account<'info, TokenAccount>, // Employer's token account

    #[account(mut)]
    pub program_token_account: Account<'info, TokenAccount>, // Program's token account to receive fees

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}