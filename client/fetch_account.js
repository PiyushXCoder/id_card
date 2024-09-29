import { Connection, PublicKey } from "@solana/web3.js";
import { getKeypairFromFile } from "@solana-developers/helpers";
import * as borsh from "borsh";
import { cardSchema } from "./schemas.js";
import * as process from "process";

const programId = new PublicKey(process.env.PROG);

const connection = new Connection("http://localhost:8899", "confirmed");

const keyPair = await getKeypairFromFile("~/.config/solana/id.json");

const [pda, _] = PublicKey.findProgramAddressSync(
  [keyPair.publicKey.toBuffer()],
  programId,
);

const accountInfo = await connection.getAccountInfo(pda);
console.log(accountInfo);

const card = borsh.deserialize(cardSchema, accountInfo.data);
console.log("Id Card:");
console.log(card);
