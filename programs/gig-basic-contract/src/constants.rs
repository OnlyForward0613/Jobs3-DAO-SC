use anchor_lang::prelude::Pubkey;

pub const CONTRACT_SEED: &str = "gig_contract";
pub const CONTRACT_NATIVE_SEED: &str = "gig_contract_native";

pub const DECIMAL: u32 = 6; // 8 for BPT, 6 for USDC

pub const ADMIN_ADDRESS: Pubkey = anchor_lang::solana_program::pubkey!("CxMudY9Vyw4p5fx1ZY173GHH2Q1ewZFo2YWmd8sozquQ"); 
// pub const PAY_TOKEN_MINT_ADDRESS: Pubkey = anchor_lang::solana_program::pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"); // USDC address for mainnet
// pub const PAY_TOKEN_MINT_ADDRESS: Pubkey = anchor_lang::solana_program::pubkey!("HSvEJfU8hXUWFRodbVbRfwYb2p4DwSwpiMaoB7UDRVD4"); // USDT address for devnet
pub const PAY_TOKEN_MINT_ADDRESS: Pubkey = anchor_lang::solana_program::pubkey!("7FctSfSZ9GonfMrybp45hzoQyU71CEjjZFxxoSzqKWT"); // BPT address for devnet
pub const SOL_KEY: Pubkey = anchor_lang::solana_program::pubkey!("So11111111111111111111111111111111111111112");