import React from 'react';
import './App.css';
import {Wallet} from "./Wallet";
import {Center, Grid, GridItem, Box} from '@chakra-ui/react'
import {WalletDisconnectButton, WalletModalProvider, WalletMultiButton} from '@solana/wallet-adapter-react-ui';
import {selector, atom, useRecoilValue, useRecoilState} from 'recoil';
import {WalletState} from "./WalletState";
import {TOKEN_PROGRAM_ID} from "@solana/spl-token";
import {PublicKey, Connection, Transaction, TransactionInstruction, SystemProgram, Keypair} from "@solana/web3.js";
import {v4} from "uuid";
import {useWallet} from "@solana/wallet-adapter-react";
import {IData, IString, Action, BlobContainer, ModuleType, IAction, ICreateAssetV1, JsonDataSchema, OwnershipModel, RoyaltyModel, InterfaceType} from "./models";

const PROGRAM = new PublicKey("assetbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
const thing = selector({
  key: 'walletReadOnly',
  set: (a) => {},
  get: ({get}) => {
    return get(WalletState);
  },
});

async function mint(owner: PublicKey, payer: PublicKey, hash: string) {
  let idbuf = new Buffer(16);
  v4(null, idbuf);
  let [id, bump] = await PublicKey.findProgramAddress([
    Buffer.from("ASSET", 'utf8'),
    idbuf.slice(0, 8)
  ], PROGRAM);
  let g = new Transaction();
  g.feePayer = payer;
  g.recentBlockhash = hash;
  let creators = new Array(5).fill(0);
  let shares: number[] = [];
  creators = creators.map(async (c, i) => {
    shares.push(20);
    let d = new Keypair();
    return [d.publicKey, d];
  });
  creators = await Promise.all(creators);
  let creator_metas = creators.map((c) =>
    ({
      pubkey: c[0],
      isSigner: false,
      isWritable: false
    })
  );
  let createAsset: ICreateAssetV1 = {
    uri: `https://gist.githubusercontent.com/austbot/fcc45b63119d12a588cc6b5bda2c7fa3/raw/91f7297eeb203e69bb7d19fa9f77b34139f88e20/image.json`,
    ownershipModel: OwnershipModel.Single,
    royaltyModel: RoyaltyModel.Address,
    royaltyTarget: [{
      address: new PublicKey("Gsv13oph2i6nkJvNkVfuzkcbHWchz6viUtEg2vsxQMtM").toBytes(),
      share: 800
    }],
    creatorShares: Uint8Array.from(shares),
    dataSchema: JsonDataSchema.MultiMedia,
    uuid: Uint8Array.from(idbuf)
  };
  let action: IAction = {
    interface: InterfaceType.NFT,
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
      },
      ...creator_metas
    ]
  }));
  return g;
}


const selectedIdentity = atom<NFT | undefined>({
  key: 'identitySelection',
  default: undefined,
});

type NFT = { name: string, img: string, id: string }

function App() {

  const walletState = useRecoilValue(thing)
  const appName = "Name of App";
  const [nfts, setNfts] = React.useState<NFT[]>([]);
  const [identity, setIdentity] = useRecoilState(selectedIdentity);
  const selectNft = (nft: NFT) => {
    setIdentity(nft);
  };
  const back = () => {
    setIdentity(undefined);
  };

  async function identify() {
    if (walletState?.wallet?.publicKey) {
      let conn = new Connection("http://127.0.0.1:8899", "confirmed");
      await conn.requestAirdrop(walletState?.wallet?.publicKey, 2*10000000000)

      const {
        context: { slot: minContextSlot },
        value: { blockhash, lastValidBlockHeight }
      } = await conn.getLatestBlockhashAndContext();

      let tx = await mint(walletState.wallet.publicKey, walletState.wallet.publicKey, blockhash);
      // @ts-ignore
      tx = await walletState.wallet.signTransaction(tx);
      const signature = await conn.sendRawTransaction(tx.serialize(), {skipPreflight: true});
      await conn.confirmTransaction({ blockhash, lastValidBlockHeight, signature });

      let asset_key = tx.instructions[0].keys[1].pubkey;
      let data = await conn.getAccountInfoAndContext(asset_key, "confirmed");
      let arr = data?.value?.data.valueOf();
      if (arr) {
        let blob = BlobContainer.decode(arr)
        let data_module = blob.blobs.get(ModuleType.Data);
        let data_module_data = data_module?.value as IData;
        let uri = data_module_data?.layout?.get(0)?.value.value as IString;
        if (uri.value) {
          const json = await fetch(uri.value).then(r => r.json());
          let meta = {name: json.metadata[0].name, img: json.files[0].quality.modes.high.uri, id: asset_key.toBase58()};
          setNfts([meta])
        }
      }
    }
  }

  return (
    <Wallet>
      <WalletModalProvider>
        <WalletMultiButton/>
        <WalletDisconnectButton/>
        {walletState && walletState.wallet.connected &&
          <div className="App">
            <header className="App-header">
              Mint a DASSET

              <span>
                   <button onClick={identify}>Click it</button>
                 </span>
              <Center>

                <Grid templateColumns='repeat(5, 1fr)' gap={6}>
                  {nfts && nfts.length > 0 && nfts.map((nft) => {
                    return <GridItem key={nft.id} className="nft">
                      <div onClick={() => selectNft(nft)}>
                        <span>{nft.id}</span>
                        <img src={nft.img} alt={nft.name}/>
                        <div>{nft.name}</div>
                      </div>
                    </GridItem>
                  })}
                </Grid>
              </Center>
            </header>
          </div>
        }
        {walletState && !walletState.wallet.connected &&
          <span>Loading</span>
        }
      </WalletModalProvider>
    </Wallet>
  );

}

export default App;