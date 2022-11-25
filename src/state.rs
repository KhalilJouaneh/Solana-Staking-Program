// state.rs
use borsh:: { BorshSerialize, BorshDeserialize };
use solana_program:: {
    program_pack::{ IsInitialized, Sealed },
    pubkey::Pubkey,
    clock::UnixTimestamp
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserStakeInfo {
    pub is_initialized: bool,  //1 bit
    pub token_account: Pubkey,  //32 bit
    pub stake_start_time: UnixTimestamp, //64 bit
    pub last_stake_redeem: UnixTimestamp, //64 bit
    pub user_pubkey: Pubkey, //32 bit
    pub stake_state: StakeState, //1 bit
}


impl UserStakeInfo {
    pub const SIZE: usize = 1 + 32 + 8 + 8 + 32 + 1;
}


impl Sealed for UserStakeInfo { }

impl IsInitialized for UserStakeInfo {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum StakeState {
    Staked,
    Unstaked
}