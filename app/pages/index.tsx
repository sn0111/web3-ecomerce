import LaptopCard from '@/components/LaptopCard'
import {getAllLaptops} from '@/libs/useWalletHook'
import { AnchorWallet, useAnchorWallet, useWallet } from '@solana/wallet-adapter-react'
import { Inter } from 'next/font/google'
import { useEffect, useState } from 'react'

export default function Home() {
  const [laptops,setLaptop] = useState([])
  const wallet = useWallet()
  useEffect(()=>{
    laptop()
  },[])

  const laptop =async ()=>{
    getAllLaptops(wallet as AnchorWallet)
    .then((res:any)=>setLaptop(res))
    .catch(err=>err)
  }
  return <div className='container'>
    <div className='row'>
      {laptops.map((laptop,key)=><div className='col-md-6 mt-3'><LaptopCard laptop={laptop}/> </div>)}
    </div>
  </div>
}
