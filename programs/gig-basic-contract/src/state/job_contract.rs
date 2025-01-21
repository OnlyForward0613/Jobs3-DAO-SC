use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::constants::*;
use crate::errors::{
    GigContractError
};

#[account]
pub struct JobContract {
    pub employer: Pubkey,
    pub employee: Pubkey,
    pub employer_referral: Pubkey,
    pub employee_referral: Pubkey,
    pub contract_id: String, 
    pub start_time: u32,
    pub deadline: u32,
    pub amount: u64,
    pub dispute: u64,
    pub split: bool,
    pub employer_satisfied: bool, // regarding split decision
    pub employer_approved: bool,
    pub employee_approved: bool,
    pub admin_approved: bool,
    pub status: JobContractStatus,
    pub featured: bool,
    pub featured_day: u8, 
}

impl JobContract {
    pub const LEN: usize = size_of::<Self>();
}

impl Default for JobContract {
    #[inline]
    fn default() -> JobContract {
        JobContract {
            contract_id: "".to_string(),
            employer: Pubkey::default(),
            employee: Pubkey::default(),
            employer_referral: anchor_lang::solana_program::pubkey!("3x9USDofKPb6rREu2dWe9rcvT4QMHQS1PrJ13WuZ1QL3"),
            employee_referral: anchor_lang::solana_program::pubkey!("3x9USDofKPb6rREu2dWe9rcvT4QMHQS1PrJ13WuZ1QL3"),
            start_time: 0,
            deadline: 0,
            amount: 0,
            dispute: 0,
            split: false,
            employer_satisfied: false,
            employer_approved: false,
            employee_approved: false,
            admin_approved: false,
            status: JobContractStatus::NoExist,
            featured: false,
            featured_day: 0,
        }
    }
}

#[derive(Eq, AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum JobContractStatus {
    Initialized,
    NotInitialized,
    NoExist,
    Created,
    Active,
    Accepted,
    Pending,
    Dispute,
    Completed,
    Listed
}
