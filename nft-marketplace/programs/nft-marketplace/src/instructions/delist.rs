use anchor_lang::prelude::*;
use anchor_spl::{
token::{close_account, CloseAccount,transfer_checked, TransferChecked},
associated_token::AssociatedToken, 
token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::state::{Listing, Marketplace};


#[derive(Accounts)]
pub struct Delist<'info> {
  
  pub maker: Signer <'info>,
  
  #[account(
    mut,
    associated_token::mint = maker_mint,
    associated_token::authority = maker,
    associated_token::token_program = token_program,
)]
pub maker_ata: InterfaceAccount<'info, TokenAccount>,

pub maker_mint: InterfaceAccount<'info, Mint>,
#[account(mut,
    associated_token::authority= listing,
    associated_token::mint= maker_mint,
)]
pub vault: InterfaceAccount< 'info, TokenAccount>,

#[account(
    seeds=[b"marketplace", marketplace.name.as_bytes()],
    bump= marketplace.bump,
)]

pub marketplace:Account <'info, Marketplace>,
#[account(mut,
    close= maker,
    seeds= [b"listing", maker_mint.key().as_ref(), marketplace.key().as_ref(), maker.key().as_ref()],
    bump= listing.bump,

)]
    pub listing: Account <'info, Listing>,
    pub token_program : Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
impl <'info> Delist <'info>{
    pub fn withdraw_nft(&mut self) -> Result<()> {

        let seeds= &[ &self.marketplace.key().to_bytes()[..], 
        &self.maker_mint.key().to_bytes()[..], &self.maker.key().to_bytes()[..], &[self.listing.bump]];
    
        let signer_seed= &[&seeds[..]];
     let cpi_program= self.token_program.to_account_info();

     let cpi_accounts= TransferChecked{
        from: self.vault.to_account_info(),
        mint: self.maker_mint.to_account_info(),
        to: self.maker_ata.to_account_info(),
        authority: self.listing.to_account_info()
     };

     let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seed);

     transfer_checked(cpi_ctx,1, 0)?;

     Ok(())
     
    
}

pub fn close_listing_account(&mut self) -> Result<()> {

    let seeds= &[ &self.marketplace.key().to_bytes()[..], 
    &self.maker_mint.key().to_bytes()[..], &self.maker.key().to_bytes()[..], &[self.listing.bump]];

    let signer_seed= &[&seeds[..]];
    let cpi_program= self.token_program.to_account_info();

    let cpi_accounts= CloseAccount{
       account: self.vault.to_account_info(),
       destination: self.maker_ata.to_account_info(),
       authority: self.listing.to_account_info()
    };

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seed);

    close_account(cpi_ctx);

    Ok(())
    
   
}
}