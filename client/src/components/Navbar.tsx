import Logo from "./svg/Logo.jsx"
import { Wallet } from "lucide-react";
import { useWallet } from "@solana/wallet-adapter-react";
import { WalletButton } from "@/components/providers/wallet-provider";

interface NavbarProps {
  isConnected: boolean;
  onConnectWallet: () => void;
}

const Navbar = ({ isConnected, onConnectWallet }: NavbarProps) => {
  const { connected } = useWallet();

  return (
    <nav className="w-auto p-4 bg-card/50 backdrop-blur-xl md:m-20 md:mt-5 rounded-2xl shadow-lg">
      <div className="max-w-6xl mx-auto flex items-center justify-between">
        <h1 className="text-2xl font-bold bg-gradient-primary bg-clip-text text-transparent">
          <Logo />
        </h1>

        {connected ? (
          <WalletButton className="font-semibold px-6 py-2 shadow-lg transition-all duration-200 hover:scale-105 hover:shadow-xl" />
        ) : (
          <WalletButton className="font-semibold px-6 py-2 shadow-lg transition-all duration-200 hover:scale-105 hover:shadow-xl">
            <Wallet className="h-4 w-4 mr-2" />
            Connect Wallet
          </WalletButton>
        )}
      </div>
    </nav>
  );
};

export default Navbar;