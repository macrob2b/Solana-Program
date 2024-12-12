use crate::error::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

//Create proposal
#[derive(Accounts)]
pub struct SubmitProposal<'info> {
    #[account(init,payer=user,space=8+32+104+804)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn proccess_create_proposal(
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

//End create propsoal

//Delete proposal
#[derive(Accounts)]
pub struct DeleteProposal<'info> {
    #[account(mut,close=user)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub user: SystemAccount<'info>,
}

pub fn proccess_delete_proposal(ctx: Context<DeleteProposal>) -> Result<()> {
    let account = &ctx.accounts.proposal;
    if account.owner != ctx.accounts.user.key() {
        return Err(ErrorCode::Unauthorized.into());
    }
    Ok(())
}

//End Delete propsoal
