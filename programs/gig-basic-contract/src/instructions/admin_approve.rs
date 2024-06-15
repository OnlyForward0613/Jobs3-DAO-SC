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


pub fn admin_approve(
    ctx: Context<AdminApproveContext>,
    contract_id: String,
) -> Result<()> {
    msg!("Releasing funds on seller side!");

    let contract = &mut ctx.accounts.contract;


    // Check if the contract is pending which means one of two parties approved.
    require!(contract.status == ContractStatus::Pending, GigContractError::CantRelease);

    // Check if the signer is a correct admin
    require_keys_eq!(ctx.accounts.admin.key(), ADMIN_ADDRESS, GigContractError::InvalidAdmin);

    let token_program = &ctx.accounts.token_program;
    let source = &ctx.accounts.contract_ata;
    let seller_destination = &ctx.accounts.seller_ata;
    let buyer_destination = &ctx.accounts.buyer_ata;
    let admin_destination = &ctx.accounts.admin_ata;

    contract.status = ContractStatus::Completed;
    contract.seller_approved = true;

    let total_balance = source.amount;

    // If admin approves, transfer funds from the contrac to seller
   if contract.buyer_approved == true {
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
   } else { // seller approved
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

    msg!("Funds released by admin successfully!");
    Ok(())
}

#[derive(Accounts)]
#[instruction(contract_id: String)]
pub struct AdminApproveContext<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

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
