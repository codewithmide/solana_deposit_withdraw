use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use borsh::{BorshDeserialize, BorshSerialize};

// Define the state of our account
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AccountState {
    pub balance: u64,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to initialize
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize the instruction data
    let instruction = instruction_data[0];

    match instruction {
        0 => initialize_account(account),
        1 => deposit(account, accounts_iter),
        2 => withdraw(account, accounts_iter),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

fn initialize_account(account: &AccountInfo) -> ProgramResult {
    let rent = Rent::get()?;
    if !rent.is_exempt(account.lamports(), account.data_len()) {
        return Err(ProgramError::AccountNotRentExempt);
    }

    let mut account_state = AccountState::try_from_slice(&account.data.borrow())?;
    account_state.balance = 0;
    account_state.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}

fn deposit(account: &AccountInfo, accounts_iter: &mut std::slice::Iter<AccountInfo>) -> ProgramResult {
    let depositor = next_account_info(accounts_iter)?;

    if !depositor.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut account_state = AccountState::try_from_slice(&account.data.borrow())?;
    let amount = depositor.lamports();

    **depositor.lamports.borrow_mut() -= amount;
    **account.lamports.borrow_mut() += amount;
    account_state.balance += amount;

    account_state.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}

fn withdraw(account: &AccountInfo, accounts_iter: &mut std::slice::Iter<AccountInfo>) -> ProgramResult {
    let withdrawer = next_account_info(accounts_iter)?;

    if !withdrawer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut account_state = AccountState::try_from_slice(&account.data.borrow())?;
    let amount = account_state.balance / 10; // 10% of the balance

    **account.lamports.borrow_mut() -= amount;
    **withdrawer.lamports.borrow_mut() += amount;
    account_state.balance -= amount;

    account_state.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}