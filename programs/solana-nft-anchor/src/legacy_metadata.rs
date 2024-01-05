use anchor_lang::solana_program::pubkey::Pubkey;
use mpl_token_metadata::ID;


pub const PREFIX: &str = "metadata";
pub const EDITION: &str = "edition";

pub fn find_master_edition_account(mint: &Pubkey) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[
      PREFIX.as_bytes(),
      ID.as_ref(),
      mint.as_ref(),
      EDITION.as_bytes(),
    ],
    &ID,
  )
}

pub fn find_metadata_account(mint: &Pubkey) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[PREFIX.as_bytes(), ID.as_ref(), mint.as_ref()],
    &ID,
  )
}
