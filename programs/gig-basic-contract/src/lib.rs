use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ self, Mint, Token, TokenAccount, Transfer as SplTransfer }
};

use instructions::*;

pub mod instructions;
pub mod constants;
pub mod errors;
pub mod state;

declare_id!("63MNpbyofgGQ7xELFmwm38yq5yyzuAdMxrN43rXrizEz");

#[program]
pub mod gig_basic_contract {
    use super::*;

    /* 
        Buyer will start a working contract between buyer and seller 
        by calling this function with payment amount and dispute fee. 
    */
    
    pub fn start_contract(ctx: Context<StartContractContext>, contract_id: String, amount: u64, dispute: u64, deadline: u32) -> Result<()> {
        instructions::start_contract::start_contract(ctx, contract_id, amount, dispute, deadline)
    }

    /* 
        Seller will activate the contract after checking all conditions that buyer set 
        when creating the contract.
    */
    pub fn activate_contract(ctx: Context<ActivateContractContext>, contract_id: String,) -> Result<()> {
        instructions::activate_contract::activate_contract(ctx, contract_id)
    }

    /*
        Buyer will release funds after satisfied with products seller will deliver.
    */
    pub fn release_funds(ctx: Context<ReleaseFundsContext>, contract_id: String,) -> Result<()> {
        instructions::release_funds::release_funds(ctx, contract_id)
    }

    /*
        Admin will approve if they don't sign.
    */
    pub fn admin_approve(ctx: Context<AdminApproveContext>, contract_id: String,) -> Result<()> {
        instructions::admin_approve::admin_approve(ctx, contract_id)
    }

    /*
        Seller will approve the amount of funds to receive 
        %% Actually, this is not necessary since they check and agree all conditions when starting and activating the contract %%
    */
    pub fn seller_approve(ctx: Context<SellerApproveContext>, contract_id: String,) -> Result<()> {
        instructions::seller_approve::seller_approve(ctx, contract_id)
    }
}
