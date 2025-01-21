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
    decision: u8 // 0 for both ok by default, 1 for seller, 2 for buyer, 3 for split
) -> Result<()> {
    msg!("Releasing funds on seller side!");

    let contract = &mut ctx.accounts.contract;

    // Check if the signer is a correct admin
    require_keys_eq!(ctx.accounts.admin.key(), ADMIN_ADDRESS, GigContractError::InvalidAdmin);

    // Check if the contract is pending which means one of two parties approved.
    require!(contract.status == ContractStatus::Pending || contract.status == ContractStatus::Dispute, GigContractError::NotReadyYet);

    let token_program = &ctx.accounts.token_program;
    let source = &ctx.accounts.contract_ata;
    let seller_destination = &ctx.accounts.seller_ata;
    let buyer_destination = &ctx.accounts.buyer_ata;
    let admin_destination = &ctx.accounts.admin_ata;

    // contract.status = ContractStatus::Completed;
    contract.admin_approved = true;

    let total_balance = source.amount;

    // If buyer is not responding, admin will approve with seller. Admin will get buyer's dispute fee
   if contract.status == ContractStatus::Pending { 
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
    
        let mut admin_amount: u64 = ((total_balance - 2 * contract.dispute ) * 10 / 100 + contract.dispute).try_into().unwrap();

        // To buyer referral
        if let Some(buyer_referral_ata) = &ctx.accounts.buyer_referral_ata {
            let buyer_referral_amount: u64 = ((total_balance - 2 * contract.dispute ) * 1 / 100).try_into().unwrap();
            admin_amount -= buyer_referral_amount;

            token::transfer(
                CpiContext::new_with_signer(
                    token_program.to_account_info(),
                    SplTransfer {
                        from: source.to_account_info().clone(),
                        to: buyer_referral_ata.to_account_info().clone(),
                        authority: contract.to_account_info().clone(),
                    },
                    &[&[CONTRACT_SEED.as_bytes(), &contract.contract_id.as_bytes(), &[ctx.bumps.contract]]],
                ),
                buyer_referral_amount,
            )?;
        } 

        // To seller referral
        if let Some(seller_referral_ata) = &ctx.accounts.seller_referral_ata {
            let seller_referral_amount: u64 = ((total_balance - 2 * contract.dispute ) * 1 / 100).try_into().unwrap();
            admin_amount -= seller_referral_amount;

            token::transfer(
                CpiContext::new_with_signer(
                    token_program.to_account_info(),
                    SplTransfer {
                        from: source.to_account_info().clone(),
                        to: seller_referral_ata.to_account_info().clone(),
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
        // if dispute, perform action based on decision value
        // 0 for both ok by default, 1 for seller, 2 for buyer, 3 for split
        
        match decision {
            1 => { // admin agrees with seller
                // transfer payment to seller and admin gets buyer's dispute fee
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
    
                let mut admin_amount: u64 = ((total_balance - 2 * contract.dispute ) * 10 / 100 + contract.dispute).try_into().unwrap();

                // To buyer referral
                if let Some(buyer_referral_ata) = &ctx.accounts.buyer_referral_ata {
                    let buyer_referral_amount: u64 = ((total_balance - 2 * contract.dispute ) * 1 / 100).try_into().unwrap();
                    admin_amount -= buyer_referral_amount;

                    token::transfer(
                        CpiContext::new_with_signer(
                            token_program.to_account_info(),
                            SplTransfer {
                                from: source.to_account_info().clone(),
                                to: buyer_referral_ata.to_account_info().clone(),
                                authority: contract.to_account_info().clone(),
                            },
                            &[&[CONTRACT_SEED.as_bytes(), &contract.contract_id.as_bytes(), &[ctx.bumps.contract]]],
                        ),
                        buyer_referral_amount,
                    )?;
                } 

                // To seller referral
                if let Some(seller_referral_ata) = &ctx.accounts.seller_referral_ata {
                    let seller_referral_amount: u64 = ((total_balance - 2 * contract.dispute ) * 1 / 100).try_into().unwrap();
                    admin_amount -= seller_referral_amount;

                    token::transfer(
                        CpiContext::new_with_signer(
                            token_program.to_account_info(),
                            SplTransfer {
                                from: source.to_account_info().clone(),
                                to: seller_referral_ata.to_account_info().clone(),
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
            },
            2 => {
                // admin agrees with buyer
                // transfer payment to buyer and admin gets seller's dispute fee
                contract.status = ContractStatus::Completed;

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
                    ((total_balance - 2 * contract.dispute) * 90 / 100 + contract.dispute).try_into().unwrap(),
                )?;

                let mut admin_amount: u64 = ((total_balance - 2 * contract.dispute ) * 10 / 100 + contract.dispute).try_into().unwrap();

                // To buyer referral
                if let Some(buyer_referral_ata) = &ctx.accounts.buyer_referral_ata {
                    let buyer_referral_amount: u64 = ((total_balance - 2 * contract.dispute ) * 1 / 100).try_into().unwrap();
                    admin_amount -= buyer_referral_amount;

                    token::transfer(
                        CpiContext::new_with_signer(
                            token_program.to_account_info(),
                            SplTransfer {
                                from: source.to_account_info().clone(),
                                to: buyer_referral_ata.to_account_info().clone(),
                                authority: contract.to_account_info().clone(),
                            },
                            &[&[CONTRACT_SEED.as_bytes(), &contract.contract_id.as_bytes(), &[ctx.bumps.contract]]],
                        ),
                        buyer_referral_amount,
                    )?;
                } 

                // To seller referral
                if let Some(seller_referral_ata) = &ctx.accounts.seller_referral_ata {
                    let seller_referral_amount: u64 = ((total_balance - 2 * contract.dispute ) * 1 / 100).try_into().unwrap();
                    admin_amount -= seller_referral_amount;

                    token::transfer(
                        CpiContext::new_with_signer(
                            token_program.to_account_info(),
                            SplTransfer {
                                from: source.to_account_info().clone(),
                                to: seller_referral_ata.to_account_info().clone(),
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
            },
            _ => {
                // admin agrees with split dicision
                // split payment and admin gets half of dispute fee from both parties
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
                    ((total_balance - 2 * contract.dispute) * 45 / 100 + contract.dispute / 2).try_into().unwrap(),
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
                    ((total_balance - 2 * contract.dispute) * 45 / 100 + contract.dispute / 2).try_into().unwrap(),
                )?;

                let mut admin_amount: u64 = ((total_balance - 2 * contract.dispute ) * 10 / 100 + contract.dispute).try_into().unwrap();

                // To buyer referral
                if let Some(buyer_referral_ata) = &ctx.accounts.buyer_referral_ata {
                    let buyer_referral_amount: u64 = ((total_balance - 2 * contract.dispute ) * 1 / 100).try_into().unwrap();
                    admin_amount -= buyer_referral_amount;

                    token::transfer(
                        CpiContext::new_with_signer(
                            token_program.to_account_info(),
                            SplTransfer {
                                from: source.to_account_info().clone(),
                                to: buyer_referral_ata.to_account_info().clone(),
                                authority: contract.to_account_info().clone(),
                            },
                            &[&[CONTRACT_SEED.as_bytes(), &contract.contract_id.as_bytes(), &[ctx.bumps.contract]]],
                        ),
                        buyer_referral_amount,
                    )?;
                } 

                // To seller referral
                if let Some(seller_referral_ata) = &ctx.accounts.seller_referral_ata {
                    let seller_referral_amount: u64 = ((total_balance - 2 * contract.dispute ) * 1 / 100).try_into().unwrap();
                    admin_amount -= seller_referral_amount;

                    token::transfer(
                        CpiContext::new_with_signer(
                            token_program.to_account_info(),
                            SplTransfer {
                                from: source.to_account_info().clone(),
                                to: seller_referral_ata.to_account_info().clone(),
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

    // Optional
    #[account(
        mut, 
        associated_token::mint = PAY_TOKEN_MINT_ADDRESS,
        associated_token::authority = contract.seller_referral,
    )]
    pub seller_referral_ata: Option<Account<'info, TokenAccount>>,

    #[account(
        mut, 
        associated_token::mint = PAY_TOKEN_MINT_ADDRESS,
        associated_token::authority = contract.buyer,
    )]
    pub buyer_ata: Account<'info, TokenAccount>,

    // Optional
    #[account(
        mut, 
        associated_token::mint = PAY_TOKEN_MINT_ADDRESS,
        associated_token::authority = contract.buyer_referral,
    )]
    pub buyer_referral_ata: Option<Account<'info, TokenAccount>>,

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
