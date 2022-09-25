pub mod plain;
#[cfg(feature = "tagged")]
pub mod tagged;

pub use plain::UnionFind;
#[cfg(feature = "tagged")]
pub use tagged::UnionFind as TaggedUnionFind;
