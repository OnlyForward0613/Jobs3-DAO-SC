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


pub fn update_worked_hour(
    ctx: Context<UpdateWorkedHourContext>,
    contract_id: String,
    week_worked_hour: u32,
) -> Result<()> {
    msg!("Updating weekly worked hours on seller side!");
    let contract = &mut ctx.accounts.contract;

    // Check if the signer is a correct seller
    require_keys_eq!(ctx.accounts.seller.key(), contract.seller, GigContractError::InvalidActivator);

    // Check if the contract is not paused
    require!(contract.paused == false, GigContractError::HourlyContractPaused);

    // Check if the contract is not ended
    require!(contract.status != HourlyContractStatus::Ended, GigContractError::HourlyContractEnded);

    // Check if the contract is Created or Accepted.
    require!(contract.status == HourlyContractStatus::Created || contract.status == HourlyContractStatus::Accepted, GigContractError::CantActivate);

    contract.status = HourlyContractStatus::Active;
    contract.week_worked_hour = week_worked_hour;

    msg!("Updated weekly worked hours successfully!");
    Ok(())
}

#[derive(Accounts)]
#[instruction(contract_id: String)]
pub struct UpdateWorkedHourContext<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

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
