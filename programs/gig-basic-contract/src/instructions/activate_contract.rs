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


pub fn activate_contract(
    ctx: Context<ActivateContractContext>,
    contract_id: String,
    with_dispute: bool,
) -> Result<()> {
    msg!("Activating contact on seller side!");
    let contract = &mut ctx.accounts.contract;

    // Check if the signer is a correct seller
    require_keys_eq!(ctx.accounts.seller.key(), contract.seller, GigContractError::InvalidActivator);

    // Check if the contract is Created or Accepted.
    require!(contract.status == ContractStatus::Created || contract.status == ContractStatus::Accepted, GigContractError::CantActivate);

    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.seller;
    let source = &ctx.accounts.seller_ata;
    let destination = &ctx.accounts.contract_ata;

    contract.status = ContractStatus::Active;

    if let Some(seller_referral) = &ctx.accounts.seller_referral {
        msg!("seller_referral provided: {}", seller_referral.key());
        contract.seller_referral = seller_referral.key();
    }

    if with_dispute ==  true {
         // Transfer paytoken(dispute) to the contract account
        token::transfer(
            CpiContext::new(
                token_program.to_account_info(),
                SplTransfer {
                    from: source.to_account_info().clone(),
                    to: destination.to_account_info().clone(),
                    authority: authority.to_account_info().clone(),
                },
            ),
            contract.dispute,
        )?;
    }

    msg!("Contract activated successfully!");
    Ok(())
}

#[derive(Accounts)]
#[instruction(contract_id: String)]
pub struct ActivateContractContext<'info> {
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
    pub contract: Account<'info, Contract>,

    #[account(
        mut, 
        associated_token::mint = PAY_TOKEN_MINT_ADDRESS,
        associated_token::authority = seller,
    )]
    pub seller_ata: Account<'info, TokenAccount>,

    // Referral is optional
    pub seller_referral:  Option<SystemAccount<'info>>,

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
