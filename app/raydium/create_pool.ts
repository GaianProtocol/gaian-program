import {
  ApiV3Token,
  CLMM_PROGRAM_ID,
  DEVNET_PROGRAM_ID,
  Token,
} from "@raydium-io/raydium-sdk-v2";
import { initSdk, txVersion, connection } from "./config";
import Decimal from "decimal.js";
import BN from "bn.js";
import { PublicKey } from "@solana/web3.js";
import { devConfigs } from "./utils";

export const createPool = async () => {
  const raydium = await initSdk({ loadToken: true });
  console.log("create clmm pool");

  // you can call sdk api to get mint info or paste mint info from api: https://api-v3.raydium.io/mint/list
  // RAY
  // const mint1 = await raydium.token.getTokenInfo(
  //   "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R"
  //   // "So11111111111111111111111111111111111111112"
  // );

  const mint1: ApiV3Token = {
    chainId: 101,
    address: "So11111111111111111111111111111111111111112",
    programId: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    logoURI: "",
    symbol: "WSOL",
    name: "Wrapped SOL",
    decimals: 9,
    tags: [],
    extensions: {},
  };
  console.log("mint1", mint1);

  // USDT
  // const mint2 = await raydium.token.getTokenInfo(
  //   "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"
  // );
  const mint2: ApiV3Token = {
    chainId: 101,
    address: "5MSmNyrsBvDXyZkgXS4wV3wbDR3dNQb8nsShjC7dMyi1",
    programId: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    logoURI: "",
    symbol: "YT-311224",
    name: "YT 311224",
    decimals: 9,
    tags: [],
    extensions: {},
  };
  // const clmmConfigs = await raydium.api.getClmmConfigs();
  const clmmConfigs = devConfigs; // devnet configs

  console.log(DEVNET_PROGRAM_ID.CLMM.toBase58(), clmmConfigs);
  const { execute } = await raydium.clmm.createPool({
    // programId: CLMM_PROGRAM_ID,
    programId: DEVNET_PROGRAM_ID.CLMM,
    mint1,
    mint2,
    ammConfig: {
      ...clmmConfigs[0],
      id: new PublicKey(clmmConfigs[0].id),
      fundOwner: "",
      description: "",
    },
    initialPrice: new Decimal(1),
    startTime: new BN(0),
    txVersion,
    // optional: set up priority fee here
    // computeBudgetConfig: {
    //   units: 600000,
    //   microLamports: 100000000,
    // },
  });
  // don't want to wait confirm, set sendAndConfirm to false or don't pass any params to execute
  try {
    const { txId } = await execute({ sendAndConfirm: true });
    console.log("clmm pool created:", {
      txId: `https://explorer.solana.com/tx/${txId}`,
    });
  } catch (e) {
    console.log(e);
  }
};

/** uncomment code below to execute */
createPool();

// pool: https://explorer.solana.com/address/DhFs8xnfvE66b5XEHBrRCuwFxRVRux9yf1ufyCXqPy2G/tokens?cluster=devnet
