mod from_native;
mod into_native;
mod from_ast;
mod into_ast;

pub use crate::value::from_native::{Decimal, Dict, Integer, List, Text};
use std::fmt::{self, Debug, Formatter};
use crate::value::from_native::Byte;

#[derive(Clone, Eq, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(Box<Integer>),
    Decimal(Box<Decimal>),
    String(Box<Text>),
    Byte(Box<Byte>),
    List(Box<List>),
    Dict(Box<Dict>),
}

impl Default for Value {
    fn default() -> Self {
        Self::Null
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Boolean(v) => Debug::fmt(v, f),
            Value::Integer(v) => Debug::fmt(v, f),
            Value::Decimal(v) => Debug::fmt(v, f),
            Value::String(v) => Debug::fmt(v, f),
            Value::Byte(v) => Debug::fmt(v, f),
            Value::List(v) => Debug::fmt(v, f),
            Value::Dict(v) => Debug::fmt(v, f),
        }
    }
}