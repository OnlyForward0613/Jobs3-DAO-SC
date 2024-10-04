use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ self, Mint, Token, TokenAccount, Transfer as SplTransfer }
};
use std::mem::size_of;

use crate::state::hourly_contract::*;
use crate::constants::{
    CONTRACT_SEED,
    PAY_TOKEN_MINT_ADDRESS
};
use crate::errors::{
    GigContractError
};


pub fn resume_hourly_contract(
    ctx: Context<ResumeHourlyContractContext>,
    contract_id: String,
) -> Result<()> {
    msg!("Resuming hourly contract on buyer side!");
    let contract = &mut ctx.accounts.contract;

    // Check if the signer is a correct buyer
    require_keys_eq!(ctx.accounts.buyer.key(), contract.buyer, GigContractError::InvalidActivator);

    // Check if the contract is Created or Accepted.
    require!(contract.status == HourlyContractStatus::Created || contract.status == HourlyContractStatus::Accepted, GigContractError::CantActivate);

    contract.status = HourlyContractStatus::Ended;
    contract.paused = false;

    msg!("Resumed hourly contract successfully!");
    Ok(())
}

#[derive(Accounts)]
#[instruction(contract_id: String)]
pub struct ResumeHourlyContractContext<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut, 
        seeds = [
            CONTRACT_SEED.as_bytes(), 
            &contract_id.as_bytes()
        ], 
        bump, 
    )]
    pub contract: Account<'info, HourlyContract>,

    pub system_program: Program<'info, System>,
}
