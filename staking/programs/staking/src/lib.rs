use anchor_lang::prelude::*;

declare_id!("4nDa3eYUBer8wNmyLDcqwXzLH1s5RYUacJ15w7izh5Py");

#[program]
pub mod staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
