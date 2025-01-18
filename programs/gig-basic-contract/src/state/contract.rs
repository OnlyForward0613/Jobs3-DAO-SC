use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::constants::*;
use crate::errors::{
    GigContractError
};

#[account]
pub struct Contract {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub buyer_referral: Pubkey,
    pub seller_referral: Pubkey,
    pub contract_id: String, 
    pub start_time: u32,
    pub deadline: u32,
    pub amount: u64,
    pub dispute: u64,
    pub split: bool,
    pub seller_satisfied: bool, // regarding split decision
    pub buyer_approved: bool,
    pub seller_approved: bool,
    pub admin_approved: bool,
    pub status: ContractStatus,
    pub payment_method: String
}

impl Contract {
    pub const LEN: usize = size_of::<Self>();
}

impl Default for Contract {
    #[inline]
    fn default() -> Contract {
        Contract {
            contract_id: "".to_string(),
            buyer: Pubkey::default(),
            seller: Pubkey::default(),
            buyer_referral: anchor_lang::solana_program::pubkey!("3x9USDofKPb6rREu2dWe9rcvT4QMHQS1PrJ13WuZ1QL3"),
            seller_referral: anchor_lang::solana_program::pubkey!("3x9USDofKPb6rREu2dWe9rcvT4QMHQS1PrJ13WuZ1QL3"),
            start_time: 0,
            deadline: 0,
            amount: 0,
            dispute: 0,
            split: false,
            seller_satisfied: false,
            buyer_approved: false,
            seller_approved: false,
            admin_approved: false,
            status: ContractStatus::NoExist,
            payment_method: "USDC".to_string()
        }
    }
}

#[derive(Eq, AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ContractStatus {
    NoExist,
    Created,
    Active,
    Accepted,
    Pending,
    Dispute,
    Completed,
}
