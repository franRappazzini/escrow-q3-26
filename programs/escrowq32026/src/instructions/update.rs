use anchor_lang::prelude::*;

use crate::{state::Escrow, ESCROW_SEED};

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    maker: Signer<'info>,

    #[account(
        mut,
        seeds = [ESCROW_SEED, maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = maker,
    )]
    pub escrow: Account<'info, Escrow>,

    system_program: Program<'info, System>,
}

impl<'info> Update<'info> {
    pub fn update(&mut self, receive: u64) -> Result<()> {
        self.escrow.receive = receive;

        Ok(())
    }
}
