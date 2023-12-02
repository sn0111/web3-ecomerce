import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorProjects } from "../target/types/anchor_projects";

describe("anchor-projects", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

    const wallet = provider.wallet as anchor.Wallet;


  const program = anchor.workspace.AnchorProjects as Program<AnchorProjects>;
  const productKeyPair = anchor.web3.Keypair.generate()

  // it("Create Product", async () => {
  //   const product = productKeyPair.publicKey
  //   const tx = await program.methods.createProduct({
  //     author:provider.publicKey,
  //     name:"Dell 15 Laptop",
  //     quantity:new anchor.BN(100),
  //     desc:"Intel Core i3-1115G4, 8GB/1TB + 256GB SSD/15.6 (39.62cm) FHD with Comfort View Mobile Connect/Windows 11 + MSO'21/15 Month McAfee/Spill-Resistant Keyboard/Carbon/ 1.69kg ",
  //     imageUrl:"https://m.media-amazon.com/images/I/51r-PhkZodL._SL1080_.jpg",
  //     cost: new anchor.BN(2227.07),
  //     mrpAmount:new anchor.BN(2287.73),
  //     discount: 10
  //   })
  //   .accounts({payer:provider.publicKey,product:product})
  //   .signers([productKeyPair])
  //   .rpc();
  //   console.log("Your transaction signature", tx);

  //   const p = await program.account.product.all();
  //   p[0].publicKey
  //   console.log(p)
  // });

  // it("Delete Product", async ()=>{

  //   const tx =await program.methods.deleteProduct()
  //             .accounts({product:productKeyPair.publicKey,payer:provider.publicKey})
  //             .rpc();
  //   console.log(tx)

  //   const p = await program.account.product.fetch(productKeyPair.publicKey);
  //   console.log(p)
  // })


    it("Delete Product", async ()=>{

    // const tx =await program.methods.transfer(new anchor.BN(anchor.web3.LAMPORTS_PER_SOL))
    //           .accounts({from:provider.wallet.publicKey,to:new anchor.web3.PublicKey("7URt4Yt6zHHJ2Ffme4NeFpPpoFmMBhf37nVc4VgntjXb")})
    //           .rpc();
    // console.log(tx)

    const p = await program.account.product.all();
    console.log(p)
  })


  

});
