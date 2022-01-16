use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount};
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
#[instruction(wallet_authority_bump: u8, vault_authority_bump: u8)]
pub struct AcceptOffer<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(mut)]
    pub site_wallet: AccountInfo<'info>,
    
    #[account(
        seeds = [b"wallet_authority".as_ref()],
        bump = wallet_authority_bump
    )]
    pub wallet_authority: AccountInfo<'info>,

    #[account(
        mut,
        constraint = product_info_account.owner == program_id
    )]
    pub product_info_account: AccountInfo<'info>,

    #[account(
        mut,
        constraint = offer_nft_token_account.mint == *nft_mint_account.key,
        constraint = offer_nft_token_account.owner == seller.key()
    )]
    pub offer_nft_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        owner = token::Token::id()
    )]
    pub nft_mint_account: AccountInfo<'info>,

    #[account(
        mut,
        constraint = vault_token_account.mint == *nft_mint_account.key
    )]
    pub vault_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [b"vault_authority".as_ref()],
        bump = vault_authority_bump
    )]
    pub vault_authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn process(
    ctx: Context<AcceptOffer>,
    offer_id: u16,
    wallet_authority_bump: u8,
    vault_authority_bump: u8
) -> ProgramResult {
    // TODO
    Ok(())
}