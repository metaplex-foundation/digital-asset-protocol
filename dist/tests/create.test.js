"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.creatorBeet = exports.init = exports.logDebug = void 0;
const tape_1 = __importDefault(require("tape"));
const amman_client_1 = require("@metaplex-foundation/amman-client");
const web3_js_1 = require("@solana/web3.js");
const debug_1 = __importDefault(require("debug"));
const beetSolana = __importStar(require("@metaplex-foundation/beet-solana"));
const beet = __importStar(require("@metaplex-foundation/beet"));
const models_1 = require("../ts/generated/models");
const uuid_1 = require("uuid");
const persistLabelsPath = process.env.ADDRESS_LABEL_PATH;
const PROGRAM = new web3_js_1.PublicKey("assetbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
exports.logDebug = (0, debug_1.default)('dasset:test:debug');
async function init() {
    const a = amman_client_1.Amman.instance();
    const [payer, payerPair] = await a.addr.genLabeledKeypair('payer');
    const connection = new web3_js_1.Connection(amman_client_1.LOCALHOST, 'confirmed');
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
exports.init = init;
var Interface = models_1.DigitalAssetTypes.InterfaceType;
var Action = models_1.DigitalAssetTypes.Action;
var OwnershipModel = models_1.DigitalAssetTypes.OwnershipModel;
var RoyaltyModel = models_1.DigitalAssetTypes.RoyaltyModel;
var JsonDataSchema = models_1.DigitalAssetTypes.JsonDataSchema;
/**
 * @category userTypes
 * @category generated
 */
exports.creatorBeet = new beet.BeetArgsStruct([
    ['address', beetSolana.publicKey],
    ['verified', beet.bool],
    ['share', beet.u8],
], 'Creator');
(0, tape_1.default)("Create An Asset", async () => {
    const { a, transactionHandler, connection, payer, payerPair } = await init();
    let [owner, ownerPair] = await a.addr.genLabeledKeypair("Owner");
    let idbuf = new Buffer(16);
    (0, uuid_1.v4)(null, idbuf);
    let [id, bump] = await web3_js_1.PublicKey.findProgramAddress([
        Buffer.from("ASSET", 'utf8'),
        idbuf.slice(0, 8)
    ], PROGRAM);
    await a.addr.addLabel("Asset", id);
    let g = new web3_js_1.Transaction();
    let creators = new Array(5).fill(0);
    let shares = [];
    creators = creators.map(async (c, i) => {
        shares.push(20);
        return a.addr.genLabeledKeypair(`🔨 Creator ${i}`);
    });
    creators = await Promise.all(creators);
    let creator_metas = creators.map((c) => ({
        pubkey: c[0],
        isSigner: true,
        isWritable: false
    }));
    let createAsset = {
        uri: `https://gist.githubusercontent.com/austbot/fcc45b63119d12a588cc6b5bda2c7fa3/raw/91f7297eeb203e69bb7d19fa9f77b34139f88e20/image.json`,
        ownershipModel: OwnershipModel.Single,
        royaltyModel: RoyaltyModel.Address,
        royaltyTarget: [{
                address: new web3_js_1.PublicKey("Gsv13oph2i6nkJvNkVfuzkcbHWchz6viUtEg2vsxQMtM").toBytes(),
                share: 800
            }],
        creatorShares: Uint8Array.from(shares),
        dataSchema: JsonDataSchema.MultiMedia,
        uuid: Uint8Array.from(idbuf)
    };
    let action = {
        interface: Interface.NFT,
        data: { discriminator: 2, value: createAsset }
    };
    g.add(new web3_js_1.TransactionInstruction({
        data: Buffer.from(Action.encode(action)),
        programId: PROGRAM,
        keys: [
            {
                isSigner: false,
                isWritable: false,
                pubkey: web3_js_1.SystemProgram.programId
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
            },
            ...creator_metas
        ]
    }));
    let tx = await transactionHandler.sendAndConfirmTransaction(g, [
        payerPair,
        ...creators.map((c) => c[1])
    ], { skipPreflight: true }, "🤓 Testing DAS Asset Creation").assertNone();
});
//# sourceMappingURL=create.test.js.map