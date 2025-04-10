use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;


declare_id!("FsLwwctiKpnAgaCi9tKh3oXhmYZ8CPY7GY21XcKMdo2K");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        
    }
}

#[derive(Accounts)]
pub struct Initialize {
                             
}
