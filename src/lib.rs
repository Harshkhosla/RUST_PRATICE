use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::{next_account_info, AccountInfo},entrypoint::{self,  ProgramResult},instruction::{AccountMeta, Instruction}, program::{invoke, invoke_signed}, pubkey::Pubkey, system_instruction::create_account};

#[derive(BorshSerialize,BorshDeserialize)]
struct OnChainData{
    count:u32
}

entrypoint!(middleContract);

pub fn middleContract(
    _program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data :&[u8]
)->ProgramResult{
    let mut acc = accounts.iter();
    let pad = next_account_info(&mut acc)?;
    let user_Account = next_account_info(&mut acc)?;
    let system_progrom = next_account_info(&mut acc)?;
    let seeds = &[user_Account.key.as_ref(), b"user"];
    let (pad_pub_key , bumps )= Pubkey::find_program_address(seeds, _program_id);
    let inx = create_account(
        user_Account.key, 
        pad.key, 
        1000000, 
        8, 
        _program_id
    );

    invoke_signed(&inx, accounts, &[&[user_Account.key.as_ref(), b"user", &[bumps]]])?;
    // invoke(&instructions , &[dataexecutions.clone()])?;

Ok(())
}