use anchor_lang::prelude::*;

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

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub owner: Pubkey,
    pub count: u64,
}