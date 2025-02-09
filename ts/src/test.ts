import { createCommand, LeHexBN, query, ZKWasmAppRpc} from "zkwasm-minirollup-rpc";
import { Player } from "./api.js";
import {get_server_admin_key} from "zkwasm-ts-server/src/config.js";
let account = "1234";
const rpc = new ZKWasmAppRpc("http://127.0.0.1:3000");

let admin = new Player(get_server_admin_key(), rpc);
let player = new Player(account, rpc);

async function act() {
  let state = await player.getState();
  console.log(state);
  if (state.player.balance < 100) {
    process.exit()
  }

  console.log("check");
  console.log(state.state.currentRound);
  if (state.state.prepare > 0) {
    if (state.state.currentRound < state.player.data.lastBetRound) {
      console.log("wait for start ...");
    } else {
      console.log("place a bet ...");
      state = await player.bet(100n);
    }
  } else {
    if (state.state.currentRound == state.player.data.lastBetRound) {
      if (state.state.ratio > 102) {
        console.log("collect reward ...");
        state = await player.checkout();
      } else {
        console.log("wait for more profit ...");
      }
    } else {
        console.log("being an audience ...");
    }
  }
}

async function waitTwoSeconds(): Promise<void> {
      await new Promise(resolve => setTimeout(resolve, 2000));
      console.log("Waited for 2 seconds");
}

async function main() {
  //let towerId = 10038n + y;
  let state = await player.getState();
  let adminstate = await admin.getState();
  console.log(state);
  if (state.player == null) {
    state = await player.installPlayer();
  }

  if (adminstate.player == null) {
    state = await admin.installPlayer();
  }

  const pubkey = new LeHexBN(query(account).pkx).toU64Array();
  console.log(pubkey);

  await admin.deposit(pubkey[1], pubkey[2], 0n, 1000n);

  console.log(state);
  while(true) {
    await waitTwoSeconds();
    await act();
  }
}


/*
player: { nonce: 0, data: { balance: 1000, lastBet: 0, lastBetRound: 0 } },
state: { counter: 2, currentRound: 0, prepare: 8, ratio: 100, players: [] }
*/


main();

