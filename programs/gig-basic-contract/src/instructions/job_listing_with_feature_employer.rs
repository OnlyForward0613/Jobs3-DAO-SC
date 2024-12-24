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

pub fn job_listing_with_feature_employer(
    ctx: Context<JobListingWithFeatureEmployerContext>,
    contract_id: String,
    featured_day: u8,
) -> Result<()> {
    msg!("Listing Job with featured fee on employer side!");

    let job_contract = &mut ctx.accounts.job_contract;

    // Define the fees based on featured_day
    let listing_fee = match featured_day {
        1 => 21_000_000,  // 24 hours
        3 => 36_000_000,  // 3 days
        7 => 71_000_000,  // 7 days
        14 => 100_000_000, // 14 days
        30 => 150_000_000,// 30 days
        _ => return Err(GigContractError::InvalidFeaturedDay.into()), // Handle invalid day
    };

    // // Check if the signer is the correct employer
    // require_keys_eq!(ctx.accounts.employer.key(), job_contract.employer, GigContractError::InvalidActivator);

    // Check if the contract is not ended.
    // require!(contract.status != JobContractStatus::Created, GigContractError::HourlyContractEnded);

    // Define the fees


    let dispute_fee = 1_000_000; // Same assumption for dispute fee

    
    
    // Define source address and destination address
    let employer_destination = &ctx.accounts.employer_ata;
    let contract_destination = &ctx.accounts.contract_ata;
    let authority = &ctx.accounts.employer;
    let token_program = &ctx.accounts.token_program;

    token::transfer(
        CpiContext::new(
            token_program.to_account_info(),
            SplTransfer {
                from: employer_destination.to_account_info().clone(),
                to: contract_destination.to_account_info().clone(),
                authority: authority.to_account_info().clone(),
            },
        ),
        listing_fee.try_into().unwrap(),
    )?;

    // // Transfer listing fee from employer to the program's account
    // let cpi_accounts = SplTransfer {
    //     from: employer_destination.to_account_info().clone(),
    //     to: contract_destination.to_account_info().clone(),
    //     authority: authority.to_account_info().clone(),
    // };
    
    // let cpi_program = ctx.accounts.token_program.to_account_info();
    
    // // Transfer listing fee
    // let transfer_listing_fee_ctx = CpiContext::new(cpi_program.clone(), cpi_accounts);
    // SplTransfer::transfer(transfer_listing_fee_ctx, listing_fee)?;

    msg!("Transferred listing fee of {} USDC!", listing_fee / 1_000_000);

    // if with_dispute {
    //     // Transfer dispute fee if applicable
    //     let dispute_cpi_accounts = SplTransfer {
    //         from: ctx.accounts.employer_token_account.to_account_info(),
    //         to: ctx.accounts.program_token_account.to_account_info(),
    //         authority: ctx.accounts.employer.to_account_info(),
    //     };

    //     let dispute_transfer_ctx = CpiContext::new(cpi_program.clone(), dispute_cpi_accounts);
    //     SplTransfer::transfer(dispute_transfer_ctx, dispute_fee)?;

    //     msg!("Transferred dispute fee of 1 USDC!");
    // }

    // Update contract status or any other necessary fields
    job_contract.contract_id = contract_id;
    job_contract.status = JobContractStatus::Created; // Example status update
    job_contract.featured = true;
    job_contract.featured_day = featured_day;
    msg!("Job listed successfully!");
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(contract_id: String)]
pub struct JobListingWithFeatureEmployerContext<'info> {
    #[account(mut)]
    pub employer: Signer<'info>,
    #[account(
        init,
        space = JobContract::LEN + 8,
        payer = employer,
        seeds = [
            CONTRACT_SEED.as_bytes(), 
            &contract_id.as_bytes()
        ], 
        bump,
    )]
    pub job_contract: Account<'info, JobContract>,
    #[account(mut)]
    pub employer_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub contract_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}