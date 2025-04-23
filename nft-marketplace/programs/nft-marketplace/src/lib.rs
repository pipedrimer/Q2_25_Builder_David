#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("nGiFCANMMQBjNR48KtcigCoVcbwjQZvCeDoqUm7aSoG");

pub mod instructions;
pub mod state;
pub mod error;

pub use instructions::*;


#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee:u16) -> Result<()> {

        ctx.accounts.init(name, fee, &ctx.bumps)?;

        Ok(())
        
    }

    pub fn list(ctx: Context<List>, price:u64)-> Result<()> {

       ctx.accounts.create_listing(price, &ctx.bumps)?;
       ctx.accounts.deposit_nft()?;

       Ok(())

    }

    pub fn delist(ctx:Context<Delist>)-> Result<()>{

        ctx.accounts.withdraw_nft()?;
        ctx.accounts.close_listing_account()?;

        Ok(())
    }

    pub fn purchase (ctx:Context<Take>, price: u64)-> Result<()>{
        ctx.accounts.send_sol()?;
        ctx.accounts.send_nft()?;
        ctx.accounts.close_vault()?;

        Ok(())
    }

}


