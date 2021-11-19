use anchor_lang::prelude::*;

declare_id!("Cq1TSA1obVQZzw2YYxvFN6Q5ia5TYxSbwyZQ9JwQCbBL");


#[program]
pub mod myepicproject {
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
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(init, payer = user, space=9000)]
    pub pool_wallet: Account<'info, PoolWallet>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

// #[derive(Accounts)]
// pub struct PlaceBet<'info>{
//     #[account(mut)]
//     pub base_account: Account<'info, BaseAccount>,
// }


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