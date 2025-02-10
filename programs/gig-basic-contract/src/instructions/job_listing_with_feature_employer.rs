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
    EMPLOYER_REFERRAL,
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
    let listing_fee : u64 = match featured_day {
        1 => 21_000_000,  // 24 hours
        3 => 36_000_000,  // 3 days
        7 => 71_000_000,  // 7 days
        14 => 100_000_000, // 14 days
        30 => 150_000_000,// 30 days
        _ => return Err(GigContractError::InvalidFeaturedDay.into()), // Handle invalid day
    };

    let dispute_fee = 1_000_000; // Same assumption for dispute fee

    // Define source and destination token accounts
    let employer_ata = &ctx.accounts.employer_ata;
    let contract_ata = &ctx.accounts.contract_ata;
    let authority = &ctx.accounts.employer;
    let token_program = &ctx.accounts.token_program;

    // Handle referral transfer if a referral account is provided and not the excluded address
    if let Some(employer_referral_ata) = &ctx.accounts.employer_referral_ata {
        msg!("Employer refferal provided!");
        if employer_referral_ata.key() != EMPLOYER_REFERRAL {
            msg!("Employer referral provided: {} {}", employer_referral_ata.key(), listing_fee);
            job_contract.employer_referral = employer_referral_ata.key();

            let referral_amount: u64 = listing_fee * 10 / 100;
            let contract_amount: u64 = listing_fee * 90 / 100;

            // Transfer referral bonus
            token::transfer(
                CpiContext::new(
                    token_program.to_account_info(),
                    SplTransfer {
                        from: employer_ata.to_account_info(),
                        to: employer_referral_ata.to_account_info(),
                        authority: authority.to_account_info(),
                    }
                ),
                referral_amount
            )?;

            msg!("confirmed first send");

            // Transfer the remaining amount to the contract
            token::transfer(
                CpiContext::new(
                    token_program.to_account_info(),
                    SplTransfer {
                        from: employer_ata.to_account_info(),
                        to: contract_ata.to_account_info(),
                        authority: authority.to_account_info(),
                    },
                ),
                contract_amount
            )?;

            msg!("confirmed second send");

        } else {
            msg!("Employer referral provided, but is the excluded address.  Skipping referral bonus.");

            // Transfer the full listing fee to the contract
            token::transfer(
                CpiContext::new(
                    token_program.to_account_info(),
                    SplTransfer {
                        from: employer_ata.to_account_info(),
                        to: contract_ata.to_account_info(),
                        authority: authority.to_account_info(),
                    },
                ),
                listing_fee as u64,
            )?;
        }
    } else {
        msg!("Employer referral not provided.");

        // Transfer the full listing fee to the contract
        token::transfer(
            CpiContext::new(
                token_program.to_account_info(),
                SplTransfer {
                    from: employer_ata.to_account_info(),
                    to: contract_ata.to_account_info(),
                    authority: authority.to_account_info(),
                },
            ),
            listing_fee as u64,
        )?;
    }

    msg!("Transferred listing fee of {} USDC!", listing_fee / 1_000_000);

    // Update contract status
    job_contract.contract_id = contract_id;
    job_contract.status = JobContractStatus::Created;
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
    // Referral is optional, but MUST be a TokenAccount if provided
    // #[account(mut)]
    // pub employer_referral: Option<Account<'info, TokenAccount>>,
    // Optional

    #[account(
        mut, 
        // constraint = employer_referral_ata.mint == PAY_TOKEN_MINT_ADDRESS,
        constraint = employer_referral_ata.owner != EMPLOYER_REFERRAL,
    )]
    pub employer_referral_ata: Option<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}