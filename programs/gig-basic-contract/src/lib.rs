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
    */
    
    pub fn start_contract_on_buyer(ctx: Context<StartContractOnBuyerContext>, contract_id: String, amount: u64, dispute: u64, deadline: u32) -> Result<()> {
        instructions::start_contract_on_buyer::start_contract_on_buyer(ctx, contract_id, amount, dispute, deadline)
    }

    /* 
        Seller will start by calling this function wiht just dispute fee.
    */
    
    pub fn start_contract_on_seller(ctx: Context<StartContractOnSellerContext>, contract_id: String, amount: u64, dispute: u64, deadline: u32) -> Result<()> {
        instructions::start_contract_on_seller::start_contract_on_seller(ctx, contract_id, amount, dispute, deadline)
    }

    /* 
        Buyer will accept the contract after seller creates a new contract.
        by calling this function with payment amount and dispute fee. 
    */
    pub fn accept_contract_on_buyer(ctx: Context<AcceptContractOnBuyerContext>, contract_id: String,) -> Result<()> {
        instructions::accept_contract_on_buyer::accept_contract_on_buyer(ctx, contract_id)
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


    // Hourly gigs part //
    /* 
        Buyer will start a working hourly contract between buyer and seller 
        by calling this function with hourly rate, weekly limit and dispute fee. 
    */
    
    pub fn start_hourly_contract_on_buyer(ctx: Context<StartHourlyContractOnBuyerContext>, contract_id: String, hourly_rate: u32, weekly_hours_limit: u32, dispute: u64, deadline: u32) -> Result<()> {
        instructions::start_hourly_contract_on_buyer::start_hourly_contract_on_buyer(ctx, contract_id, hourly_rate, weekly_hours_limit, dispute, deadline)
    }

    /* 
        Seller will activate the contract after checking all conditions that buyer set 
        when creating the contract.
    */
    pub fn activate_hourly_contract(ctx: Context<ActivateHourlyContractContext>, contract_id: String, with_dispute: bool) -> Result<()> {
        instructions::activate_hourly_contract::activate_hourly_contract(ctx, contract_id, with_dispute)
    }

    /* 
        Freelancer will update his worked hours per week
    */
    pub fn update_worked_hour(ctx: Context<UpdateWorkedHourContext>, contract_id: String, week_worked_hour: u32) -> Result<()> {
        instructions::update_worked_hour::update_worked_hour(ctx, contract_id, week_worked_hour)
    }

    /* 
        Client will pay worked hours of freelancers per week
    */
    pub fn pay_worked_hour(ctx: Context<PayWorkedHourContext>, contract_id: String, amount: u64) -> Result<()> {
        instructions::pay_worked_hour::pay_worked_hour(ctx, contract_id, amount)
    }

    /* 
        Freelancer will approve to get paid
    */
    pub fn seller_approve_hourly_contract(ctx: Context<SellerApproveHourlyContractContext>, contract_id: String, seller_satisfied: bool) -> Result<()> {
        instructions::seller_approve_hourly_contract::seller_approve_hourly_contract(ctx, contract_id, seller_satisfied)
    }

    /* 
        Client will end hourly contracts
    */
    pub fn end_hourly_contract(ctx: Context<EndHourlyContractContext>, contract_id: String) -> Result<()> {
        instructions::end_hourly_contract::end_hourly_contract(ctx, contract_id)
    }

    /* 
        Client will pause hourly contracts
    */
    pub fn pause_hourly_contract(ctx: Context<PauseHourlyContractContext>, contract_id: String) -> Result<()> {
        instructions::pause_hourly_contract::pause_hourly_contract(ctx, contract_id)
    }

    /* 
        Client will resume hourly contracts
    */
    pub fn resume_hourly_contract(ctx: Context<ResumeHourlyContractContext>, contract_id: String) -> Result<()> {
        instructions::resume_hourly_contract::resume_hourly_contract(ctx, contract_id)
    }
}
