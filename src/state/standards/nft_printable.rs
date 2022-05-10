

impl NFT_Printable {
    pub fn new() -> Self {
        let mut mset =BTreeSet::new();
        mset.insert(Module::Ownership);
        mset.insert(Module::Governed);
        mset.insert(Module::Data);
        mset.insert(Module::Creators);
        mset.insert(Module::Royalty);
        mset.insert(Module::Supply);
        NFTStandard {
            modules: mset
        }
    }
}

impl Standardized for NFT_Printable {
    fn standard(&self) -> Standard {
        Standard::NFT
    }


    fn modules(&self) -> &BTreeSet<Module> {
        &self.modules
    }

    fn valid_asset(&self) -> bool {
        todo!()
    }
}