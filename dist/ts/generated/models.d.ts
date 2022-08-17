import { BebopView } from "bebop";
export declare namespace DigitalAssetTypes {
    enum InterfaceType {
        Unknown = 0,
        NFTv1 = 1,
        NFT = 2,
        NFTPrintable = 3,
        NFTGroup = 4,
        FungibleAsset = 5,
        IdentityAsset = 6
    }
    enum ModuleType {
        Invalid = 0,
        Signature = 1,
        Ownership = 2,
        Creators = 3,
        Data = 4,
        Royalty = 5,
        Rights = 6,
        Extension = 7,
        Governance = 8
    }
    interface ICreatorsData {
        creators: Array<ICreator>;
    }
    const CreatorsData: {
        discriminator: 1;
        encode(message: ICreatorsData): Uint8Array;
        encodeInto(message: ICreatorsData, view: BebopView): number;
        decode(buffer: Uint8Array): ICreatorsData;
        readFrom(view: BebopView): ICreatorsData;
    };
    interface IOwnershipData {
        model: OwnershipModel;
        owner: Uint8Array;
    }
    const OwnershipData: {
        discriminator: 2;
        encode(message: IOwnershipData): Uint8Array;
        encodeInto(message: IOwnershipData, view: BebopView): number;
        decode(buffer: Uint8Array): IOwnershipData;
        readFrom(view: BebopView): IOwnershipData;
    };
    interface IRoyaltyData {
        royalty: number;
        model: RoyaltyModel;
        target: Array<IRoyaltyTarget>;
        locked: boolean;
    }
    const RoyaltyData: {
        discriminator: 3;
        encode(message: IRoyaltyData): Uint8Array;
        encodeInto(message: IRoyaltyData, view: BebopView): number;
        decode(buffer: Uint8Array): IRoyaltyData;
        readFrom(view: BebopView): IRoyaltyData;
    };
    interface IGovernanceData {
        authorities: Array<IAuthority>;
    }
    const GovernanceData: {
        discriminator: 4;
        encode(message: IGovernanceData): Uint8Array;
        encodeInto(message: IGovernanceData, view: BebopView): number;
        decode(buffer: Uint8Array): IGovernanceData;
        readFrom(view: BebopView): IGovernanceData;
    };
    interface IData {
        layout: Map<number, IDataItem>;
    }
    const Data: {
        discriminator: 5;
        encode(message: IData): Uint8Array;
        encodeInto(message: IData, view: BebopView): number;
        decode(buffer: Uint8Array): IData;
        readFrom(view: BebopView): IData;
    };
    type IModuleData = {
        discriminator: 1;
        value: ICreatorsData;
    } | {
        discriminator: 2;
        value: IOwnershipData;
    } | {
        discriminator: 3;
        value: IRoyaltyData;
    } | {
        discriminator: 4;
        value: IGovernanceData;
    } | {
        discriminator: 5;
        value: IData;
    };
    const ModuleData: {
        encode(message: IModuleData): Uint8Array;
        encodeInto(message: IModuleData, view: BebopView): number;
        decode(buffer: Uint8Array): IModuleData;
        readFrom(view: BebopView): IModuleData;
    };
    enum Encoding {
        Invalid = 0,
        Borsh = 1,
        Bincode = 2
    }
    interface IString {
        value?: string;
    }
    const String: {
        discriminator: 1;
        encode(message: IString): Uint8Array;
        encodeInto(message: IString, view: BebopView): number;
        decode(buffer: Uint8Array): IString;
        readFrom(view: BebopView): IString;
    };
    interface IInt {
        value?: number;
    }
    const Int: {
        discriminator: 2;
        encode(message: IInt): Uint8Array;
        encodeInto(message: IInt, view: BebopView): number;
        decode(buffer: Uint8Array): IInt;
        readFrom(view: BebopView): IInt;
    };
    interface IBigInt {
        value?: bigint;
    }
    const BigInt: {
        discriminator: 3;
        encode(message: IBigInt): Uint8Array;
        encodeInto(message: IBigInt, view: BebopView): number;
        decode(buffer: Uint8Array): IBigInt;
        readFrom(view: BebopView): IBigInt;
    };
    interface IBytes {
        encoding: Encoding;
        raw: Uint8Array;
    }
    const Bytes: {
        discriminator: 4;
        encode(message: IBytes): Uint8Array;
        encodeInto(message: IBytes, view: BebopView): number;
        decode(buffer: Uint8Array): IBytes;
        readFrom(view: BebopView): IBytes;
    };
    type IDataItemValue = {
        discriminator: 1;
        value: IString;
    } | {
        discriminator: 2;
        value: IInt;
    } | {
        discriminator: 3;
        value: IBigInt;
    } | {
        discriminator: 4;
        value: IBytes;
    };
    const DataItemValue: {
        encode(message: IDataItemValue): Uint8Array;
        encodeInto(message: IDataItemValue, view: BebopView): number;
        decode(buffer: Uint8Array): IDataItemValue;
        readFrom(view: BebopView): IDataItemValue;
    };
    interface IDataItem {
        key: string;
        value: IDataItemValue;
    }
    const DataItem: {
        encode(message: IDataItem): Uint8Array;
        encodeInto(message: IDataItem, view: BebopView): number;
        decode(buffer: Uint8Array): IDataItem;
        readFrom(view: BebopView): IDataItem;
    };
    interface IBlobContainer {
        blobs: Map<number, IModuleData>;
    }
    const BlobContainer: {
        encode(message: IBlobContainer): Uint8Array;
        encodeInto(message: IBlobContainer, view: BebopView): number;
        decode(buffer: Uint8Array): IBlobContainer;
        readFrom(view: BebopView): IBlobContainer;
    };
    interface IAuthority {
        scopes: Array<string>;
        address: Uint8Array;
    }
    const Authority: {
        encode(message: IAuthority): Uint8Array;
        encodeInto(message: IAuthority, view: BebopView): number;
        decode(buffer: Uint8Array): IAuthority;
        readFrom(view: BebopView): IAuthority;
    };
    interface IAction {
        interface: InterfaceType;
        data: IActionData;
    }
    const Action: {
        encode(message: IAction): Uint8Array;
        encodeInto(message: IAction, view: BebopView): number;
        decode(buffer: Uint8Array): IAction;
        readFrom(view: BebopView): IAction;
    };
    interface ICreateIdentityV1 {
        uri?: string;
    }
    const CreateIdentityV1: {
        discriminator: 1;
        encode(message: ICreateIdentityV1): Uint8Array;
        encodeInto(message: ICreateIdentityV1, view: BebopView): number;
        decode(buffer: Uint8Array): ICreateIdentityV1;
        readFrom(view: BebopView): ICreateIdentityV1;
    };
    interface ICreateAssetV1 {
        uri?: string;
        ownershipModel?: OwnershipModel;
        royaltyModel?: RoyaltyModel;
        royalty?: number;
        dataSchema?: JsonDataSchema;
        creatorShares?: Uint8Array;
        royaltyTarget?: Array<IRoyaltyTarget>;
        authorities?: Array<IAuthority>;
        uuid?: Uint8Array;
    }
    const CreateAssetV1: {
        discriminator: 2;
        encode(message: ICreateAssetV1): Uint8Array;
        encodeInto(message: ICreateAssetV1, view: BebopView): number;
        decode(buffer: Uint8Array): ICreateAssetV1;
        readFrom(view: BebopView): ICreateAssetV1;
    };
    interface IUpdateAssetV1 {
        msg?: string;
    }
    const UpdateAssetV1: {
        discriminator: 3;
        encode(message: IUpdateAssetV1): Uint8Array;
        encodeInto(message: IUpdateAssetV1, view: BebopView): number;
        decode(buffer: Uint8Array): IUpdateAssetV1;
        readFrom(view: BebopView): IUpdateAssetV1;
    };
    type IActionData = {
        discriminator: 1;
        value: ICreateIdentityV1;
    } | {
        discriminator: 2;
        value: ICreateAssetV1;
    } | {
        discriminator: 3;
        value: IUpdateAssetV1;
    };
    const ActionData: {
        encode(message: IActionData): Uint8Array;
        encodeInto(message: IActionData, view: BebopView): number;
        decode(buffer: Uint8Array): IActionData;
        readFrom(view: BebopView): IActionData;
    };
    const MAX_MODULES: number;
    enum OwnershipModel {
        Invalid = 0,
        Single = 1,
        Token = 2
    }
    enum RoyaltyModel {
        Invalid = 0,
        Address = 1,
        Fanout = 2,
        Creators = 3
    }
    interface IRoyaltyTarget {
        address: Uint8Array;
        share: number;
    }
    const RoyaltyTarget: {
        encode(message: IRoyaltyTarget): Uint8Array;
        encodeInto(message: IRoyaltyTarget, view: BebopView): number;
        decode(buffer: Uint8Array): IRoyaltyTarget;
        readFrom(view: BebopView): IRoyaltyTarget;
    };
    interface ICreator {
        address: Uint8Array;
        share: number;
        verified: boolean;
    }
    const Creator: {
        encode(message: ICreator): Uint8Array;
        encodeInto(message: ICreator, view: BebopView): number;
        decode(buffer: Uint8Array): ICreator;
        readFrom(view: BebopView): ICreator;
    };
    enum Lifecycle {
        Invalid = 0,
        Create = 1,
        Transfer = 2,
        Destroy = 3,
        Update = 4,
        Freeze = 5,
        Thaw = 6,
        Split = 7,
        Combine = 8,
        SupplyIncrease = 9,
        SupplyDecrease = 10,
        ActivateExtension = 11,
        DeactivateExtension = 12
    }
    enum JsonDataSchema {
        Invalid = 0,
        Core = 1,
        MultiMedia = 2,
        SimpleImage = 3,
        SimpleAudio = 4,
        Simple3d = 5,
        SimpleText = 6,
        MusicRecording = 7
    }
}
