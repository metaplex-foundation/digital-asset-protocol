use std::collections::BTreeSet;
use crate::modules::Module;
use crate::state::{Standard, Standardized};

pub struct NFTStandard {
    modules: BTreeSet<Module>
}

impl NFTStandard {
    
    pub fn new() -> Self {
        let mut mset = BTreeSet::new();
        mset.insert(Module::Ownership);
        mset.insert(Module::Governance);
        mset.insert(Module::Data(Schema));
        mset.insert(Module::Creators);
        mset.insert(Module::Royalty);
        NFTStandard {
            modules: mset
        }
    }
}

impl Standardized for NFTStandard {
    fn standard(&self) -> Standard {
        Standard::NFT
    }
    fn modules(&self) -> &BTreeSet<Module> {
        &self.modules
    }
}
