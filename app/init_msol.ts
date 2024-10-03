import {
  PublicKey,
  ComputeBudgetProgram,
  Transaction,
  Connection,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import * as idl from "./idl/gaian.json";
import { Gaian } from "./idl/gaian";
import {
  tokenAddresses,
  getProvider,
  loadKeypairFromFile,
  getPTTokenPda,
  getYTTokenPda,
  getGaianPda,
  getGaianTokenPda,
} from "./utils";

const network = process.env.NETWORK || "devnet";

async function initialize() {
  const owner = loadKeypairFromFile("./deployer.json");
  const provider = await getProvider(tokenAddresses[network].rpcUrl, owner);
  console.log(`owner address: ${provider.wallet.publicKey.toBase58()}`);

  const program = new Program(idl as unknown as Gaian, provider);

  const suffix = tokenAddresses[network].msolSuffix;
  const { pt } = getPTTokenPda(program, suffix);
  const { yt } = getYTTokenPda(program, suffix);
  console.log("ptMint:", pt.toBase58());
  console.log("ytMint:", yt.toBase58());

  const expiration = new BN(1735541168);

  const ix = await program.methods
    .initializeToken(suffix, expiration)
    .accounts({
      mint: tokenAddresses[network].msol,
    })
    .instruction();

  const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
    microLamports: 1,
  });
  const transaction = new Transaction().add(addPriorityFee).add(ix);

  const connection = new Connection(tokenAddresses[network].rpcUrl);
  console.log("sending tx...");

  const sig = await sendAndConfirmTransaction(connection, transaction, [owner]);
  console.log("tx:", sig);

  const { gaian } = getGaianTokenPda(program, pt, yt);
  console.log(`gaian address: ${gaian.toBase58()}`);

  const data = await program.account.gaian.fetch(gaian);
  console.log("gaian: ", data);
}

async function main() {
  await initialize();
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
