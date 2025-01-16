use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ self, Mint, Token, TokenAccount, Transfer as SplTransfer }
};
use std::mem::size_of;

use crate::state::contract::*;
use crate::constants::{
    CONTRACT_SEED,
    ADMIN_ADDRESS,
    PAY_TOKEN_MINT_ADDRESS
};
use crate::errors::{
    GigContractError
};


pub fn buyer_approve(
    ctx: Context<BuyerApproveContext>,
    contract_id: String,
    split: bool
) -> Result<()> {
    msg!("Releasing funds on buyer side!");

    let contract = &mut ctx.accounts.contract;

    // Check if the signer is a correct buyer
    require_keys_eq!(ctx.accounts.buyer.key(), contract.buyer, GigContractError::InvalidBuyer);

    // Check if the contract is Active or pending.
    require!(contract.status == ContractStatus::Active || contract.status == ContractStatus::Pending, GigContractError::CantRelease);

    let token_program = &ctx.accounts.token_program;
    let source = &ctx.accounts.contract_ata;
    let seller_destination = &ctx.accounts.seller_ata;
    let buyer_destination = &ctx.accounts.buyer_ata;
    let admin_destination = &ctx.accounts.admin_ata;

    contract.status = ContractStatus::Pending;
    contract.buyer_approved = true;
    contract.split = split;

    let total_balance = source.amount;

    msg!("Funds released by buyer successfully!");
    Ok(())
}

#[derive(Accounts)]
#[instruction(contract_id: String)]
pub struct BuyerApproveContext<'info> {
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
        associated_token::authority = contract.seller,
    )]
    pub seller_ata: Account<'info, TokenAccount>,

    #[account(
        mut, 
        associated_token::mint = PAY_TOKEN_MINT_ADDRESS,
        associated_token::authority = contract.buyer,
    )]
    pub buyer_ata: Account<'info, TokenAccount>,

    #[account(
        mut, 
        associated_token::mint = PAY_TOKEN_MINT_ADDRESS,
        associated_token::authority = ADMIN_ADDRESS,
    )]
    pub admin_ata: Account<'info, TokenAccount>,


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
