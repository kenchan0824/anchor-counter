use anchor_lang::prelude::*;

declare_id!("Cf9qZuP2WabAWALeqXTKDTnfu8HMqSczFK8AFBDmnzVS");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
