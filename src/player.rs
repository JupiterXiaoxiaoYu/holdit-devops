use crate::StorageData;
use zkwasm_rest_abi::Player;
use core::slice::IterMut;
use serde::Serialize;
use crate::error::*;

#[derive(Debug, Serialize, Default)]
pub struct PlayerData {
    pub balance: u64,
    pub last_bet: u64,
    pub last_bet_round: u64,
}

impl StorageData for PlayerData {
    fn from_data(u64data: &mut IterMut<u64>) -> Self {
        let balance = *u64data.next().unwrap();
        let last_bet = *u64data.next().unwrap();
        let last_bet_round = *u64data.next().unwrap();
        PlayerData {
            balance,
            last_bet,
            last_bet_round,
        }
    }
    fn to_data(&self, data: &mut Vec<u64>) {
        data.push(self.balance);
        data.push(self.last_bet);
        data.push(self.last_bet_round);
    }
}

impl PlayerData {
    pub fn is_in_current_round(&self, round: u64) -> Result<(), u32> {
        if self.last_bet_round == round {
            Ok(())
        } else {
            Err(ERROR_PLAYER_NOT_IN_CURRENT_ROUND)
        }
    }
    pub fn checkout(&mut self, current_round: u64, ratio: u64) -> Result <(), u32> {
        if self.last_bet_round == current_round {
            self.balance += (self.last_bet * ratio) / 100;
            self.last_bet= 0;
            self.last_bet_round = 0;
            Ok(())
        } else {
            Err(ERROR_PLAYER_NOT_IN_CURRENT_ROUND)
        }
    }
    pub fn place(&mut self, amount: u64, round: u64) -> Result<(), u32> {
        if amount > self.balance {
            Err(ERROR_PLAYER_NOT_ENOUGH_BALANCE)
        } else {
            self.last_bet = amount;
            self.balance -= amount;
            self.last_bet_round = round;
            Ok(())
        }
    }
}

pub type HITPlayer = Player<PlayerData>;
