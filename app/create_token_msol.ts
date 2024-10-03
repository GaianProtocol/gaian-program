import {
  PublicKey,
  ComputeBudgetProgram,
  Transaction,
  Connection,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
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

  const suffix = tokenAddresses[network].msolSuffix;
  const { pt } = getPTTokenPda(program, suffix);
  console.log("ptMint:", pt.toBase58());

  const { yt } = getYTTokenPda(program, suffix);
  console.log("ytMint:", yt.toBase58());

  const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const [ptMetadataAddress] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      pt.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );

  const [ytMetadataAddress] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      yt.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );

  const ix = await program.methods
    .createToken(suffix)
    .accounts({
      ptMetadataAccount: ptMetadataAddress,
      ytMetadataAccount: ytMetadataAddress,
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
}

async function main() {
  await initialize();
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
