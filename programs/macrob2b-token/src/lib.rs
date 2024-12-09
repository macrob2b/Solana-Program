use anchor_lang::prelude::*;

declare_id!("BFQBgQicbMZUvgbC6AkLQoevG8QYR7ryf42EpDTCMcko");

#[program]
pub mod macrob2b_program {

    use super::*;

    pub fn submit_proposal(
        ctx: Context<SubmitProposal>,
        title: String,
        brief: String,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.owner = ctx.accounts.user.key();
        proposal.title = title;
        proposal.brief = brief;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitProposal<'info> {
    #[account(init,payer=user,space=8+32+104+804)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Proposal {
    pub owner: Pubkey,
    pub title: String,
    pub brief: String,
}
