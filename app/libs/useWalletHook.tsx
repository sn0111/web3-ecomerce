import { Program, AnchorProvider, web3, BN } from "@project-serum/anchor";
import { AnchorProjects } from "../../target/types/anchor_projects";
import {
  connection,
  commitmentLevel,
  helloWorldprogramId,
  helloWorldprogramInterface,
} from "./constants";
import { AnchorWallet } from "@solana/wallet-adapter-react";
import { Keypair, PublicKey } from "@solana/web3.js";

export async function getAllLaptops(
  wallet: AnchorWallet
) {
  const provider = new AnchorProvider(connection, wallet, {
    preflightCommitment: commitmentLevel
  });

  if (!provider) return;

  /* create the program interface combining the idl, program Id, and provider */
  const program = new Program(
    helloWorldprogramInterface,
    helloWorldprogramId,
    provider
  ) as Program<AnchorProjects>;

  return await program.account.product.all([]);
}

export async function buySolLaptop(wallet:AnchorWallet,amount:any){
  const provider = new AnchorProvider(connection, wallet, {
    preflightCommitment: commitmentLevel
  });

  if (!provider) return;

  const program = new Program(
    helloWorldprogramInterface,
    helloWorldprogramId,
    provider
  ) as Program<AnchorProjects>;
  // await program.methods.transfer(new anchor.BN(anchor.web3.LAMPORTS_PER_SOL))
  // .accounts({from:provider.wallet.publicKey,to:new anchor.web3.PublicKey("7URt4Yt6zHHJ2Ffme4NeFpPpoFmMBhf37nVc4VgntjXb")})
  // .rpc();
  return await program.methods.transfer(new BN(amount)).
    accounts({from:provider.wallet.publicKey,to: new PublicKey("J3ieXe7NVgemdmaFfFS9Qx5mQVVoU4pHg7qGuzrPZ6ak")}).rpc()
}
