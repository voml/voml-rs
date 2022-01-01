mod literal_pattern;
mod ordered_map;
mod ordered_set;
mod sparse_array;

pub use literal_pattern::*;
pub use ordered_map::*;
pub use ordered_set::*;
pub use sparse_array::*;

use super::*;

/// Ordered set of values
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct OrderedSet {
    inner: IndexSet<Literal<Value>>,
}

/// Ordered map of key value pairs
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct OrderedMap {
    inner: IndexMap<String, LiteralPair>,
}

/// Ordered map of key value pairs
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiteralPair {
    key: Literal<String>,
    value: Literal<Value>,
}

/// Literal Patterns for command
#[derive(Clone, Default, Eq, PartialEq)]
pub struct LiteralPattern {
    inner: Vec<Literal<String>>,
}

/// Sparse representation of the array, the subscript can be any non-zero integer
/// 1-index
#[derive(Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct SparseArray {
    default: Value,
    inner: BTreeMap<BigUint, Literal<Value>>,
}
