use anchor_lang::prelude::*;
#[account]
pub struct Proposal {
    pub owner: Pubkey,
    pub title: String,
    pub brief: String,
    pub agree_votes: u64, // Number of "Agree" votes
    pub disagree_votes: u64,
}

#[account]
pub struct VoteRecord {
    pub proposal_id: Pubkey, // Linked proposal ID
    pub voter: Pubkey,       // Voter's public key
    pub has_voted: bool,     // Prevent double voting
    pub vote: String,
}
