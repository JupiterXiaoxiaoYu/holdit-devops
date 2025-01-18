use crate::player::HITPlayer;
use crate::error::*;
use zkwasm_rest_abi::WithdrawInfo;
use zkwasm_rest_convention::SettlementInfo;
use zkwasm_rest_convention::CommonState;
use zkwasm_rust_sdk::require;
use crate::state::State;

pub const TICK: u64 = 0;
pub const INSTALL_PLAYER: u64 = 1;
pub const BET_AND_HOLD: u64 = 2;
pub const CHECKOUT: u64 = 3;
pub const WITHDRAW: u64 = 5;
pub const DEPOSIT: u64 = 6;

pub trait CommandHandler {
    fn handle(&self, pid: &[u64; 2], nonce: u64, rand: &[u64; 4]) -> Result<(), u32>;
}

pub enum Command {
    // standard activities
    Activity(Activity),
    // standard withdraw and deposit
    Withdraw(Withdraw),
    Deposit(Deposit),
    // standard player install and timer
    InstallPlayer,
    Tick,
}

pub enum Activity {
    Bet(u64),
    Checkout,
}

impl CommandHandler for Activity {
    fn handle(&self, pid: &[u64; 2], nonce: u64, rand: &[u64; 4]) -> Result<(), u32> {
        let mut player = HITPlayer::get_from_pid(pid);
        match player.as_mut() {
            None => Err(ERROR_PLAYER_NOT_EXIST),
            Some(player) => {
                match self {
                    Activity::Bet(amount) => {
                        let next_round = State::get_global().get_next_active_round()?;
                        player.data.place(*amount, next_round)
                    },
                    Activity::Checkout => {
                        let (round, ratio) = State::get_global().get_active_round_info()?;
                        // This is the selected player; allow them to open the blind box
                        player.data.checkout(ratio, round)?;
                        Ok(())
                    }
                }
            }
        }
    }
}


#[derive (Clone)]
pub struct Withdraw {
    pub data: [u64; 3],
}

impl CommandHandler for Withdraw {
    fn handle(&self, pid: &[u64; 2], nonce: u64, _rand: &[u64; 4]) -> Result<(), u32> {
        let mut player = HITPlayer::get_from_pid(pid);
        match player.as_mut() {
            None => Err(ERROR_PLAYER_NOT_EXIST),
            Some(player) => {
                player.check_and_inc_nonce(nonce);
                let balance = player.data.balance;
                let amount = self.data[0] & 0xffffffff;
                unsafe { require(balance >= amount) };
                player.data.balance -= amount;
                let withdrawinfo =
                    WithdrawInfo::new(&[self.data[0], self.data[1], self.data[2]], 0);
                SettlementInfo::append_settlement(withdrawinfo);
                player.store();
                Ok(()) 
            }
        }
    }
}

#[derive (Clone)]
pub struct Deposit {
    pub data: [u64; 3],
}

impl CommandHandler for Deposit {
    fn handle(&self, pid: &[u64; 2], nonce: u64, _rand: &[u64; 4]) -> Result<(), u32> {
        let mut admin = HITPlayer::get_from_pid(pid).unwrap();
        admin.check_and_inc_nonce(nonce);
        let mut player = HITPlayer::get_from_pid(&[self.data[0], self.data[1]]);
        match player.as_mut() {
            None => Err(ERROR_PLAYER_NOT_EXIST),
            Some(player) => {
                player.data.balance += self.data[2];
                player.store();
                admin.store();
                Ok(())
            }
        }
    }
}
