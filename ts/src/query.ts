import { createCommand, LeHexBN, query, ZKWasmAppRpc} from "zkwasm-minirollup-rpc";
import { Player } from "./api.js";
import {get_server_admin_key} from "zkwasm-ts-server/src/config.js";
let account = "1234";
const rpc = new ZKWasmAppRpc("http://127.0.0.1:3000");

let admin = new Player(get_server_admin_key(), rpc);
let player = new Player(account, rpc);

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

  let rslt = await player.rpc.queryData(`history/${pubkey[1]}/${pubkey[2]}`)
  console.log(rslt);

}


/*
player: { nonce: 0, data: { balance: 1000, lastBet: 0, lastBetRound: 0 } },
state: { counter: 2, currentRound: 0, prepare: 8, ratio: 100, players: [] }
*/


main();

