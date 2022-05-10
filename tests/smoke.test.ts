import test from 'tape';
import {ActionT} from "../ts-src/generated/digital-asset/action";
import {LifeCycle} from "../ts-src/generated/digital-asset/life-cycle";
import * as flatbuffers from 'flatbuffers';
import {CreateT} from "../ts-src/generated/digital-asset/create";
import {ModuleWrapT} from "../ts-src/generated/digital-asset/module-wrap";
import {RoyaltyT} from "../ts-src/generated/digital-asset/royalty";
import {RoyaltyTarget} from "../ts-src/generated/digital-asset/royalty-target";
import {Module} from "../ts-src/generated/digital-asset/module";
import {Amman, LOCALHOST} from '@metaplex-foundation/amman';
import {Connection, PublicKey, Transaction, TransactionInstruction} from '@solana/web3.js';
import debug from 'debug';
import {ActionPayload} from "../ts-src/generated/digital-asset/action-payload";
import {CreatorsT} from "../ts-src/generated/digital-asset/creators";
import {CreatorT} from "../ts-src/generated/asset";

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


import * as web3 from '@solana/web3.js';
import * as beetSolana from '@metaplex-foundation/beet-solana';
import * as beet from '@metaplex-foundation/beet';
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


test("Flatbuffers", async () => {
    const {a, transactionHandler, connection, payer, payerPair} = await init();
    {
        let [creator, creatorPair] = await a.addr.genLabeledKeypair("🔨 Creator 1");
        const act = new CreatorT();
        act.verified = true;
        act.share = 100;
        act.address = Array.from(creator.toBytes());
        let fbb = new flatbuffers.Builder(0);
        let off = act.pack(fbb);
        fbb.finish(off);
        let arr = Array.from(fbb.asUint8Array())
        arr.unshift(0)
        const ix = Buffer.from(arr);
        let g = new Transaction();
        g.add(new TransactionInstruction({
            data: ix,
            programId: PROGRAM,
            keys: [
                {isSigner: true, isWritable: true, pubkey: payer}
            ]
        }));

        await transactionHandler.sendAndConfirmTransaction(g, [], {skipPreflight: true}, "🤓 Testing FlatBuffers");
    }

    {
        let [creator, creatorPair] = await a.addr.genLabeledKeypair("🔨 Creator 1");
        let c: Creator = {
            address: creator,
            verified: true,
            share: 100
        };
        let buf = Array.from(creatorBeet.serialize(c)[0]);
        buf.unshift(1)
        const ix = Buffer.from(buf);
        console.log(ix);

        let g = new Transaction();
        g.add(new TransactionInstruction({
            data: ix,
            programId: PROGRAM,
            keys: [
                {isSigner: true, isWritable: true, pubkey: payer},
                {isSigner: false, isWritable: false, pubkey: creator}
            ]
        }));

        await transactionHandler.sendAndConfirmTransaction(g, [], {skipPreflight: true}, "😡 Testing Borsh");
    }
});