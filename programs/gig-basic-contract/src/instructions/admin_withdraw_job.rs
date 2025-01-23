use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount, Transfer as SplTransfer};
use crate::state::contract::*;
use crate::constants::*;
use crate::errors::*;
use crate::state::job_contract::*;
use crate::state::hourly_contract::*;

pub fn admin_withdraw_job_contract(ctx: Context<AdminWithdrawJobContractContext>, contract_id: String) -> Result<()> {
    // Log the contract_id for debugging
    msg!("Contract ID: {}", contract_id);

    let cpi_accounts = anchor_spl::token::Transfer {
        from: ctx.accounts.contract_ata.to_account_info(),
        to: ctx.accounts.withdraw_address.to_account_info(),
        authority: ctx.accounts.contract.to_account_info(),
    };

    let amount = ctx.accounts.contract_ata.amount;

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let seeds = &[b"gig_contract".as_ref(), &contract_id.as_bytes(), &[ctx.bumps.contract]];
    
    // Log the derived address for debugging
    let derived_address = Pubkey::create_program_address(seeds, ctx.program_id).expect("Failed to create derived address");
    msg!("Derived Address: {}", derived_address);

    let signer = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    anchor_spl::token::transfer(cpi_ctx, amount)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(contract_id: String)]
pub struct AdminWithdrawJobContractContext<'info> {
    #[account(
        mut,
        seeds = [
            CONTRACT_SEED.as_bytes(), 
            &contract_id.as_bytes()
        ],
        bump, 
    )]
    pub contract: Account<'info, JobContract>,

    #[account(mut)]
    pub admin: Signer<'info>,
    
    /// CHECK: this is safe account. no need to check
    pub pay_token_mint: AccountInfo<'info>,

    #[account(mut)]
    pub contract_ata: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,

    #[account(mut,
    constraint = withdraw_address.owner == ADMIN_ADDRESS.key())]
    pub withdraw_address: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>
}