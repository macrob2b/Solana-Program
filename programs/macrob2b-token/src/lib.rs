use anchor_lang::prelude::*;

declare_id!("BFQBgQicbMZUvgbC6AkLQoevG8QYR7ryf42EpDTCMcko");

use instructions::*;
mod error;
mod instructions;
mod state;
#[program]
pub mod macrob2b_program {

    use super::*;

    pub fn create_proposal(
        ctx: Context<SubmitProposal>,
        title: String,
        brief: String,
    ) -> Result<()> {
        proccess_create_proposal(ctx, title, brief)
    }

    pub fn delete_proposal(ctx: Context<DeleteProposal>) -> Result<()> {
        proccess_delete_proposal(ctx)
    }

    pub fn vote(ctx: Context<Vote>, agree: bool) -> Result<()> {
        proccess_vote(ctx, agree)
    }
}
