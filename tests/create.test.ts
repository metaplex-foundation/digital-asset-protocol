import test from 'tape';
import {Amman, LOCALHOST} from '@metaplex-foundation/amman';
import * as web3 from '@solana/web3.js';
import {Connection, PublicKey, SystemProgram, Transaction, TransactionInstruction} from '@solana/web3.js';
import debug from 'debug';
import * as beetSolana from '@metaplex-foundation/beet-solana';
import * as beet from '@metaplex-foundation/beet';
import {DigitalAssetTypes} from "../ts/generated/models";
import uuid from "uuid";

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
import Action = DigitalAssetTypes.Action;
import ICreateAssetV1 = DigitalAssetTypes.ICreateAssetV1;
import OwnershipModel = DigitalAssetTypes.OwnershipModel;
import RoyaltyModel = DigitalAssetTypes.RoyaltyModel;
import JsonDataSchema = DigitalAssetTypes.JsonDataSchema;

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
  let idbuf = new Buffer(16);
  uuid.v4(null, idbuf)
  let [id, bump] = await PublicKey.findProgramAddress([
    Buffer.from("asset"),
    idbuf
  ], PROGRAM);
  await a.addr.addLabel("Asset", id);
  let g = new Transaction();


  let createAsset: ICreateAssetV1 = {
    uri: `https://gist.githubusercontent.com/austbot/fcc45b63119d12a588cc6b5bda2c7fa3/raw/91f7297eeb203e69bb7d19fa9f77b34139f88e20/image.json`,
    ownershipModel: OwnershipModel.Single,
    royaltyModel: RoyaltyModel.Address,
    royaltyTarget: [{
      address: new PublicKey("Gsv13oph2i6nkJvNkVfuzkcbHWchz6viUtEg2vsxQMtM").toBytes(),
      share: 100
    }],
    dataSchema: JsonDataSchema.MultiMedia
  };
  let action: IAction = {
    standard: Interface.NFT,
    data: {discriminator: 2, value: createAsset}
  };

  g.add(new TransactionInstruction({
    data: Buffer.from(Action.encode(action)),
    programId: PROGRAM,
    keys: [
      {
        isSigner: false,
        isWritable: false,
        pubkey: SystemProgram.programId
      },
      {
        isSigner: false,
        isWritable: true,
        pubkey: id
      },
      {
        pubkey: owner,
        isSigner: false,
        isWritable: false
      },
      {
        pubkey: payer,
        isSigner: true,
        isWritable: true
      }
    ]
  }));

  let tx = await transactionHandler.sendAndConfirmTransaction(g, [
    payerPair
  ], {skipPreflight: true}, "🤓 Testing DAS Asset Creation");


});