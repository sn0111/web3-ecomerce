import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import react from 'react';
const Navbar = () =>{
    
    return <nav className="navbar bg-dark navbar-dark">
    <div className="container-fluid">
      <a className="navbar-brand" href="#">Laptop Stores</a>
      <form className="d-flex cart-icon" role="search">
        {/* <i className="bi bi-cart mr-4" style={{fontSize: "2rem", color: "white"}}></i>
        <span>3</span> */}
        <WalletMultiButton/>
      </form>
    </div>
  </nav>
}

export default Navbar;