pub const ERROR_PLAYER_ALREADY_EXIST:u32 = 1;
pub const ERROR_PLAYER_NOT_EXIST:u32 = 2;
pub const ERROR_PLAYER_NOT_ENOUGH_BALANCE:u32 = 3;
pub const ERROR_PLAYER_NOT_IN_CURRENT_ROUND:u32 = 4;
pub const ERROR_CURRENT_ROUND_STARTED:u32 = 5;
pub const ERROR_CURRENT_ROUND_IN_PREPARATION:u32 = 6;
pub const ERROR_CURRENT_BET_NOT_FINISHED:u32 = 7;


pub fn decode_error(e: u32) -> &'static str {
    match e {
        ERROR_PLAYER_NOT_EXIST => "PlayerNotExist",
        ERROR_PLAYER_ALREADY_EXIST => "PlayerAlreadyExist",
        ERROR_PLAYER_NOT_ENOUGH_BALANCE => "PlayerBalanceInsufficient",
        ERROR_PLAYER_NOT_IN_CURRENT_ROUND => "PlayerNotInCurrentRound",
        ERROR_CURRENT_ROUND_STARTED => "CurrentRoundStarted",
        ERROR_CURRENT_ROUND_IN_PREPARATION => "CurrentRoundInPreparation",
        ERROR_CURRENT_BET_NOT_FINISHED => "CurrentBetNotFinished",
        _ => "Unknown"
    }

}


