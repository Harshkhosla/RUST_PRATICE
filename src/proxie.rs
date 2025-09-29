use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::{next_account_info, AccountInfo},entrypoint, entrypoint:: ProgramResult, instruction::{AccountMeta, Instruction}, program::invoke, pubkey::Pubkey};

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
    let dataexecutions = next_account_info(&mut acc)?;
    let doubleaccountpubkey = next_account_info(&mut acc)?;
    let instructions = Instruction{
        program_id:*doubleaccountpubkey.key,
        accounts:vec![AccountMeta{
            is_signer:true,
            is_writable:true,
            pubkey:*dataexecutions.key
        }],
        data:vec![]
    };
    invoke(&instructions , &[dataexecutions.clone()])?;

Ok(())
}