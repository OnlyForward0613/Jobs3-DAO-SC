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


pub fn seller_approve(
    ctx: Context<SellerApproveContext>,
    contract_id: String,
    seller_satisfied: bool
) -> Result<()> {
    msg!("Releasing funds on seller side!");

    let contract = &mut ctx.accounts.contract;

    require_keys_eq!(ctx.accounts.pay_token_mint.key(), PAY_TOKEN_MINT_ADDRESS, GigContractError::PayTokenMintError);

    // Check if the signer is a correct seller
    require_keys_eq!(ctx.accounts.seller.key(), contract.seller, GigContractError::InvalidSeller);

    // Check if the contract is Active or pending.
    require!(contract.status == ContractStatus::Active || contract.status == ContractStatus::Pending, GigContractError::CantRelease);

    let token_program = &ctx.accounts.token_program;
    let source = &ctx.accounts.contract_ata;
    let seller_destination = &ctx.accounts.seller_ata;
    let buyer_destination = &ctx.accounts.buyer_ata;
    let admin_destination = &ctx.accounts.admin_ata;
    let buyer_referral_destination = &ctx.accounts.buyer_referral_ata;
    let seller_referral_destination = &ctx.accounts.seller_referral_ata;

    contract.status = ContractStatus::Pending;
    contract.seller_approved = true;

    let total_balance = source.amount;

    // If both parties approve, transfer funds from the contrac to seller
    // dispute for both party and platform fee to admin
    if contract.buyer_approved == true {
        if contract.split == true {
            if seller_satisfied == true {
                // if both parties agress with split decision, then split payment
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
                ((total_balance - 2 * contract.dispute) * 45 / 100 + contract.dispute).try_into().unwrap(),
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
                ((total_balance - 2 * contract.dispute) * 45 / 100 + contract.dispute).try_into().unwrap(),
                )?;

                let mut admin_amount: u64 = ((total_balance - 2 * contract.dispute ) * 10 / 100).try_into().unwrap();

                // To buyer referral
                if contract.buyer_referral != anchor_lang::solana_program::pubkey!("3x9USDofKPb6rREu2dWe9rcvT4QMHQS1PrJ13WuZ1QL3") {
                    let buyer_referral_amount: u64 = ((total_balance - 2 * contract.dispute ) * 1 / 100).try_into().unwrap();
                    admin_amount -= buyer_referral_amount;
    
                    token::transfer(
                        CpiContext::new_with_signer(
                            token_program.to_account_info(),
                            SplTransfer {
                                from: source.to_account_info().clone(),
                                to: buyer_referral_destination.to_account_info().clone(),
                                authority: contract.to_account_info().clone(),
                            },
                            &[&[CONTRACT_SEED.as_bytes(), &contract.contract_id.as_bytes(), &[ctx.bumps.contract]]],
                        ),
                        buyer_referral_amount,
                    )?;
                } 
    
                // To seller referral
                if contract.seller_referral != anchor_lang::solana_program::pubkey!("3x9USDofKPb6rREu2dWe9rcvT4QMHQS1PrJ13WuZ1QL3") {
                    let seller_referral_amount: u64 = ((total_balance - 2 * contract.dispute ) * 1 / 100).try_into().unwrap();
                    admin_amount -= seller_referral_amount;
    
                    token::transfer(
                        CpiContext::new_with_signer(
                            token_program.to_account_info(),
                            SplTransfer {
                                from: source.to_account_info().clone(),
                                to: seller_referral_destination.to_account_info().clone(),
                                authority: contract.to_account_info().clone(),
                            },
                            &[&[CONTRACT_SEED.as_bytes(), &contract.contract_id.as_bytes(), &[ctx.bumps.contract]]],
                        ),
                        seller_referral_amount,
                    )?;
                } 
    
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
                    admin_amount,
                )?;
            } else {
                // Raise dispute if seller is not satisfied with split decision
                contract.status = ContractStatus::Dispute;
            }
        } else {
            // When both parties are satisfied with the result
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
            ((total_balance - 2 * contract.dispute) * 90 / 100 + contract.dispute).try_into().unwrap(),
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

            let mut admin_amount: u64 = ((total_balance - 2 * contract.dispute ) * 10 / 100).try_into().unwrap();

            // To buyer referral
            if contract.buyer_referral != anchor_lang::solana_program::pubkey!("3x9USDofKPb6rREu2dWe9rcvT4QMHQS1PrJ13WuZ1QL3") {
                let buyer_referral_amount: u64 = ((total_balance - 2 * contract.dispute ) * 1 / 100).try_into().unwrap();
                admin_amount -= buyer_referral_amount;

                token::transfer(
                    CpiContext::new_with_signer(
                        token_program.to_account_info(),
                        SplTransfer {
                            from: source.to_account_info().clone(),
                            to: buyer_referral_destination.to_account_info().clone(),
                            authority: contract.to_account_info().clone(),
                        },
                        &[&[CONTRACT_SEED.as_bytes(), &contract.contract_id.as_bytes(), &[ctx.bumps.contract]]],
                    ),
                    buyer_referral_amount,
                )?;
            } 

            // To seller referral
            if contract.seller_referral != anchor_lang::solana_program::pubkey!("3x9USDofKPb6rREu2dWe9rcvT4QMHQS1PrJ13WuZ1QL3") {
                let seller_referral_amount: u64 = ((total_balance - 2 * contract.dispute ) * 1 / 100).try_into().unwrap();
                admin_amount -= seller_referral_amount;

                token::transfer(
                    CpiContext::new_with_signer(
                        token_program.to_account_info(),
                        SplTransfer {
                            from: source.to_account_info().clone(),
                            to: seller_referral_destination.to_account_info().clone(),
                            authority: contract.to_account_info().clone(),
                        },
                        &[&[CONTRACT_SEED.as_bytes(), &contract.contract_id.as_bytes(), &[ctx.bumps.contract]]],
                    ),
                    seller_referral_amount,
                )?;
            } 

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
                admin_amount,
            )?;
        }
    }

    msg!("Funds released by seller successfully!");
    Ok(())
}

#[derive(Accounts)]
#[instruction(contract_id: String)]
pub struct SellerApproveContext<'info> {
    pub pay_token_mint: Account<'info, Mint>, // Define the mint account

    #[account(mut)]
    pub seller: Signer<'info>,

    pub seller_referral: SystemAccount<'info>,
    pub buyer_referral: SystemAccount<'info>,

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
        associated_token::mint = pay_token_mint,
        associated_token::authority = contract.buyer,
    )]
    pub buyer_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut, 
        associated_token::mint = pay_token_mint,
        associated_token::authority = contract.seller,
    )]
    pub seller_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = pay_token_mint,
        associated_token::authority = seller_referral,
    )]
    pub seller_referral_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = pay_token_mint,
        associated_token::authority = buyer_referral,
    )]
    pub buyer_referral_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut, 
        associated_token::mint = pay_token_mint,
        associated_token::authority = ADMIN_ADDRESS,
    )]
    pub admin_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut, 
        associated_token::mint = pay_token_mint,
        associated_token::authority = contract,
    )]
    pub contract_ata: Box<Account<'info, TokenAccount>>,


    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
