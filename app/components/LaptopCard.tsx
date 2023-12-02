import { buySolLaptop } from "@/libs/useWalletHook";
import { AnchorWallet, useWallet } from "@solana/wallet-adapter-react";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import Link from "next/link";
import { useEffect, useState } from "react";

const LaptopCard = (props:any) =>{
    const [laptop,setLaptop] = useState(props.laptop.account)

    const wallet = useWallet()
    useEffect(()=>{
      
      console.log(props.laptop.publicKey.toBase58())
    })

    const buyLaptop = () =>{
      console.log(LAMPORTS_PER_SOL/laptop.mrpAmount.toNumber())
      buySolLaptop(wallet as AnchorWallet,LAMPORTS_PER_SOL/laptop.mrpAmount)
      .then(res=>console.log(res))
      .catch(err=>err)
    }

    return <div className="card mb-3" style={{"cursor":"pointer",maxWidth:"540px"}}>
      <div className="row g-0">
        <div className="col-md-4">
          <img src={props.laptop.account.imageUrl} className="card-img-top" alt="../public/laptop.jpeg"/>
        </div>
        <div className="col-md-8">
          <div className="card-body">
            <h5 className="card-title">{laptop.name}</h5>
            <div className="card-text">
              <div>
                <b>{laptop.mrpAmount.toNumber() - laptop.mrpAmount.toNumber()/laptop.discount}</b> Sol ({laptop.mrpAmount.toNumber()} sol)
              </div>
              <div className="h-20">
                {laptop.desc}
                {/* {"Intel Core i3-1115G4, 8GB/1TB + 256GB SSD/15.6 (39.62cm) FHD with Comfort View Mobile Connect/Windows 11 + MSO'21/15 Month McAfee/Spill-Resistant Keyboard/Carbon/ 1.69kg"} */}
              </div>
            </div>
            <div className="row">
              <div className="col-md-6"></div>
              <div className="col-md-3">
                  {/* <Link href="/buy" className="btn btn-danger">Buy</Link> */}
                  <a href="#" className="btn btn-danger" onClick={buyLaptop}>Buy</a>
              </div>
              <div className="col-md-3">
                  <button className="btn btn-primary" disabled>Cart</button>
              </div>
            </div>
          </div>
        </div>
    </div>
  </div>
}

export default LaptopCard;