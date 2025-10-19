use anchor_lang::prelude::*;

declare_id!("3yLnRn15E1q4QqezAFASxTj3ahhiwkomjBotB7DuHVbg");

#[program]
pub mod whitelist_transfer_hook {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
