use super::*;

#[derive(Accounts)]
pub struct InitFaucetPDA<'info> {
    #[account(
      init, 
      payer = signer, 
      space = 8 + 8 + 8 + 8 + 1 + 7 + 10, 
      seeds = [b"faucet_pda".as_ref()], 
      bump
  )]
    pub faucet_pda: Account<'info, FaucetPDA>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitRecipientPDA<'info> {
    #[account(
      init, 
      payer = signer, 
      space = 8 + 8 + 10, 
      seeds = [b"recipient_pda".as_ref(), recipient.key().as_ref()], 
      bump
  )]
    pub recipient_pda: Account<'info, RecipientPDA>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub recipient: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferSol<'info> {
  #[account(mut)]
  pub faucet_account: SystemAccount<'info>,

  #[account(mut, seeds = [b"faucet_pda".as_ref()], bump)]
  pub faucet_pda: Account<'info, FaucetPDA>,

  #[account(mut, seeds = [b"recipient_pda".as_ref(), recipient.key().as_ref()], bump)]
  pub recipient_pda: Account<'info, RecipientPDA>,

  #[account(mut)]
  pub recipient: SystemAccount<'info>,

  #[account(mut)]
  pub signer: Signer<'info>,

  pub system_program: Program<'info, System>
}

#[account]
pub struct FaucetPDA {
    pub max_distribution_amount: u64,
    pub cooldown_time: u64, 
    pub last_request_time: u64,
    pub total_dispensed: u64,     
    pub is_active: bool,        
}

#[account]
pub struct RecipientPDA {     
    pub last_request_time: u64,
    pub total_dispensed: u64,   
}
