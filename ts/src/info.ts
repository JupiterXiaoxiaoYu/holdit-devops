import mongoose from 'mongoose';

export class Position {
    pid_1: bigint;
    pid_2: bigint;
    object_index: bigint;
    data: bigint[];
    constructor(pid_1: bigint, pid_2: bigint, object_index: bigint, data: bigint[]) {
        this.pid_1 = pid_1;
        this.pid_2 = pid_2;
        this.object_index = object_index;
        this.data = data;
    }

    static fromMongooseDoc(doc: mongoose.Document): Position {
        const obj = doc.toObject({
            transform: (doc, ret) => {
                delete ret._id;
                return ret;
            }
        });
        return new Position(obj.pid_1, obj.pid_2, obj.object_index, obj.data);
    }

    toMongooseDoc(): mongoose.Document {
        return new PositionModel({
            pid_1: this.pid_1,
            pid_2: this.pid_2,
            object_index: this.object_index,
            data: this.data,
        });
    }

    toObject(): { pid_1: bigint, pid_2: bigint, object_index: bigint, data: bigint[] } {
        return {
            pid_1: this.pid_1,
            pid_2: this.pid_2,
            object_index: this.object_index,
            data: this.data,
        };
    }

    static fromObject(obj: { pid_1: bigint, pid_2: bigint, object_index: bigint, data: bigint[]}): Position {
        return new Position(obj.pid_1, obj.pid_2, obj.object_index, obj.data);
    }

    toJSON() {
        return {
            pid_1: this.pid_1.toString(),
            pid_2: this.pid_2.toString(),
            object_index: this.object_index.toString(),
            data: this.data.toString()
        };
    }

    static fromJSON(obj: { pid_1: string, pid_2: string, object_index: string, data: string[]}): Position {
        return new Position(
            BigInt(obj.pid_1),
            BigInt(obj.pid_2),
            BigInt(obj.object_index),
            obj.data.map((x) => BigInt(x)),
        );
    }
    static fromEvent(data: BigUint64Array): Position {
        let bigintarray:bigint[] = Array.from(data);
        return new Position(data[0], data[1], data[2], bigintarray.slice(3));
    }

}

// 创建 Schema
const PositionSchema = new mongoose.Schema({
    pid_1: {
        type: BigInt,
        required: true
    },
    pid_2: {
        type: BigInt,
        required: true
    },
    object_index: {
        type: BigInt,
        required: true
    },
    data : {
        type: [BigInt],
        required: true
    },
});

// add composition index
PositionSchema.index(
    { pid_1: 1, pid_2: 1, object_index: 1 },
    { unique: true }
);

export const PositionModel = mongoose.model('Position', PositionSchema);


export class Event {
    id: bigint;
    data:BigUint64Array;
    constructor(id: bigint, data: BigUint64Array) {
        this.id = id;
        this.data = data;
    }

    static fromMongooseDoc(doc: mongoose.Document): Event {
        const obj = doc.toObject({
            transform: (doc, ret) => {
                delete ret._id;
                return ret;
            }
        });
        // Convert the Binary data back to a Buffer
        const buffer = obj.buffer;

        // Create a new BigUint64Array from the buffer
        const retrieved = new BigUint64Array(buffer.buffer, buffer.byteOffset, buffer.byteLength / 8);

        return new Event(BigInt(obj.id), retrieved);
    }

    toMongooseDoc(): mongoose.Document {
        return new EventModel({
            id: this.id.toString(),
            data: Buffer.from(this.data.buffer)
        });
    }

    toObject(): { id: bigint, data: BigUint64Array } {
        return {
            id: this.id,
            data: this.data
        };
    }

    fromObject(obj: { id: bigint, data: BigUint64Array }): Event {
        return new Event(obj.id, obj.data);
    }

    toJSON():{ id: string, data: string[] }{
        let data = Array.from(this.data).map((x) => x.toString());
        return {
            id: this.id.toString(),
            data: data,
        }
    }

    static fromJSON(obj: { id: string, data: string[] }): Event {
        let data = new BigUint64Array(obj.data.map((x) => BigInt(x)));
        return new Event(BigInt(obj.id), data);
    }
}

// Define the schema
const eventSchema = new mongoose.Schema({
    id: {
        type: String, // We'll convert bigint to string
        required: true,
        unique: true
    },
    data: {
        type: Buffer, // Use Buffer to store the binary data
        required: true
    }
});

// Create the model
export const EventModel = mongoose.model('Event', eventSchema);
