import React, {FC, useEffect, useMemo} from 'react';
import {ConnectionProvider, useConnection, useWallet, WalletProvider} from '@solana/wallet-adapter-react';
import {WalletAdapterNetwork} from '@solana/wallet-adapter-base';
import {
  LedgerWalletAdapter,
  PhantomWalletAdapter,
  SlopeWalletAdapter,
  SolflareWalletAdapter,
  SolletExtensionWalletAdapter,
  SolletWalletAdapter,
  TorusWalletAdapter,
} from '@solana/wallet-adapter-wallets';
import {
  useRecoilState
} from 'recoil';
import {clusterApiUrl} from '@solana/web3.js';
import {WalletState} from "./WalletState";

require('@solana/wallet-adapter-react-ui/styles.css');

const WalletCatcher: FC = ({children}) => {
  const walletAdapter = useWallet();
  const connection = useConnection();
  const [_wallet, setWalletState] = useRecoilState(WalletState);

  useEffect(() => {
    setTimeout(() => {
      const {
        publicKey,
        connecting,
        connected,
        disconnecting, sendTransaction, signTransaction
      } = walletAdapter;
      setWalletState({
        wallet: {
          sendTransaction,
          signTransaction,
          publicKey,
          connecting,
          connected,
          disconnecting
        },
        connection: connection
      });
    })
  }, [connection, walletAdapter]);

  return <>{children}</>;
};


export const Wallet: FC = ({children}) => {
  // The network can be set to 'devnet', 'testnet', or 'mainnet-beta'.
  const network = WalletAdapterNetwork.Devnet;

  // You can also provide a custom RPC endpoint.
  const endpoint = useMemo(() => "http://127.0.0.1:8899", [network]);

  // @solana/wallet-adapter-wallets includes all the adapters but supports tree shaking and lazy loading --
  // Only the wallets you configure here will be compiled into your application, and only the dependencies
  // of wallets that your users connect to will be loaded.
  const wallets = useMemo(
    () => [
      new PhantomWalletAdapter(),
      new SlopeWalletAdapter(),
      new SolflareWalletAdapter({network}),
      new TorusWalletAdapter(),
      new LedgerWalletAdapter(),
      new SolletWalletAdapter({network}),
      new SolletExtensionWalletAdapter({network}),
    ],
    [network]
  );

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletCatcher>
          {children}
        </WalletCatcher>
      </WalletProvider>
    </ConnectionProvider>
  );
};
