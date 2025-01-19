import { ZKWasmAppRpc} from "zkwasm-minirollup-rpc";
import { Player } from "./api.js";
let account = "1234";
const rpc = new ZKWasmAppRpc("http://127.0.0.1:3000");
let player = new Player(account, rpc);
async function main() {
  //let towerId = 10038n + y;
  let state = await player.getState();
  console.log(state);
  state = await player.installPlayer();
  console.log(state);
}

main();

