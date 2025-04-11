use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;


declare_id!("FsLwwctiKpnAgaCi9tKh3oXhmYZ8CPY7GY21XcKMdo2K");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed:u64, deposit:u64,recieve:u64,) -> Result<()> {
    ctx.accounts.deposit(deposit)?;
    ctx.accounts.init_escrow(seed, recieve, &ctx.bumps)
      

    }
}

#[derive(Accounts)]
pub struct Initialize {
                             
}
