import { PlayerConvention, ZKWasmAppRpc } from "zkwasm-minirollup-rpc";
import BN from 'bn.js';

function bytesToHex(bytes: Array<number>): string  {
    return Array.from(bytes, byte => byte.toString(16).padStart(2, '0')).join('');
}

const TICK = 0n;
const CMD_INSTALL_PLAYER = 1n;
const CMD_BET_AND_HOLD = 2n;
const CMD_CHECKOUT = 3n;
const CMD_WITHDRAW = 5n;
const CMD_DEPOSIT = 6n;

export class Player extends PlayerConvention {
  constructor(key: string, rpc: ZKWasmAppRpc) {
    super(key, rpc, CMD_DEPOSIT, CMD_WITHDRAW);
    this.processingKey = key,
    this.rpc = rpc;
  }

  async sendTransactionWithCommand(cmd: BigUint64Array) {
    try {
      let result = await this.rpc.sendTransaction(
        cmd,
        this.processingKey
      );
      return result;
    } catch(e) {
      if(e instanceof Error) {
        console.log(e.message);
      }
      throw e;
    }
  }

  async installPlayer() {
    let cmd = this.createCommand(0n, CMD_INSTALL_PLAYER, []);
    await this.sendTransactionWithCommand(cmd);
  }

  async bet(n: bigint) {
    let cmd = this.createCommand(0n, CMD_BET_AND_HOLD, [n]);
    await this.sendTransactionWithCommand(cmd);
  }

  async checkout() {
    let cmd = this.createCommand(0n, CMD_CHECKOUT, []);
    await this.sendTransactionWithCommand(cmd);
  }
}

