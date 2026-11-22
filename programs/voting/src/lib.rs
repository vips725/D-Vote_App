
use anchor_lang::prelude::*;

declare_id!("9vWHdvsGqbBPRzQDSzpbgGZCnDwTo5fBnzvN1W1rBT6b");

#[program]
pub mod voting {
    use super::*;

    pub fn init_poll(ctx: Context<Initialize>) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx)
    }
}   

#[account]
#[derive(Initspace)]
pub struct PollAccount{
    #[max_length(32)]
    pub poll_name: String,
    #[max_length(256)]
    pub poll_description: String,
    pub poll_voting_start: u64,
    pub poll_voting_end: u64,
    pub poll_option_index:u64,
}

#[account]
#[derive(Initspace)]
pub struct CandidateAccount{
    #[max_length(32)]
    pub candidate_name: String,
    pub votes: u64, 
}


