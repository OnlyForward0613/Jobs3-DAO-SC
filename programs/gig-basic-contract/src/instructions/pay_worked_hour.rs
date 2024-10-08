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


pub fn pay_worked_hour(
    ctx: Context<PayWorkedHourContext>,
    contract_id: String,
    amount: u64
) -> Result<()> {
    msg!("Paying weekly worked hours on buyer side!");
    let contract = &mut ctx.accounts.contract;

    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.buyer;
    let source = &ctx.accounts.buyer_ata;
    let destination = &ctx.accounts.contract_ata;

    // Check if the signer is a correct seller
    require_keys_eq!(ctx.accounts.buyer.key(), contract.buyer, GigContractError::InvalidActivator);

    // Check if the contract is ReadyToPay.
    require!(contract.status == HourlyContractStatus::ReadyToPay, GigContractError::HourlyGigPayError);

    // powi(10.0, 6) for USDC, powi(10.0, 8) for BPT for test
    require!(amount == (contract.week_worked_hour as f64 * contract.hourly_rate as f64 * f64::powi(10.0, 8)).round() as u64 , GigContractError::PayAmountError);

    contract.status = HourlyContractStatus::Paid;
    contract.total_worked_hour += contract.week_worked_hour;
    contract.week_worked_hour = 0;

    // Transfer paytoken(amount) which is calculated by hourly rate * worked hour to the contract account
    token::transfer(
        CpiContext::new(
            token_program.to_account_info(),
            SplTransfer {
                from: source.to_account_info().clone(),
                to: destination.to_account_info().clone(),
                authority: authority.to_account_info().clone(),
            },
        ),
        amount,
    )?;

    msg!("Paid weekly worked hours successfully!");
    Ok(())
}

#[derive(Accounts)]
#[instruction(contract_id: String)]
pub struct PayWorkedHourContext<'info> {
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

    pub pay_token_mint: Account<'info, Mint>,
    
    #[account(
        mut, 
        associated_token::mint = pay_token_mint,
        associated_token::authority = buyer,
    )]
    pub buyer_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = pay_token_mint,
        associated_token::authority = contract,
    )]
    pub contract_ata: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
