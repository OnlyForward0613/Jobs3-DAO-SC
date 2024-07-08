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

declare_id!("GuqgMVMCLi9daQMHyhnLRGTpgpwNGU4yZXs9GK4SYrbS");

#[program]
pub mod gig_basic_contract {
    use super::*;

    /* 
        Buyer will start a working contract between buyer and seller 
        by calling this function with payment amount and dispute fee. 
        Or Seller will start by calling this function wiht just dispute fee.
    */
    
    pub fn start_contract(ctx: Context<StartContractContext>, contract_id: String, amount: u64, dispute: u64, deadline: u32, on_buyer: bool) -> Result<()> {
        instructions::start_contract::start_contract(ctx, contract_id, amount, dispute, deadline, on_buyer)
    }

    /* 
        Buyer will accept the contract after buyer creates a new contract.
        by calling this function with payment amount and dispute fee. 
    */
    pub fn accept_contract(ctx: Context<AcceptContractContext>, contract_id: String,) -> Result<()> {
        instructions::accept_contract::accept_contract(ctx, contract_id)
    }

    /* 
        Seller will activate the contract after checking all conditions that buyer set 
        when creating the contract.
    */
    pub fn activate_contract(ctx: Context<ActivateContractContext>, contract_id: String, with_dispute: bool) -> Result<()> {
        instructions::activate_contract::activate_contract(ctx, contract_id, with_dispute)
    }

    /*
        Buyer will release funds after satisfied with products seller will deliver.
        Here, split will be true if buyer is dissatisfied
    */
    pub fn buyer_approve(ctx: Context<BuyerApproveContext>, contract_id: String, split: bool) -> Result<()> {
        instructions::buyer_approve::buyer_approve(ctx, contract_id, split)
    }

    /*
        Admin will approve if there is a dispute.
        decision value: 0 for both ok by default, 1 for seller, 2 for buyer, 3 for split
    */
    pub fn admin_approve(ctx: Context<AdminApproveContext>, contract_id: String, decision: u8) -> Result<()> {
        instructions::admin_approve::admin_approve(ctx, contract_id, decision)
    }

    /*
        Seller will approve the amount of funds to receive 
        Here, seller_satisfied will be true if seller agree with split payment. Otherwise false
    */
    pub fn seller_approve(ctx: Context<SellerApproveContext>, contract_id: String, seller_satisfied: bool) -> Result<()> {
        instructions::seller_approve::seller_approve(ctx, contract_id, seller_satisfied)
    }
}
