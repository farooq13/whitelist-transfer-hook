use anchor_lang::prelude::*;
use anchor_spl::token_interface::{spl_token_metadata_interface::instruction::Initialize, Mint};
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta,
    seeds::Seed,
    state::ExtraAccountMetaList
};

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    //CHECK: ExtraAccountMetaList
    #[account(
        init,
        payer = payer,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        space = ExtraAccountMetaList::size_of(
            InitializeExtraAccountMetaList::extra_account_metas()?.len()
        )?,
        bump,
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeExtraAccountMetaList<'info> {
    pub fn extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
        Ok(
            vec![
                ExtraAccountMeta::new_with_seeds(
                    &[
                        Seed::Literal {
                            bytes: b"whitelist".to_vec(),
                        },
                    ],
                    false, // is_signer
                    false // is_writable
                )?
            ]
        )
    }
}