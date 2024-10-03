import fs from "fs";
import { clusterApiUrl, PublicKey, Keypair, Connection } from "@solana/web3.js";
import { Wallet, AnchorProvider, Program } from "@coral-xyz/anchor";
import { Gaian } from "./idl/gaian";

interface Config {
  rpcUrl: string;

  solSuffix: string;

  msol: PublicKey;
  msolSuffix: string;
}

export const tokenAddresses: Record<string, Config> = {
  devnet: {
    rpcUrl: clusterApiUrl("devnet"),

    solSuffix: "311224",

    msol: new PublicKey("2A8vKToJrRGQwQyCZgVmp6jDd2gqZquRPZLhvNCaU4QD"),
    msolSuffix: "msolQ4",
  },
};

export function loadKeypairFromFile(filePath: string): Keypair {
  const secretKeyString = fs.readFileSync(filePath, { encoding: "utf8" });
  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  return Keypair.fromSecretKey(secretKey);
}

export async function getProvider(
  rpcUrl: string,
  kp: Keypair
): Promise<AnchorProvider> {
  const conn = new Connection(rpcUrl, {
    commitment: "confirmed",
  });
  const wallet = new Wallet(kp);

  const provider = new AnchorProvider(
    conn,
    wallet,
    AnchorProvider.defaultOptions()
  );

  return provider;
}

export function getGaianPda(
  program: Program<Gaian>,
  ptMint: PublicKey,
  ytMint: PublicKey
): {
  gaian: PublicKey;
  bump: number;
} {
  const [gaian, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("gaian"), ptMint.toBuffer(), ytMint.toBuffer()],
    program.programId
  );

  return { gaian, bump };
}

export function getGaianTokenPda(
  program: Program<Gaian>,
  ptMint: PublicKey,
  ytMint: PublicKey
): {
  gaian: PublicKey;
  bump: number;
} {
  const [gaian, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("gaian_token"), ptMint.toBuffer(), ytMint.toBuffer()],
    program.programId
  );

  return { gaian, bump };
}

export function getPTTokenPda(
  program: Program<Gaian>,
  suffix: string
): {
  pt: PublicKey;
  bump: number;
} {
  const [pt, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("gaian_pt"), Buffer.from(suffix)],
    // [Buffer.from("gaian_pt")],
    program.programId
  );

  return { pt, bump };
}

export function getYTTokenPda(
  program: Program<Gaian>,
  suffix: string
): {
  yt: PublicKey;
  bump: number;
} {
  const [yt, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("gaian_yt"), Buffer.from(suffix)],
    // [Buffer.from("gaian_yt")],
    program.programId
  );

  return { yt, bump };
}
