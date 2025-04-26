use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{transfer_checked, Mint, TransferChecked, TokenAccount, TokenInterface},

};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed:u64)]

pub struct Make <'info> {
    #[account(mut)]
     pub maker: Signer<'info>,
    
    #[account(
        mint::token_program = token_program,

    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program= token_program,
    )]

    pub mint_b: InterfaceAccount<'info, Mint>,
    
    #[account(mut,
        associated_token::mint= mint_a,
        associated_token::authority = maker,
        associated_token::token_program=token_program,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        init,
        payer=maker,
        space= 8 + Escrow::INIT_SPACE,
        seeds=[b"escrow", maker.key.as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        
    )]
    pub escrow: Account<'info, Escrow>,
     

    #[account(
        init,
        payer=maker,
        associated_token::mint = mint_a,
        associated_token::authority= escrow,
        associated_token::token_program= token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface <'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>


}

impl <'info> Make <'info> {
    pub fn init_escrow (&mut self, seed:u64, recieve:u64, bumps:&MakeBumps)-> Result<()>{
         self.escrow.set_inner(Escrow{
           maker:self.maker.key(),
           seed,
           mint_a: self.mint_a.key(),
           mint_b: self.mint_b.key(),
           recieve,
           bump:bumps.escrow





         });
         Ok(())
        
    }

    pub fn deposit (&mut self, amount: u64)-> Result<()> {

        let cpi_program = self.token_program.to_account_info();
        let transfer_accounts = TransferChecked{
            from:self.maker_ata_a.to_account_info(),
            mint:self.mint_a.to_account_info(),
            to:self.vault.to_account_info(),
            authority:self.maker.to_account_info(), 
};

  let cpi_ctx = CpiContext::new(cpi_program, transfer_accounts);

  transfer_checked(cpi_ctx, amount , self.mint_a.decimals);
  Ok(())

}

    
     }
    

