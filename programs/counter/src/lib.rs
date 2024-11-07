use anchor_lang::prelude::*;
use anchor_safe_math::{SafeMath};

declare_id!("Cf9qZuP2WabAWALeqXTKDTnfu8HMqSczFK8AFBDmnzVS");

#[program]
pub mod counter {
    use super::*;

    pub fn init_counter(ctx: Context<InitCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.owner = ctx.accounts.owner.key();
        counter.count = 0;

        Ok(())
    }

    pub fn update(ctx: Context<UpdateCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter.count.safe_add(1)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCounter<'info> {
    
    #[account(
        init,
        seeds = ["counter".as_bytes(), owner.key().as_ref()],
        bump,
        space = 8 + Counter::INIT_SPACE,
        payer = owner
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateCounter<'info> {

    #[account(
        mut,
        has_one = owner,
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub owner: Pubkey,
    pub count: u64,
}