use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ self, Mint, Token, TokenAccount, Transfer as SplTransfer }
};
use std::mem::size_of;

use crate::state::contract::*;
use crate::constants::{
    CONTRACT_SEED,
    PAY_TOKEN_MINT_ADDRESS
};
use crate::errors::{
    GigContractError
};


pub fn accept_contract_on_buyer(
    ctx: Context<AcceptContractOnBuyerContext>,
    contract_id: String,
) -> Result<()> {
    msg!("Accepting contact on buyer side!");
    let contract = &mut ctx.accounts.contract;

    // Check if the signer is a correct buyer
    require_keys_eq!(ctx.accounts.buyer.key(), contract.buyer, GigContractError::InvalidActivator);

    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.buyer;
    let source = &ctx.accounts.buyer_ata;
    let destination = &ctx.accounts.contract_ata;

    contract.status = ContractStatus::Accepted;

    // Transfer paytoken(amount + dispute) to the contract account
    token::transfer(
    CpiContext::new(
        token_program.to_account_info(),
        SplTransfer {
            from: source.to_account_info().clone(),
            to: destination.to_account_info().clone(),
            authority: authority.to_account_info().clone(),
        },
    ),
    (contract.amount + contract.dispute).try_into().unwrap(),
    )?;

    msg!("Contract accepted successfully!");
    Ok(())
}

#[derive(Accounts)]
#[instruction(contract_id: String)]
pub struct AcceptContractOnBuyerContext<'info> {
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
    pub contract: Account<'info, Contract>,

    #[account(
        mut, 
        associated_token::mint = PAY_TOKEN_MINT_ADDRESS,
        associated_token::authority = buyer,
    )]
    pub buyer_ata: Account<'info, TokenAccount>,


    #[account(
        mut, 
        associated_token::mint = PAY_TOKEN_MINT_ADDRESS,
        associated_token::authority = contract,
    )]
    pub contract_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
