import React, { useState, useEffect } from "react";
import BrowserOnly from "@docusaurus/BrowserOnly";

const AddToWalletButton = () => {
  const [isAlreadyAdded, setIsAlreadyAdded] = useState(false);

  const resetButtonState = () => {
    setTimeout(() => {
      setIsAlreadyAdded(false);
    }, 2000);
  };

  useEffect(() => {
    const checkIfNetworkExists = async () => {
      if (!window.ethereum) return;

      try {
        const chainId = await window.ethereum.request({
          method: "eth_chainId",
        });
        if (chainId === "0xED88") {
          setIsAlreadyAdded(true);
          resetButtonState();
        }
      } catch (error) {
        console.error("Error checking network:", error);
      }
    };

    checkIfNetworkExists();
  }, []);

  const addNetwork = async () => {
    if (!window.ethereum) {
      alert(
        "Please install MetaMask or another Web3 wallet to add the network."
      );
      return;
    }

    try {
      await window.ethereum.request({
        method: "wallet_addEthereumChain",
        params: [
          {
            chainId: "0xED88",
            chainName: "BOB",
            nativeCurrency: {
              name: "ETH",
              symbol: "ETH",
              decimals: 18,
            },
            rpcUrls: ["https://rpc.gobob.xyz/"],
            blockExplorerUrls: ["https://explorer.gobob.xyz/"],
          },
        ],
      });
      setIsAlreadyAdded(true);
      resetButtonState();
    } catch (error) {
      if (
        error.message.includes("already exists") ||
        error.code === 4902 || // Standard EIP-1193 error code for already existing chain
        error.message.toLowerCase().includes("already")
      ) {
        setIsAlreadyAdded(true);
        resetButtonState();
      } else {
        console.error("Error adding network:", error);
        alert("Failed to add network to wallet: " + error.message);
      }
    }
  };

  return (
    <button
      onClick={addNetwork}
      style={{
        backgroundColor: "var(--bob-accent)",
        border: "none",
        borderRadius: "4px",
        color: "white",
        cursor: "pointer",
        padding: "8px 16px",
        marginBottom: "16px",
        fontSize: "14px",
        transition: "all var(--ifm-transition-fast)",
      }}
    >
      {isAlreadyAdded ? "Already Added ✓" : "Add to Wallet"}
    </button>
  );
};

// Wrap in BrowserOnly to prevent SSR issues
export default function AddToWallet() {
  return <BrowserOnly>{() => <AddToWalletButton />}</BrowserOnly>;
}
