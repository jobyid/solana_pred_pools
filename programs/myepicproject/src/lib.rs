use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        // get a referance to the account 
        let base_account = &mut ctx.accounts.base_account;
        // initliase total_count. 
        base_account.total_pools =0;
        Ok(())
    }
    pub fn add_gif(ctx: Context<AddGif>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_pools += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space= 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

//Tell solana we want to store on this account 
#[account]
pub struct BaseAccount {
    pub total_pools: u64,
}