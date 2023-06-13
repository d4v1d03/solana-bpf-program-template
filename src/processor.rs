use crate::instruction::MailInstruction;
use solana_program::{
  account_info::AccountInfo,
  entrypoint::ProgramResult,
  msg,
  pubkey::Pubkey,
};

pub struct Processor;
impl Processor {
    pub fn process(
      program_id: &Pubkey;
      accounts: &[AccountInfo],
      instruction_data: &[u8],
    ) -> ProgramResult {
    let instruction = MailInstruction::unpack(instruction_data)?;

    match instruction {
      MailInstruction::InitAccount => {
        msg!("Instruction: InitAccount");
        Self::process_init_account(accounts, program_id)
      }
    }
  }
}