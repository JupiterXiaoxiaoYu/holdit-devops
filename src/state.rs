use crate::player::HITPlayer;
use crate::config::ADMIN_PUBKEY;
use zkwasm_rust_sdk::require;
use serde::Serialize;
use crate::settlement::SettlementInfo;
use crate::player::PlayerData;
use core::slice::IterMut;
use std::cell::{Ref, RefCell, RefMut};
use crate::command;
use crate::command::Command;
use crate::command::Deposit;
use crate::command::Withdraw;
use crate::command::CommandHandler;
use crate::command::Activity;
use zkwasm_rest_convention::CommonState;
use zkwasm_rest_abi::StorageData;
use crate::error::*;


#[derive (Serialize)]
pub struct State {
    counter: u64,
    current_round: u64, // current game index
    prepare: u64,  // counting down of the next game
    ratio: u64, // current earning ratio, equals zero if the next round is under preparation
}

pub struct SafeState(pub RefCell<State>);
unsafe impl Sync for SafeState {}

lazy_static::lazy_static! {
    pub static ref GLOBAL_STATE: SafeState = SafeState(RefCell::new(State::new()));
}

impl CommonState for State {
    type PlayerData = PlayerData;

    fn get_global<'a>() -> Ref<'a, State> {
        GLOBAL_STATE.0.borrow()
    }
    fn get_global_mut<'a>() -> RefMut<'a, State> {
        GLOBAL_STATE.0.borrow_mut()
    }
}

impl StorageData for State {
    fn from_data(u64data: &mut IterMut<u64>) -> Self {
        State {
            counter: *u64data.next().unwrap(),
            current_round: *u64data.next().unwrap(),
            prepare: *u64data.next().unwrap(),
            ratio: *u64data.next().unwrap()
        }
    }
    fn to_data(&self, data: &mut Vec<u64>) {
        data.push(self.counter);
        data.push(self.current_round);
        data.push(self.prepare);
        data.push(self.ratio);
    }
}

impl State {
    pub fn rand_seed() -> u64 {
        0
    }

    pub fn new() -> Self {
        State {
            counter: 0,
            current_round: 0,
            prepare: 10,
            ratio: 0,
        }
    }

    pub fn preempt() -> bool {
        return Self::get_global().counter % 100 == 0
    }

    pub fn store() {
        unsafe { Self::get_global().store() };
    }

    pub fn flush_settlement() -> Vec<u8> {
        let data = SettlementInfo::flush_settlement();
        data
    }

    /// get the next round if the state is preparing the next round
    pub fn get_next_active_round(&self) -> Result<u64, u32> {
        if self.ratio == 0 {
            Ok(self.current_round + 1)
        } else {
            Err(ERROR_CURRENT_ROUND_STARTED)
        }
    }

    /// get the current round info
    pub fn get_active_round_info(&self) -> Result<(u64, u64), u32> {
        if self.ratio != 0 {
            Ok((self.current_round, self.ratio))
        } else {
            Err(ERROR_CURRENT_ROUND_IN_PREPARATION)
        }
    }


    pub fn tick(&mut self) {
        self.counter += 1;
    }

    pub fn proceed(&mut self, rand: u64) {
        if self.ratio == 0 {
            if self.prepare == 0 {
                self.ratio = 100;
            } else {
                self.prepare -= 1;
            }
        } else {
            if rand & 0xf == 0 {
                self.prepare = 10;
                self.current_round += 1;
                self.ratio = 0;
            } else {
                self.ratio = (self.ratio * 101) / 100
            }
        }
    }
}

pub struct Transaction {
    pub command: Command,
    pub nonce: u64,
}

impl Transaction {
    pub fn decode_error(e: u32) -> &'static str {
        crate::error::decode_error(e)
    }
    pub fn decode(params: &[u64]) -> Self {
        let cmd = params[0] & 0xff;
        let nonce = params[0] >> 16;
        let command = if cmd == command::WITHDRAW {
            unsafe { require (params[1] == 0) }; // only token index 0 is supported
            Command::Withdraw (Withdraw {
                data: [params[2], params[3], params[4]]
            })
        } else if cmd == command::DEPOSIT {
            zkwasm_rust_sdk::dbg!("params: {:?}\n", params);
            unsafe { require (params[3] == 0) }; // only token index 0 is supported
            Command::Deposit (Deposit {
                data: [params[1], params[2], params[4]]
            })
        } else if cmd == command::INSTALL_PLAYER {
            Command::InstallPlayer
        } else if cmd == command::CHECKOUT {
            Command::Activity(Activity::Checkout)
        } else if cmd == command::BET_AND_HOLD {
            Command::Activity(Activity::Bet(params[1]))
        } else {
            Command::Tick
        };

        Transaction {
            command,
            nonce,
        }
    }

    pub fn create_player(&self, pkey: &[u64; 4]) -> Result<(), u32> {
        let pid = HITPlayer::pkey_to_pid(pkey);
        let player = HITPlayer::get_from_pid(&pid);
        match player {
            Some(_) => Err(ERROR_PLAYER_ALREADY_EXIST),
            None => {
                let player = HITPlayer::new_from_pid(pid);
                player.store();
                Ok(()) 
            }
        }
    }

    pub fn process(&self, pkey: &[u64; 4], rand: &[u64; 4]) -> Vec<u64> {
        let pid = HITPlayer ::pkey_to_pid(&pkey);
        let error_code = match &self.command {
            Command::Tick=> {
                State::get_global_mut().tick();
                State::get_global_mut().proceed(rand[2]);
                0
            },
            Command::InstallPlayer => self.create_player(pkey)
                .map_or_else(|e| e, |_| 0),
            Command::Withdraw(cmd) => cmd.handle(&pid, self.nonce, rand)
                .map_or_else(|e| e, |_| 0),
            Command::Activity(cmd) => cmd.handle(&pid, self.nonce, rand)
                .map_or_else(|e| e, |_| 0),
            Command::Deposit(cmd) => {
                unsafe { require(*pkey == *ADMIN_PUBKEY) };
                cmd.handle(&pid, self.nonce, rand)
                    .map_or_else(|e| e, |_| 0)
            },
            _ => {
                unreachable!();
            }
        };
        vec![error_code as u64]
    }
}
