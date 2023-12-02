import '@/styles/globals.css'
import type { AppProps } from 'next/app'
import 'bootstrap/dist/css/bootstrap.css'
import "bootstrap-icons/font/bootstrap-icons.css";
import SolWalletAdapter from '@/components/SolWalletAdapter'


export default function App({ Component, pageProps }: AppProps) {

  
  return <SolWalletAdapter><Component {...pageProps} /></SolWalletAdapter>
}
