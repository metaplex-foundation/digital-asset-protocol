import {ConnectionContextState, WalletContextState, Wallet} from '@solana/wallet-adapter-react';
import {PublicKey, Transaction} from '@solana/web3.js';
import {atom, AtomOptions, RecoilState} from 'recoil';

export type WalletAndConnection = {
  wallet: {
    publicKey: PublicKey | null;
    connecting: boolean;
    connected: boolean;
    disconnecting: boolean,
    sendTransaction: Function,
    signTransaction: ((transaction: Transaction) => Promise<Transaction>) | undefined
  }
  connection: ConnectionContextState
} | undefined;
const defaultState: AtomOptions<WalletAndConnection> = {
  key: 'walletState', // unique ID (with respect to other atoms/selectors)
  default: undefined, // default value (aka initial value)
}
export const WalletState: RecoilState<WalletAndConnection> = atom(defaultState);