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

const connection = new Connection("http://localhost:8899", "confirmed");

const keyPair = await getKeypairFromFile("~/.config/solana/id.json");

const blockhashInfo = await connection.getLatestBlockhash();

const tx = new Transaction({
  ...blockhashInfo,
});

const [pda, _] = PublicKey.findProgramAddressSync(
  [keyPair.publicKey.toBuffer()],
  programId,
);

const data = { Delete: {} };

const encoded = borsh.serialize(cardDataSchema, data);

console.log("Data:", encoded);
console.log("Keys:", keyPair.publicKey.toBase58());

tx.add(
  new TransactionInstruction({
    programId: programId,
    keys: [
      {
        pubkey: keyPair.publicKey,
        isSigner: true,
        isWritable: true,
      },
      {
        pubkey: pda,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    data: Buffer.from(encoded),
  }),
);

tx.sign(keyPair);

const txHash = await connection.sendRawTransaction(tx.serialize());

console.log("Transaction sent with hash:", txHash);

await connection.confirmTransaction({
  blockhash: blockhashInfo.blockhash,
  lastValidBlockHeight: blockhashInfo.lastValidBlockHeight,
  signature: txHash,
});

console.log(`https://explorer.solana.com/tx/${txHash}?cluster=custom`);
