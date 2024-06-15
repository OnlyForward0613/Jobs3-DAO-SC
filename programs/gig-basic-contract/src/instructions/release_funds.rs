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


pub fn release_funds(
    ctx: Context<ReleaseFundsContext>,
    contract_id: String,
) -> Result<()> {
    msg!("Releasing funds on buyer side!");

    let contract = &mut ctx.accounts.contract;

    // Check if the contract is Active or pending.
    require!(contract.status == ContractStatus::Active || contract.status == ContractStatus::Pending, GigContractError::CantRelease);

    // Check if the signer is a correct buyer
    require_keys_eq!(ctx.accounts.buyer.key(), contract.buyer, GigContractError::InvalidBuyer);

    let token_program = &ctx.accounts.token_program;
    let source = &ctx.accounts.contract_ata;
    let seller_destination = &ctx.accounts.seller_ata;
    let buyer_destination = &ctx.accounts.buyer_ata;
    let admin_destination = &ctx.accounts.admin_ata;

    contract.status = ContractStatus::Pending;
    contract.buyer_approved = true;

    let total_balance = source.amount;

    // If both parties approve, transfer funds from the contrac to seller
    // dispute for both party and platform fee to admin
    if contract.seller_approved == true {
        contract.status = ContractStatus::Completed;

        // To seller
        token::transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            SplTransfer {
                from: source.to_account_info().clone(),
                to: seller_destination.to_account_info().clone(),
                authority: contract.to_account_info().clone(),
            },
            &[&[CONTRACT_SEED.as_bytes(), &contract.contract_id.as_bytes(), &[ctx.bumps.contract]]],
        ),
        ((total_balance - 2 * contract.dispute) * 95 / 100 + contract.dispute).try_into().unwrap(),
        )?;

        // To buyer
        token::transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            SplTransfer {
                from: source.to_account_info().clone(),
                to: buyer_destination.to_account_info().clone(),
                authority: contract.to_account_info().clone(),
            },
            &[&[CONTRACT_SEED.as_bytes(), &contract.contract_id.as_bytes(), &[ctx.bumps.contract]]],
        ),
        contract.dispute,
        )?;

        // To admin
        token::transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            SplTransfer {
                from: source.to_account_info().clone(),
                to: admin_destination.to_account_info().clone(),
                authority: contract.to_account_info().clone(),
            },
            &[&[CONTRACT_SEED.as_bytes(), &contract.contract_id.as_bytes(), &[ctx.bumps.contract]]],
        ),
        ((total_balance - 2 * contract.dispute ) * 5 / 100).try_into().unwrap(),
        )?;
    }

    msg!("Funds released by buyer successfully!");
    Ok(())
}

#[derive(Accounts)]
#[instruction(contract_id: String)]
pub struct ReleaseFundsContext<'info> {
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
