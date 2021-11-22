use anchor_lang::prelude::*;
use anchor_lang::AccountsClose;

declare_id!("Cq1TSA1obVQZzw2YYxvFN6Q5ia5TYxSbwyZQ9JwQCbBL");


#[program]
pub mod myepicproject {
    use anchor_lang::solana_program::{program::invoke, system_instruction::transfer};

    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        // get a referance to the account 
        let base_account = &mut ctx.accounts.base_account;
        // initliase total_count. 
        base_account.total_pools = 0;
        Ok(())
    }
    // old name which I don't want to change as used in other places function is add_pool 
    pub fn add_gif(ctx: Context<AddGif>, image_link: String, pool_name: String, pool_desc: String, win_opt:String, close: u32, verify:String, fee:u8) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let wins: Vec<String> = win_opt.split(';').map(|s| s.trim().to_string()).collect(); //chars().filter(|c| !c.is_whitespace()).collect()
        // make a program address which will hold the SOL for this pool 
        let pool_wallet = &ctx.accounts.pool_wallet;
        let bump = 5;

        let pool = PoolStruct{
            pool_wallet: pool_wallet.to_account_info().key.to_string(),
            pool_id: base_account.total_pools,
            image_link: image_link.to_string(),
            user_address: *base_account.to_account_info().key,
            pool_name: pool_name.to_string(), 
            pool_balance: 0,
            pool_description: pool_desc.to_string(),
            win_options: wins, 
            close_date_time: close, 
            verify_url:verify.to_string(),
            owner_fee: fee,
            result: "".to_string(), 
            closed: false, 
            entries: Vec::new()
        };
        base_account.pool_list.push(pool);
        base_account.total_pools += 1;
        Ok(())
    }

    pub fn add_result(ctx: Context<AddGif>,result:String, pool_id:u32 ) -> ProgramResult{
        //TODO: Make this only callable by the pool owner. 
        let base_account = &mut ctx.accounts.base_account;
        let mut i = 0; 
        let mut found = false;
        for p in &base_account.pool_list {
            if p.pool_id == pool_id {
                found = true;
                break;
            } 
            i += 1;
        }
        if found {
            base_account.pool_list[i].closed = true;
            base_account.pool_list[i].result = result.to_string();
        };
        Ok(())
    }

    pub fn place_bet(ctx: Context<AddGif>, pred: String, pool_id:u32, stake_bal:u32, user:String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let pool_wallet = &mut ctx.accounts.pool_wallet;
        // TODO: check prediction is one of possible options 
        // TODO: Add payment to this function 
        // TODO: make sure today is before the close date. 
        let bet = EntryStruct{
            user: user,
            prediction: pred,
            stake_bal: stake_bal
        };
        let mut i = 0;
        let mut found = false;
        for p in &base_account.pool_list{
            if p.pool_id == pool_id {
                found = true;
                break;
            }
            i += 1;
        };
        if found{
            let sb = stake_bal as u64;
            let account_lamports = **pool_wallet.to_account_info().lamports.borrow();
            let transfer_amount = sb.checked_sub(account_lamports).ok_or(0)?;

            if transfer_amount > 0 {
                invoke( 
                    &transfer(
                        ctx.accounts.user.to_account_info().key,
                        pool_wallet.to_account_info().key,
                        transfer_amount,
                ),
                &[
                    ctx.accounts.user.to_account_info(),
                    pool_wallet.to_account_info(),
                    ctx.accounts.system_program.to_account_info()
                ],    
            )?;
            }
            base_account.pool_list[i].pool_balance += stake_bal as u64;
            base_account.pool_list[i].entries.push(bet);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space= 10240)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(pool_name: String, bump: u8)]
pub struct AddGif<'info> {
    #[account(mut, signer)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(init, seeds=[pool_name.as_bytes(),b"pool_wallet"], space=9000,bump = bump, payer=user)]
    pub pool_wallet: Account<'info, PoolWallet>,
    // #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}



#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct PoolStruct{
    pub pool_wallet: String,
    pub pool_id: u32,
    pub image_link: String,
    pub user_address: Pubkey,
    pub pool_name: String, 
    pub pool_balance: u64, 
    pub pool_description: String,
    pub win_options: Vec<String>,
    pub close_date_time: u32, 
    pub verify_url: String, 
    pub owner_fee: u8,
    // TODO: to allow for pools with more then 1 winning result perhaps result should be an array?
    pub result: String,
    pub closed: bool,
    pub entries: Vec<EntryStruct>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct EntryStruct{
    pub user: String,
    pub prediction: String,
    pub stake_bal: u32,
}

//Tell solana we want to store on this account 
#[account]
pub struct BaseAccount {
    pub total_pools: u32,
    pub pool_list: Vec<PoolStruct>,
}

#[account]
pub struct PoolWallet{
    pub balance: u64 
}