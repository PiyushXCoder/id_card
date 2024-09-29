import { Connection, PublicKey } from "@solana/web3.js";
import { getKeypairFromFile } from "@solana-developers/helpers";
import * as borsh from "borsh";
import { cardSchema } from "./schemas.js";
import * as process from "process";

const programId = new PublicKey(process.env.PROG);

// Connect to a solana cluster. Either to your local test validator or to devnet
const connection = new Connection("http://localhost:8899", "confirmed");
//const connection = new Connection("https://api.devnet.solana.com", "confirmed");

// We load the keypair that we created in a previous step
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
