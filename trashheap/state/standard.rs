

#[derive(Clone)]
enum Standard {
    Unknown,
    NFTv1,
    NFT,
    NFTGroup,
    FungibleAsset,
}

struct Standard {
    name: String,
    modules: Vec
}

trait Standardized {

    fn modules() -> Ord<Mod>
}