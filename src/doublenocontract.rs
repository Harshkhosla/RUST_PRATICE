use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::{next_account_info, AccountInfo},entrypoint, entrypoint::ProgramResult, pubkey::Pubkey};

entrypoint!(process_instruction);


#[derive(BorshSerialize,BorshDeserialize)]
struct OnChainData{
    count:u32
}

pub  fn process_instruction(
     _program_id:&Pubkey ,
    accounts:&[AccountInfo],
    instruction_data:&[u8]
)->ProgramResult{
    let mut iter = accounts.iter();
    let account = next_account_info( &mut iter)?;
    let mut  acountdata = OnChainData::try_from_slice(& account.data.borrow_mut())?;
    // account.data.
    if acountdata.count == 0{
        acountdata.count=1;
    }else{
         acountdata.count=acountdata.count*2;
    }
    acountdata.serialize(&mut * account.data.borrow_mut());
Ok(())
}