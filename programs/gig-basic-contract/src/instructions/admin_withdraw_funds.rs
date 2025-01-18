use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_instruction, program::invoke};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ self, Mint, Token, TokenAccount, Transfer as SplTransfer }
};
use std::mem::size_of;

use crate::state::contract::*;
use crate::constants::{
    CONTRACT_SEED,
    PAY_TOKEN_MINT_ADDRESS,
    ADMIN_ADDRESS
};
use crate::errors::{
    GigContractError
};


pub fn admin_withdraw_funds(
    ctx: Context<AdminWithdrawFundsContext>,
    amount: u64,
    withdrawer_address: Pubkey,
    contract_type: u8
) -> Result<()> {
    msg!("Admin sent request to withdraw funds from the program account!");
    
    // Check if the withdrawer is admin.
    require!(withdrawer_address == ADMIN_ADDRESS, GigContractError::InvalidAdmin);

    let program_balance;
    let transfer_instruction;

    match contract_type {
        0 => {
            // Admin can withdraw from the standard contract account
            msg!("Processing withdrawal for Gig contract.");
            program_balance = **ctx.accounts.contract.try_borrow_lamports()?;
            if program_balance < amount {
                return Err(GigContractError::InsufficientFunds.into());
            }

            // Create transfer instruction for standard contract account
            transfer_instruction = system_instruction::transfer(ctx.accounts.contract.key, withdrawer_address, amount);
            invoke(
                &transfer_instruction,
                &[ctx.accounts.contract.clone(), withdrawer_address.clone()],
            )?;
        }
        1 => {
            // Admin can withdraw from the job contract account
            msg!("Processing withdrawal for job contract.");
            program_balance = **ctx.accounts.job_contract.try_borrow_lamports()?;
            if program_balance < amount {
                return Err(GigContractError::InsufficientFunds.into());
            }

            // Create transfer instruction for job contract account
            transfer_instruction = system_instruction::transfer(ctx.accounts.job_contract.key, withdrawer_address, amount);
            invoke(
                &transfer_instruction,
                &[ctx.accounts.job_contract.clone(), withdrawer_address.clone()],
            )?;
        }
        _ => return Err(GigContractError::InvalidContractType.into()), // Handle invalid type
    }

    // // Invoke the transfer instruction
    // invoke(
    //     &transfer_instruction,
    //     &[ctx.accounts.contract_account.clone(), ctx.accounts.recipient.clone()],
    // )?;

    // let contract = &mut ctx.accounts.contract;


    // let token_program = &ctx.accounts.token_program;
    // let authority = &ctx.accounts.buyer;
    // let source = &ctx.accounts.buyer_ata;
    // let destination = &ctx.accounts.contract_ata;

    // contract.status = ContractStatus::Accepted;

    // if let Some(buyer_referral) = &ctx.accounts.buyer_referral {
    //     msg!("buyer_referral provided: {}", buyer_referral.key());
    //     contract.buyer_referral = buyer_referral.key();
    // }

    // // Transfer paytoken(amount + dispute) to the contract account
    // token::transfer(
    // CpiContext::new(
    //     token_program.to_account_info(),
    //     SplTransfer {
    //         from: source.to_account_info().clone(),
    //         to: destination.to_account_info().clone(),
    //         authority: authority.to_account_info().clone(),
    //     },
    // ),
    // (contract.amount + contract.dispute).try_into().unwrap(),
    // )?;

    msg!("Admin Withdrew successfully!");
    Ok(())
}

#[derive(Accounts)]
#[instruction(contract_id: String)]
pub struct AdminWithdrawFundsContext<'info> {
    // #[account(mut)]
    // pub contract: AccountInfo<'info>,
    // #[account(mut)]
    // pub job_contract: AccountInfo<'info>

    // #[account(
    //     mut, 
    //     seeds = [
    //         CONTRACT_SEED.as_bytes(), 
    //         &contract_id.as_bytes()
    //     ], 
    //     bump, 
    // )]
    #[account(mut)]
    pub contract: Account<'info, Contract>,
    #[account(mut)]
    pub job_contract: Account<'info, JobContract>

    // pub system_program: Program<'info, System>,
}
