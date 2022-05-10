#[derive(Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Module {
    Ownership,
    Metadata,
    Governed,
    Creators,
    Royalty,
    Rights,
    Supply,
}