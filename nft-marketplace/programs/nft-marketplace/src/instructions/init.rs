use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::Marketplace;


#[derive(Accounts)]
#[instruction(name:String)]
pub struct Initialize<'info>{

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(init,
     payer=admin,
     seeds=[b"marketplace", name.as_bytes()],
     bump,
     space= 8+ Marketplace::INIT_SPACE)]

     pub marketplace:Account <'info, Marketplace>,
     

    #[account(
        seeds=[b"treasury", marketplace.key().as_ref() ],
        bump

    )]
    pub treasury: SystemAccount<'info>,
  
    #[account(
        init,
        payer=admin,
        seeds=[b"rewards", marketplace.key().as_ref()],
        bump,
        mint::authority= marketplace,
        mint::decimals= 6,

    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,
    pub token_program : Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>

}

impl <'info> Initialize <'info> {

    pub fn init(&mut self, name:String, fee:u16, bumps:&InitializeBumps,)-> Result<()>{
         
          self.marketplace.set_inner(
            Marketplace{
                admin: self.admin.key(),
                name,
                fee,
                rewards_bump:bumps.reward_mint,
                treasury_bump:bumps.treasury,
                bump:bumps.marketplace,
              }
          );

          Ok(())

    }
}
