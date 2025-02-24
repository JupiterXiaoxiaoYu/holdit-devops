import { Service } from "zkwasm-ts-server";
import { Express } from "express";
import { TxWitness} from "zkwasm-ts-server/src/prover";
import { Position, PositionModel } from "./info.js";
import { Event, EventModel } from "zkwasm-ts-server";

const comments = new Array();

function addComment(comment: {msg: string, pubkey: string}) {
  comments.push(comment);
}

function extra (app: Express) {
  app.post('/extrinsic', async(req:any, res) => {
    const value = req.body.msg;
    const pubkey = req.body.pkx;
    addComment({msg: value, pubkey: pubkey});
    res.status(201).send({
      success: true,
      data: comments,
    });
  });
  app.get('/data/comments', async(req:any, res) => {
    res.status(201).send({
      success: true,
      data: comments,
    });
  });
  app.get('/data/history/:pid1/:pid2', async(req:any, res) => {
    let pid1:bigint = BigInt(req.params.pid1);
    let pid2:bigint = BigInt(req.params.pid2);
    let doc = await PositionModel.find(
        {pid_1: pid1, pid_2: pid2},
    );
    let data = doc.map((d) => {return Position.fromMongooseDoc(d).toJSON()})
    res.status(201).send({
      success: true,
      data: data,
    });
  });

}

async function bootstrap(merkleRoot: string): Promise<TxWitness[]> {
  /*
  const txs = await getTxFromCommit(merkleRoot);
  console.log("tsx in bootstrap:", txs);
  return txs;
  */
  return [];
}

async function batchedCallback(arg: TxWitness[], preMerkle: string, postMerkle: string) {
  /*
  currentUncommitMerkleRoot = postMerkle;
  await clearTxFromCommit(currentUncommitMerkleRoot);
  preemptcounter = 0;
  */
  return;
}

async function eventCallback(arg: TxWitness, data: BigUint64Array) {
  //insertTxIntoCommit(currentUncommitMerkleRoot, arg, preemptcounter);
  //preemptcounter ++;
  //
  //
  const EVENT_BET = 1;
  const EVENT_CHECKOUT = 2;
  if(data.length == 0) {
    return;
  }

  //console.log("eventCallback", arg, data);
  if(data[0] != 0n) {
    console.log("non-zero return, tx failed", data[0]);
    return;
  }
  if(data.length <= 2) {
    console.log("no event data");
    return;
  }

  let event = new Event(data[1], data);
  let doc = new EventModel({
    id: event.id.toString(),
    data: Buffer.from(event.data.buffer)
  });

  try {
    let result = await doc.save();
    if (!result) {
      console.log("failed to save event");
      throw new Error("save event to db failed");
    }
  } catch(e) {
    console.log(e);
    console.log("event ignored");
  }
  let i = 2; // start pos
  while(i < data.length) {
    let eventType = Number(data[i]>>32n);
    let eventLength = data[i]&((1n<<32n)-1n);
    let eventData = data.slice(i+1, i+1+Number(eventLength));
    console.log("event", eventType, eventLength, eventData);
    switch(eventType) {
      case EVENT_BET:
        {
          console.log("bet event");
          let position = Position.fromEvent(eventData);
          let doc = await PositionModel.findOneAndUpdate(
              {pid_1: position.pid_1, pid_2: position.pid_2, object_index: position.object_index},
              position.toObject(),
              {upsert: true}
          );
          console.log("save position", position.pid_1, position.pid_2, position.object_index);
        }
        break;
      case EVENT_CHECKOUT:
        {
          console.log("checkout event");
          let position = Position.fromEvent(eventData);
          let doc = await PositionModel.findOneAndUpdate(
              {pid_1: position.pid_1, pid_2: position.pid_2, object_index: position.object_index},
              position.toObject(),
              {upsert: true}
          );
          console.log("save checkout", position.pid_1, position.pid_2, position.object_index);
        }
        break;
      default:
        console.log("unknown event");
        break;
    }
    i += 1 + Number(eventLength);
  }
}


const service = new Service(eventCallback, batchedCallback, extra, bootstrap);
service.initialize();
service.serve();


