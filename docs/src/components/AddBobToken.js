import React, { useState } from "react";
import BrowserOnly from "@docusaurus/BrowserOnly";

const networks = [
  {
    name: "BOB",
    chainId: "0xED88",
    chainLabel: "BOB Mainnet",
    tokenAddress: "0xb0bd54846a92b214c04a63b26ad7dc5e19a60808",
    rpcUrls: ["https://rpc.gobob.xyz/", "https://bob.gateway.tenderly.co"],
    blockExplorerUrls: ["https://explorer.gobob.xyz/"],
    nativeCurrency: {
      name: "ETH",
      symbol: "ETH",
      decimals: 18,
    },
  },
  {
    name: "Ethereum",
    chainId: "0x1",
    chainLabel: "Ethereum Mainnet",
    tokenAddress: "0xC9746F73cC33a36c2cD55b8aEFD732586946Cedd",
  },
  {
    name: "BSC",
    chainId: "0x38",
    chainLabel: "BNB Smart Chain",
    tokenAddress: "0x52B5fB4B0F6572B8C44d0251Cc224513ac5eB7E7",
    rpcUrls: ["https://bsc-dataseed.binance.org/"],
    blockExplorerUrls: ["https://bscscan.com/"],
    nativeCurrency: {
      name: "BNB",
      symbol: "BNB",
      decimals: 18,
    },
  },
];

const buttonStyle = {
  backgroundColor: "var(--ifm-color-primary)",
  border: "none",
  borderRadius: "6px",
  color: "white",
  cursor: "pointer",
  padding: "10px 16px",
  fontSize: "0.95rem",
  fontWeight: 600,
  transition: "all var(--ifm-transition-fast)",
  width: "100%",
};

async function ensureNetwork(ethereum, network) {
  try {
    await ethereum.request({
      method: "wallet_switchEthereumChain",
      params: [{ chainId: network.chainId }],
    });
  } catch (error) {
    if (error.code === 4902 && network.rpcUrls) {
      await ethereum.request({
        method: "wallet_addEthereumChain",
        params: [
          {
            chainId: network.chainId,
            chainName: network.chainLabel || network.name,
            rpcUrls: network.rpcUrls,
            blockExplorerUrls: network.blockExplorerUrls,
            nativeCurrency: network.nativeCurrency,
          },
        ],
      });
    } else {
      throw error;
    }
  }
}

const TokenButton = ({ network }) => {
  const [status, setStatus] = useState("idle");

  const handleClick = async () => {
    if (status === "loading") {
      return;
    }

    if (!window.ethereum) {
      alert("Please install MetaMask or another injected wallet provider.");
      return;
    }

    try {
      setStatus("loading");
      await ensureNetwork(window.ethereum, network);
      const result = await window.ethereum.request({
        method: "wallet_watchAsset",
        params: {
          type: "ERC20",
          options: {
            address: network.tokenAddress,
            symbol: "BOB",
            decimals: 18,
          },
        },
      });
      if (result) {
        setStatus("success");
      } else {
        setStatus("idle");
      }
    } catch (error) {
      console.error(`Failed to add ${network.name}:`, error);
      alert(`Unable to add BOB on ${network.name}: ${error.message}`);
      setStatus("idle");
    }
  };

  let label = `Add BOB on ${network.name}`;
  if (status === "loading") {
    label = "Requesting...";
  } else if (status === "success") {
    label = `Added on ${network.name} ✓`;
  }

  return (
    <button
      onClick={handleClick}
      style={buttonStyle}
      disabled={status === "loading"}
    >
      {label}
    </button>
  );
};

function TokenButtons() {
  return (
    <div
      style={{
        display: "grid",
        gap: "12px",
        gridTemplateColumns: "repeat(auto-fit, minmax(220px, 1fr))",
        marginBottom: "1rem",
      }}
    >
      {networks.map((network) => (
        <TokenButton key={network.name} network={network} />
      ))}
    </div>
  );
}

export default function AddBobToken() {
  return (
    <BrowserOnly>
      {() => (
        <div>
          <TokenButtons />
          <p style={{ fontSize: "0.85rem", marginBottom: 0 }}>
            Need to move BOB between networks? Use{" "}
            <a href="https://app.transporter.io/?token=BOB" target="_blank">
              CCIP Transporter
            </a>{" "}
            or{" "}
            <a href="https://app.gobob.xyz/en/bridge" target="_blank">
              BOB Gateway Bridge
            </a>
            .
          </p>
        </div>
      )}
    </BrowserOnly>
  );
}


