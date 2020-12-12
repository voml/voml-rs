pub mod byte;
pub mod dict;
pub mod list;
pub mod number;
pub mod string;

pub use byte::Byte;
pub use dict::Dict;
pub use list::List;
pub use string::Text;

use crate::Value;
use arc_number::{BigDecimal, BigInt, BigUint};
use indexmap::IndexMap;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
    fmt::{self, Debug, Formatter},
};
use std::ops::Index;
use std::fmt::Display;

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self::List(Box::new(List::default()))
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Value::Null,
        }
    }
}

impl<T, E> From<Result<T, E>> for Value
where
    T: Into<Value>,
{
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(value) => value.into(),
            Err(_) => Value::Null,
        }
    }
}
