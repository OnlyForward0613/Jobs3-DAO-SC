use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::constants::*;
use crate::errors::{
    GigContractError
};

#[account]
pub struct HourlyContract {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub buyer_referral: Pubkey,
    pub seller_referral: Pubkey,
    pub contract_id: String, 
    pub start_time: u32,
    pub deadline: u32,
    pub dispute: u64,
    pub split: bool,
    pub seller_satisfied: bool, // regarding split decision
    pub buyer_approved: bool,
    pub seller_approved: bool,
    pub admin_approved: bool,
    pub paused: bool,
    pub status: HourlyContractStatus,

    pub hourly_rate: u32,
    pub week_worked_hour: u32, // will be set by freelancer
    pub total_worked_hour: u32, // will be updated when doing payment
    pub weekly_hours_limit: u32, // will be set by client
}

impl HourlyContract {
    pub const LEN: usize = size_of::<Self>();
}

impl Default for HourlyContract {
    #[inline]
    fn default() -> HourlyContract {
        HourlyContract {
            contract_id: "".to_string(),
            buyer: Pubkey::default(),
            seller: Pubkey::default(),
            buyer_referral: anchor_lang::solana_program::pubkey!("3x9USDofKPb6rREu2dWe9rcvT4QMHQS1PrJ13WuZ1QL3"),
            seller_referral: anchor_lang::solana_program::pubkey!("3x9USDofKPb6rREu2dWe9rcvT4QMHQS1PrJ13WuZ1QL3"),
            start_time: 0,
            deadline: 0,
            dispute: 0,
            split: false,
            seller_satisfied: false,
            buyer_approved: false,
            seller_approved: false,
            admin_approved: false,
            paused: false,
            status: HourlyContractStatus::NoExist,

            hourly_rate: 0,
            week_worked_hour: 0, // will be set by freelancer
            total_worked_hour: 0, // will be updated when doing payment
            weekly_hours_limit: 0, // will be set by client
        }
    }
}

#[derive(Eq, AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum HourlyContractStatus {
    NoExist,
    Created,
    Active,
    Accepted,
    ReadyToPay,
    Paid,
    Pending,
    Dispute,
    Completed,
    Ended,
}
