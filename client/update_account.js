import {
  Connection,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { getKeypairFromFile } from "@solana-developers/helpers";
import * as borsh from "borsh";
import { cardDataSchema } from "./schemas.js";
import * as process from "process";

const programId = new PublicKey(process.env.PROG);

// Connect to a solana cluster. Either to your local test validator or to devnet
const connection = new Connection("http://localhost:8899", "confirmed");
//const connection = new Connection("https://api.devnet.solana.com", "confirmed");

// We load the keypair that we created in a previous step
const keyPair = await getKeypairFromFile("~/.config/solana/id.json");

// Every transaction requires a blockhash
const blockhashInfo = await connection.getLatestBlockhash();

// Create a new transaction
const tx = new Transaction({
  ...blockhashInfo,
});

const [pda, _] = PublicKey.findProgramAddressSync(
  [keyPair.publicKey.toBuffer()],
  programId,
);

const data = {
  Update: {
    name: "Yay",
    bio: "Yhe bio",
  },
};
const encoded = borsh.serialize(cardDataSchema, data);

console.log("Data:", encoded);
console.log("Keys:", keyPair.publicKey.toBase58());

tx.add(
  new TransactionInstruction({
    programId: programId,
    keys: [
      {
        pubkey: pda,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: keyPair.publicKey,
        isSigner: true,
        isWritable: true,
      },
    ],
    data: Buffer.from(encoded),
  }),
);

// Sign the transaction with your previously created keypair
tx.sign(keyPair);

// Send the transaction to the Solana network
const txHash = await connection.sendRawTransaction(tx.serialize());

console.log("Transaction sent with hash:", txHash);

await connection.confirmTransaction({
  blockhash: blockhashInfo.blockhash,
  lastValidBlockHeight: blockhashInfo.lastValidBlockHeight,
  signature: txHash,
});

console.log(`https://explorer.solana.com/tx/${txHash}?cluster=custom`);
