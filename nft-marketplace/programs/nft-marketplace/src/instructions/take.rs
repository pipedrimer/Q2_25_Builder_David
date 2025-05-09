use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{associated_token::AssociatedToken,
    token::{close_account, transfer_checked, CloseAccount, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::{Listing,Marketplace};

#[derive(Accounts)]
pub struct Take<'info>{
 #[account(mut)]
 pub taker: Signer<'info>,
 #[account(mut)]
 pub maker: SystemAccount<'info>,
 
 pub maker_mint: InterfaceAccount<'info, Mint>,
 #[account(
    init_if_needed,
    payer = taker,
    associated_token::authority= taker,
    associated_token::mint= maker_mint,
    associated_token::token_program = token_program,
 )]
 pub taker_ata: InterfaceAccount<'info, TokenAccount>,
 

 #[account(
    seeds=[b"marketplace", marketplace.name.as_bytes()],
    bump= marketplace.bump,
)]

pub marketplace:Account <'info, Marketplace>,
#[account(mut,
    associated_token::authority= listing,
    associated_token::mint= maker_mint,
    associated_token::token_program = token_program
    

)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(mut,
        close=maker,
        seeds= [b"listing", maker_mint.key().as_ref(), marketplace.key().as_ref(), maker.key().as_ref()],
        bump=listing.bump,
    
    )]
        pub listing: Account <'info, Listing>, 
        #[account(
            seeds = [b"treasury", marketplace.key().as_ref()],
            bump=marketplace.treasury_bump
          )]
          pub treasury: SystemAccount<'info>,

        pub token_program : Interface<'info, TokenInterface>,
        pub associated_token_program: Program<'info, AssociatedToken>,
        pub system_program: Program<'info, System>


}

impl <'info> Take <'info>{

    pub fn send_sol(&mut self)-> Result<()> {
        let fee = (self.marketplace.fee as u64)
        .checked_mul(self.listing.price)
        .unwrap()
        .checked_div(10000_u64)
        .unwrap();
    
    let amount= (self.listing.price).checked_sub(fee).unwrap();

    let cpi_program = self.system_program.to_account_info();

    let cpi_accounts=  Transfer{
        from:self.taker.to_account_info(),
        to:self.maker.to_account_info(),
    };

     let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts );

     transfer(cpi_ctx, amount)?;


     let cpi_program = self.system_program.to_account_info();

     let cpi_accounts=  Transfer {
         from:self.taker.to_account_info(),
         to:self.treasury.to_account_info(),
     };

     let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts );
     transfer(cpi_ctx, fee)?;

     Ok(())

}

pub fn send_nft(&mut self)-> Result<()>{

    let cpi_program = self.token_program.to_account_info();

    let cpi_accounts=  TransferChecked{
        from:self.vault.to_account_info(),
        to:self.taker_ata.to_account_info(),
        mint:self.maker_mint.to_account_info(),
        authority:self.listing.to_account_info(),
    };
 
 
    // let seeds= &[b"listing", &self.maker_mint.key().as_ref(), self.marketplace.to_account_info().key.as_ref(), self.maker.key().as_ref(), &[self.listing.bump]];

    // let signer_seed= &[&seeds[..]];
    let seeds= &[ &self.marketplace.key().to_bytes()[..], 
    &self.maker_mint.key().to_bytes()[..], &self.maker.key().to_bytes()[..], &[self.listing.bump]];

    let signer_seed= &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seed);

    transfer_checked(cpi_ctx, 1, 0);

    Ok(())

}

pub fn close_vault(&mut self)-> Result<()>{

    let cpi_program = self.token_program.to_account_info();

    let cpi_accounts= CloseAccount{
        account: self.vault.to_account_info(),
        destination: self.maker.to_account_info(),
        authority: self.listing.to_account_info()
     };
     let seeds= &[ &self.marketplace.key().to_bytes()[..], 
     &self.maker_mint.key().to_bytes()[..], &self.maker.key().to_bytes()[..], &[self.listing.bump]];
 
     let signer_seed= &[&seeds[..]];
     let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seed);

     close_account(cpi_ctx);

     Ok(())
 
}
}