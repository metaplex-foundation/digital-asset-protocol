import test from 'tape';
import {Amman, LOCALHOST} from '@metaplex-foundation/amman';
import * as web3 from '@solana/web3.js';
import {Connection, Keypair, PublicKey, Transaction, TransactionInstruction} from '@solana/web3.js';
import debug from 'debug';
import * as beetSolana from '@metaplex-foundation/beet-solana';
import * as beet from '@metaplex-foundation/beet';
import {DigitalAssetTypes} from "../ts/generated/models";
import nacl from 'tweetnacl';
import {sha3_512} from "js-sha3";

const persistLabelsPath = process.env.ADDRESS_LABEL_PATH;
const PROGRAM = new PublicKey("assetbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
export const logDebug = debug('dasset:test:debug');

export async function init() {
    const a = Amman.instance();
    const [payer, payerPair] = await a.addr.genLabeledKeypair('payer');
    const connection = new Connection(LOCALHOST, 'confirmed');
    const transactionHandler = a.payerTransactionHandler(connection, payerPair);
    await a.airdrop(connection, payer, 2);
    return {
        a,
        transactionHandler,
        connection,
        payer,
        payerPair
    };
}


import IAction = DigitalAssetTypes.IAction;
import Interface = DigitalAssetTypes.Interface;
import CreateIdentity = DigitalAssetTypes.CreateIdentity;
import ICreateIdentity = DigitalAssetTypes.ICreateIdentity;
import Action = DigitalAssetTypes.Action;

export type Creator = {
    address: web3.PublicKey;
    verified: boolean;
    share: number;
};

/**
 * @category userTypes
 * @category generated
 */
export const creatorBeet = new beet.BeetArgsStruct<Creator>(
    [
        ['address', beetSolana.publicKey],
        ['verified', beet.bool],
        ['share', beet.u8],
    ],
    'Creator',
);


test("Create An Identity", async () => {
    const {a, transactionHandler, connection, payer, payerPair} = await init();

    let [owner, ownerPair] = await a.addr.genLabeledKeypair("🔨 Owner 1");
    let sig = nacl.sign.detached(Uint8Array.from(Buffer.from("this is an identity")), ownerPair.secretKey)
    const hash = sha3_512.arrayBuffer(sig);
    const digest = Buffer.from(hash.slice(0, 32));
    let idKeypair = Keypair.fromSeed(digest);
    let [identity, bump] = await PublicKey.findProgramAddress([
        Buffer.from("identity"),
        owner.toBuffer(),
    ], PROGRAM);
    await a.addr.addLabel("Identity", identity);
    let g = new Transaction();
    let createIdentity: ICreateIdentity = {
        name: 'first Identity ever'
    };
    let action: IAction = {
        standard: Interface.IdentityAsset,
        data: {discriminator: 1, value: createIdentity}
    };

    g.add(new TransactionInstruction({
        data: Buffer.from(Action.encode(action)),
        programId: PROGRAM,
        keys: [
            {isSigner: true, isWritable: false, pubkey: idKeypair.publicKey},
            {isSigner: true, isWritable: false, pubkey: owner},
            {isSigner: false, isWritable: true, pubkey: identity},
            {isSigner: true, isWritable: true, pubkey: owner}
        ]
    }));

    let tx = await transactionHandler.sendAndConfirmTransaction(g, [
        idKeypair,
        ownerPair
    ], {skipPreflight: true}, "🤓 Testing Identity Creation");


});