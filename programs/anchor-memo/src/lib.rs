use anchor_lang::prelude::*;

declare_id!("FQAYV6Ru2VyZU24EpoY1Ms8nZHTn5qNEdYZopgzhfHkH");

// NOTE: Replace with your wallet's public key
const OWNER: &str = "BVMBzPv6uyZzW7jjL578SWiWd8XmZ9YJN2UBEmpZY6aY";

#[program]
pub mod anchor_memo {
    use super::*;

    #[access_control(check(&ctx))]

    pub fn post_message(ctx: Context<OnlyOwner>, data: Vec<u8>) -> Result<()> {
      // Convert raw instruction data to string
      let instruction_string = String::from_utf8(data)
          .map_err(|_| OnlyOwnerError::InvalidInstructionData)?;

      // Log the instruction string
      msg!("{}", instruction_string);

      Ok(())
  }
}
fn check(ctx: &Context<OnlyOwner>) -> Result<()> {
    // Check if signer === owner
    require_keys_eq!(
        ctx.accounts.signer_account.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerError::NotOwner
    );

    Ok(())
}

#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    signer_account: Signer<'info>,
}

// An enum for custom error codes
#[error_code]
pub enum OnlyOwnerError {
    NotOwner,
    InvalidInstructionData,
}
