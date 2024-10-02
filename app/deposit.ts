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
} from "./utils";

const network = process.env.NETWORK || "devnet";

async function initialize() {
  const owner = loadKeypairFromFile("./deployer.json");
  const provider = await getProvider(tokenAddresses[network].rpcUrl, owner);
  console.log(`owner address: ${provider.wallet.publicKey.toBase58()}`);

  const program = new Program(idl as unknown as Gaian, provider);

  const suffix = "311224";
  const amount = new BN(100_000); // 0.1 SOL
  const { pt, bump: ptBump } = getPTTokenPda(program, suffix);
  const { yt, bump: ytBump } = getYTTokenPda(program, suffix);
  console.log("pt:", pt.toBase58(), "bump:", ptBump);
  console.log("yt:", yt.toBase58(), "bump:", ytBump);

  const ix = await program.methods
    .deposit(suffix, amount, ptBump, ytBump)
    .accounts({})
    .instruction();

  const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
    microLamports: 1,
  });
  const transaction = new Transaction().add(addPriorityFee).add(ix);

  const connection = new Connection(tokenAddresses[network].rpcUrl);
  console.log("sending tx...");

  const sig = await sendAndConfirmTransaction(connection, transaction, [owner]);
  console.log("tx:", sig);
}

async function main() {
  await initialize();
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
