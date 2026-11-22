
use anchor_lang::prelude::*;

declare_id!("9vWHdvsGqbBPRzQDSzpbgGZCnDwTo5fBnzvN1W1rBT6b");

#[program]
pub mod voting {
    use super::*;

    // 1. CREATE A POLL
    pub fn init_poll(
        ctx: Context<InitPoll>,
        poll_id: u64,
        start: i64,
        end: i64,
        name: String,
        description: String,
    ) -> Result<()> {

        require!(
            start < end,
            VotingError::InvalidVotingPeriod
        );

        let poll = &mut ctx.accounts.poll_account;

        poll.poll_id = poll_id;
        poll.poll_name = name;
        poll.poll_description = description;
        poll.poll_voting_start = start;
        poll.poll_voting_end = end;
        poll.poll_option_index = 0;

        Ok(())
    }

    // 2. CREATE A CANDIDATE
    pub fn init_candidate(
        ctx: Context<InitCandidate>,
        poll_id: u64,
        candidate_index: u64,
        candidate_name: String,
    ) -> Result<()> {

        require!(
            candidate_index == ctx.accounts.poll_account.poll_option_index,
            VotingError::InvalidCandidateIndex
        );

        let candidate = &mut ctx.accounts.candidate_account;

        candidate.poll_id = poll_id;
        candidate.candidate_index = candidate_index;
        candidate.candidate_name = candidate_name;
        candidate.votes = 0;

        ctx.accounts.poll_account.poll_option_index += 1;

        Ok(())
    }

    // 3. VOTE FOR A CANDIDATE
    pub fn vote(
        ctx: Context<Vote>,
        poll_id: u64,
        candidate_index: u64,
    ) -> Result<()> {

        let poll = &ctx.accounts.poll_account;

        let candidate = &mut ctx.accounts.candidate_account;

        let current_time = Clock::get()?.unix_timestamp;

        require!(
            current_time >= poll.poll_voting_start,
            VotingError::VotingNotStarted
        );

        require!(
            current_time <= poll.poll_voting_end,
            VotingError::VotingEnded
        );

        require!(
            candidate.poll_id == poll_id &&
            candidate.candidate_index == candidate_index,
            VotingError::InvalidCandidate
        );

        candidate.votes = candidate.votes
            .checked_add(1)
            .ok_or(VotingError::VoteCountOverflow)?;

        Ok(())
    }
}


// ACCOUNT VALIDATION STRUCTS


// CREATE POLL
#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitPoll<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [
            b"poll_account",
            poll_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    pub system_program: Program<'info, System>,
}


// CREATE CANDIDATE
#[derive(Accounts)]
#[instruction(poll_id: u64, candidate_index: u64)]
pub struct InitCandidate<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"poll_account",
            poll_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        init,
        payer = signer,
        space = 8 + CandidateAccount::INIT_SPACE,
        seeds = [
            b"candidate_account",
            poll_id.to_le_bytes().as_ref(),
            candidate_index.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,

    pub system_program: Program<'info, System>,
}


// VOTE
#[derive(Accounts)]
#[instruction(poll_id: u64, candidate_index: u64)]
pub struct Vote<'info> {

    pub signer: Signer<'info>,

    #[account(
        seeds = [
            b"poll_account",
            poll_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        mut,
        seeds = [
            b"candidate_account",
            poll_id.to_le_bytes().as_ref(),
            candidate_index.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,
}


// ACCOUNT DATA STRUCTURES


#[account]
#[derive(InitSpace)]
pub struct PollAccount {

    pub poll_id: u64,

    #[max_len(32)]
    pub poll_name: String,

    #[max_len(256)]
    pub poll_description: String,

    pub poll_voting_start: i64,

    pub poll_voting_end: i64,

    pub poll_option_index: u64,
}


#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {

    pub poll_id: u64,

    pub candidate_index: u64,

    #[max_len(32)]
    pub candidate_name: String,

    pub votes: u64,
}


// CUSTOM ERROR ENUM

#[error_code]
pub enum VotingError {

    #[msg("Voting start time must be before the end time.")]
    InvalidVotingPeriod,

    #[msg("Voting has not started yet.")]
    VotingNotStarted,

    #[msg("Voting has already ended.")]
    VotingEnded,

    #[msg("Invalid candidate index.")]
    InvalidCandidateIndex,

    #[msg("Candidate does not belong to this poll.")]
    InvalidCandidate,

    #[msg("Vote count overflow.")]
    VoteCountOverflow,
}
