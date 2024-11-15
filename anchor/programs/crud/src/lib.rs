#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

#[program]
pub mod crud {
    use super::*;

    pub fn create_journal_entry(
        ctx: Context<CreateEntry>,
        title: String,
        message: String,
    ) -> Result<()> {

        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.owner = *ctx.accounts.owner.key;
        journal_entry.title = title;
        journal_entry.message = message;
        
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct CreateEntry<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        space = ANCHOR_DISCRIMINATOR_SIZE + JournalEntryState::INIT_SPACE,
        payer = owner,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    pub system_program: Program<'info, System>,
}

#[account()]
#[derive(InitSpace)]
pub struct JournalEntryState {
    pub owner: Pubkey,

    #[max_len(50)]
    pub title: String,

    #[max_len(280)]
    pub message: String,
}
