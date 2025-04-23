
use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};

declare_id!("GLwHHbTtp48L4xN1XXBmANTV2yK94Jjptof3bc7vYNTa");

#[program]
pub mod vault_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps);
        Ok(())
        
    }
    pub fn deposit(ctx: Context<Payment>, amount:u64) -> Result<()>{
      ctx.accounts.deposit(amount);
      Ok(())
    }
    pub fn withdraw(ctx: Context<Payment>, amount:u64) -> Result<()>{
        ctx.accounts.withdraw(amount);
        Ok(())
    }
    pub fn close(ctx: Context<Close>) -> Result<()>{
       ctx.accounts.close();
       Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer= user,
        seeds= [b"state", user.key().as_ref()],
        bump,
        space= 8 + VaultState::INIT_SPACE,
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(seeds= [b"vault", vault_state.key().as_ref()], bump,)]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
   
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
  
      self.vault_state.vault_bump = bumps.vault;
      self.vault_state.state_bump= bumps.vault_state;

      Ok(())
    }
  }

#[derive(Accounts)]

pub struct Payment<'info>{
    #[account(mut)]
    pub user: Signer <'info>,
    #[account(
        seeds= [b"state", user.key().as_ref()], bump=vault_state.state_bump, 
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(seeds= [b"vault", vault_state.key().as_ref()], bump= vault_state.vault_bump,)]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount:u64) -> Result<()>{
        let cpi_program = self.system_program.to_account_info();
        let cpi_account = Transfer{
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };
            let cpi_ctx = CpiContext::new(cpi_program, cpi_account);
            transfer(cpi_ctx, amount)

    }

    pub fn withdraw (&mut self, amount:u64) -> Result<()>{

        let cpi_program= self.system_program.to_account_info();
        let cpi_account= Transfer{
            from: self.vault.to_account_info(),
            to: self.user.to_account_info()
    };
    let seeds = &[b"vault", self.vault_state.to_account_info().key.as_ref(),
    &[self.vault_state.vault_bump],];

    let signer_seed= &[&seeds[..]];
    let cpi_ctx= CpiContext::new_with_signer(cpi_program, cpi_account, signer_seed);

    transfer(cpi_ctx, amount)
}
}

#[derive(Accounts)]

pub struct Close <'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(mut,
        seeds= [b"state", user.key().as_ref()], bump= vault_state.state_bump,
        close=user,
    )]
    vault_state: Account<'info, VaultState>,
    #[account(seeds= [b"vault", vault_state.key().as_ref()], bump= vault_state.vault_bump,)]
    vault: SystemAccount<'info>,
    system_program: Program<'info, System>,
    
}

impl <'info> Close <'info>{
    pub fn close(&mut self) -> Result<()>{

        let cpi_program= self.system_program.to_account_info();
        let cpi_account= Transfer{
            from: self.vault.to_account_info(),
            to: self.user.to_account_info()
    };
    let seeds = &[b"vault", self.vault_state.to_account_info().key.as_ref(),
    &[self.vault_state.vault_bump],];

    let signer_seed= &[&seeds[..]];
    let cpi_ctx= CpiContext::new_with_signer(cpi_program, cpi_account, signer_seed);

    transfer(cpi_ctx, self.vault.lamports())
    }
}

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

