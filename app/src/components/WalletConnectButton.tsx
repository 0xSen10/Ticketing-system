// src/components/WalletConnectButton.tsx
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
require("@solana/wallet-adapter-react-ui/styles.css");

export default function WalletConnectButton() {
  return (
    <div className="flex justify-end p-4">
      <WalletMultiButton />
    </div>
  );
}
