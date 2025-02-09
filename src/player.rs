use crate::StorageData;
use zkwasm_rest_abi::Player;
use core::slice::IterMut;
use serde::Serialize;
use crate::error::*;

#[derive(Debug, Serialize, Default)]
pub struct PlayerData {
    pub balance: u64,
    pub lastBet: u64,
    pub lastBetRound: u64,
}

impl StorageData for PlayerData {
    fn from_data(u64data: &mut IterMut<u64>) -> Self {
        let balance = *u64data.next().unwrap();
        let lastBet = *u64data.next().unwrap();
        let lastBetRound = *u64data.next().unwrap();
        PlayerData {
            balance,
            lastBet,
            lastBetRound,
        }
    }
    fn to_data(&self, data: &mut Vec<u64>) {
        data.push(self.balance);
        data.push(self.lastBet);
        data.push(self.lastBetRound);
    }
}

impl PlayerData {
    pub fn is_in_current_round(&self, round: u64) -> Result<(), u32> {
        if self.lastBetRound == round {
            Ok(())
        } else {
            Err(ERROR_PLAYER_NOT_IN_CURRENT_ROUND)
        }
    }
    pub fn checkout(&mut self, current_round: u64, ratio: u64) -> Result <u64, u32> {
        if self.lastBetRound == current_round {
            let inc = (self.lastBet * ratio) / 100;
            self.balance += inc;
            self.lastBet = 0;
            self.lastBetRound = 0;
            Ok(inc)
        } else {
            Err(ERROR_PLAYER_NOT_IN_CURRENT_ROUND)
        }
    }
    pub fn place(&mut self, amount: u64, round: u64) -> Result<(), u32> {
        zkwasm_rust_sdk::dbg!("amount is {}\n", amount);
        if amount > self.balance {
            Err(ERROR_PLAYER_NOT_ENOUGH_BALANCE)
        } else {
            self.lastBet = amount;
            self.balance -= amount;
            self.lastBetRound = round;
            Ok(())
        }
    }
}

pub type HITPlayer = Player<PlayerData>;
