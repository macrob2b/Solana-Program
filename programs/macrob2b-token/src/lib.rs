use anchor_lang::prelude::*;

declare_id!("Hs5A3fAN5GpEVpHPSZtmAix8RPf171Ry89EQQkVtoVwP");

#[program]
pub mod macrob2b_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
