import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Stablecoin } from "../target/types/stablecoin";
import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";

describe("stablecoin", () => {
   
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.stablecoin as Program<Stablecoin>;
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = provider.wallet as anchor.Wallet;
  
  const SOL_PRICE_FEED_ID = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
  const pythSolanaReceiver = new PythSolanaReceiver({ connection,wallet});
  const sol_usd_price_feed_account = pythSolanaReceiver.getPriceFeedAccountAddress(0,SOL_PRICE_FEED_ID).toBase58(); //e What is a shardId
  anchor.setProvider(provider);

  //e Deriving the 
  const [collateralAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("collateral"), wallet.publicKey.toBuffer()],
    program.programId
  );

  it("Is Initialized", async() => {
    const tx = await program.methods.initializeConfig().accounts({}).rpc({skipPreflight: true,commitment: "confirmed"});
    console.log("Initialized: ",tx);
  })

  it("Deposit Collateral and Mint USDC",async() => {
    const amountCollateral = 1_000_000_000;
    const amountToMint = 1_000_000;

    const tx = await program.methods.depositCollateralAndMint(
      new anchor.BN(amountCollateral),
      new anchor.BN(amountToMint),
    ).accounts({priceUpdate: sol_usd_price_feed_account}).rpc({skipPreflight: true, commitment: "confirmed"});

    console.log("Deposit transaction signature: ", tx);
  })

  it("Liquidate",async() => {
    const tx = await program.methods.Liquidate(new anchor.BN(1)).accounts({}).rpc({skippreflight: true,commitment: "confirmed"});
  })

 });
