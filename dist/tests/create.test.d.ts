import { Amman } from '@metaplex-foundation/amman';
import * as web3 from '@solana/web3.js';
import debug from 'debug';
import * as beet from '@metaplex-foundation/beet';
export declare const logDebug: debug.Debugger;
export declare function init(): Promise<{
    a: Amman;
    transactionHandler: import("@metaplex-foundation/amman/dist/transactions/transaction-handler").PayerTransactionHandler;
    connection: web3.Connection;
    payer: web3.PublicKey;
    payerPair: web3.Keypair;
}>;
export declare type Creator = {
    address: web3.PublicKey;
    verified: boolean;
    share: number;
};
/**
 * @category userTypes
 * @category generated
 */
export declare const creatorBeet: beet.BeetArgsStruct<Creator>;
