use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{CloseAccount, close_account}, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}};

use crate::state::Escrow;


#[derive(Accounts)]

pub struct  Take <'info> {
    
    #[account(mut)]
    pub taker: Signer <'info>,

    pub maker: SystemAccount<'info>,

    #[account(
      address= escrow.mint_a
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(address= escrow.mint_b)]
    pub mint_b: InterfaceAccount<'info, Mint>,


     #[account(
        init_if_needed,
        payer= taker,
        associated_token::mint= mint_a,
        associated_token::authority= taker,
        associated_token::token_program= token_program, 
     )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,
    
    #[account(mut,
        associated_token::mint= mint_b,
        associated_token::authority= taker,
        associated_token::token_program= token_program,
    
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer= taker,
        associated_token::mint= mint_b,
        associated_token::authority= maker,
        associated_token::token_program= token_program,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,




    #[account(mut,
      associated_token::mint= mint_a,
      associated_token::authority = escrow,
      associated_token::token_program = token_program,)]

      pub vault: InterfaceAccount<'info, TokenAccount>,


    #[account(
        mut,
       close=taker,
       has_one=mint_a,
       has_one=mint_b,
       has_one=maker,
       seeds= [b"escrow", maker.key.as_ref(), escrow.seed.to_le_bytes().as_ref()],
       bump= escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program:Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,


}

impl <'info> Take <'info> {

    pub fn deposit(&mut self)-> Result<()> {
     
     let cpi_program = self.token_program.to_account_info();
     let cpi_accounts = TransferChecked{
        from:self.taker_ata_b.to_account_info(),
        mint: self.mint_b.to_account_info(),
        to: self.maker_ata_b.to_account_info(),
        authority: self.taker.to_account_info(),

     };

     let ctx_cpi = CpiContext::new(cpi_program, cpi_accounts);

         transfer_checked(ctx_cpi, self.escrow.recieve, self.mint_b.decimals);

         Ok(())
    }

    pub fn release(&mut self)-> Result<()>{

        let program = self.token_program.to_account_info();

        let accounts = TransferChecked{
            from:self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
    
         };

         let seed_bytes = self.escrow.seed.to_le_bytes();

         let seeds= &[b"escrow", self.escrow.maker.as_ref(), seed_bytes.as_ref(), &[self.escrow.bump]];

         let signer_seeds = &[&seeds[..]];
        

        let ctx_cpi = CpiContext::new_with_signer(program, accounts, signer_seeds);

        transfer_checked(ctx_cpi, self.escrow.recieve, self.mint_a.decimals);

        Ok(())


        }

        pub fn close(&mut self)-> Result<()>{

            let cpi_program = self.token_program.to_account_info();
            let cpi_accounts= CloseAccount{
                account:self.vault.to_account_info(),
                destination:self.taker.to_account_info(),
                authority:self.escrow.to_account_info()

            };

            let seed_bytes = self.escrow.seed.to_le_bytes();

            let seeds=  &[b"escrow", self.escrow.maker.as_ref(), seed_bytes.as_ref(), &[self.escrow.bump]];

            let signer_seeds = &[&seeds[..]];

            let ctx_cpi = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

            close_account(ctx_cpi);

            Ok(())
        }
    }