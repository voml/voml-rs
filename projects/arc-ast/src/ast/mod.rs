mod display;
mod into_value;
mod literal;
mod statements;

use crate::{
    value::{parse_number, Decimal, Integer, Text, TextDelimiter},
    Value,
};
use lsp_types::Range;
use num::{BigInt, Num};
pub use statements::{ExtendFormat, ExtendStatement};

/// AST tree for arc
#[derive(Clone, Eq, PartialEq)]
pub struct AST {
    /// AST data
    pub kind: ASTKind,
    /// 1-indexed start to end position
    pub range: Range,
    /// Whitespace, Newline, Comment
    pub additional: Option<String>,
}

/// missing
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ASTKind {
    /// Placeholder
    /// Unreachable structure
    None,
    /// Root of all nodes
    Program(Vec<AST>),
    /// Flattened structure
    Sequence(Vec<AST>),
    /// Plain Text, NewLines
    Span(String),
    ExtendStatement(Box<ExtendStatement>),
    /// `[list.scope]`
    ListScope(usize, Box<AST>),
    /// `{dict.scope}`
    DictScope(usize, Box<AST>),
    ///
    Pair(Box<AST>, Box<AST>),
    /// `null`
    Null,
    /// `true` | `false`
    Boolean(bool),
    ///
    String(Box<Text>),
    ///
    Namespace(Vec<AST>),
    ///
    Integer(Box<Integer>),
    ///
    Decimal(Box<Decimal>),
    ///
    Cite(Box<AST>),
    ///
    Dict(Vec<AST>),
    ///
    List(Vec<AST>),
}

impl Default for AST {
    fn default() -> Self {
        Self { kind: ASTKind::None, range: Default::default(), additional: None }
    }
}

impl From<ASTKind> for AST {
    fn from(kind: ASTKind) -> Self {
        Self { kind, range: Default::default(), additional: None }
    }
}

// impl AST {
//     pub fn as_vec(&self) -> Vec<AST> {
//         match &self.kind {
//             ASTKind::Program(v) | ASTKind::Block(v) => v.to_owned(),
//             _ => vec![],
//         }
//     }
// }

impl AST {
    ///
    pub fn set_additional(&mut self, info: impl Into<String>) {
        self.additional = Some(info.into())
    }
}

#[allow(non_upper_case_globals)]
impl ASTKind {
    ///
    pub fn into_node(self) -> AST {
        AST {
            kind: self,
            range: Default::default(),
            additional: None
        }
    }
    ///
    pub const True: Self = ASTKind::Boolean(true);
    ///
    pub const False: Self = ASTKind::Boolean(false);
}

impl ASTKind {
    ///
    pub fn program(children: Vec<AST>) -> Self {
        Self::Program(children)
    }
    ///
    pub fn block(children: Vec<AST>) -> Self {
        Self::Sequence(children)
    }
    // pub fn statement(children: Vec<AST>, r: TextRange) -> Self {
    //     Self { kind: ASTKind::Statement(children), range: box_range(r) }
    // }
    //
    // pub fn if_else_chain(cds: Vec<AST>, acts: Vec<AST>, r: TextRange) -> Self {
    //     let kind = ASTKind::IfElseChain(Box::new(IfElseChain::build(cds, acts)));
    //     Self { kind, range: box_range(r) }
    // }
    //
    // pub fn for_in_loop(pattern: AST, terms: AST, block: AST, guard: Option<AST>, for_else: Option<AST>, r: TextRange) -> Self {
    //     let kind = ASTKind::ForInLoop(Box::new(ForInLoop { pattern, terms, guard, block, for_else }));
    //     Self { kind, range: box_range(r) }
    // }
    //
    // pub fn expression(children: AST, eos: bool, r: TextRange) -> Self {
    //     let kind = ASTKind::Expression(Box::new(children), eos);
    //     Self { kind, range: box_range(r) }
    // }
    //
    // pub fn operation(op: &str, kind: &str, r: TextRange) -> Self {
    //     let o = match kind {
    //         "<" => Operator::prefix(op),
    //         ">" => Operator::suffix(op),
    //         _ => Operator::infix(op),
    //     };
    //     let kind = ASTKind::Operator(Box::new(o));
    //     Self { kind, range: box_range(r) }
    // }
    //
    // pub fn string_expression(value: Vec<AST>, handler: Option<AST>, r: TextRange) -> Self {
    //     let kind = ASTKind::StringExpression(Box::new(StringExpression { handler, inner: value }));
    //     Self { kind, range: box_range(r) }
    // }
    //
    // pub fn infix_expression(op: AST, lhs: AST, rhs: AST, r: TextRange) -> Self {
    //     let kind = ASTKind::InfixExpression(Box::new(InfixExpression { op, lhs, rhs }));
    //     Self { kind, range: box_range(r) }
    // }
    //
    // pub fn prefix_expression(op: AST, rhs: AST, r: TextRange) -> Self {
    //     let kind = ASTKind::PrefixExpression(Box::new(UnaryExpression { op, base: rhs }));
    //     Self { kind, range: box_range(r) }
    // }
    //
    // pub fn suffix_expression(op: AST, lhs: AST, r: TextRange) -> Self {
    //     let kind = ASTKind::PrefixExpression(Box::new(UnaryExpression { op, base: lhs }));
    //     Self { kind, range: box_range(r) }
    // }
    //
    // pub fn call_chain(chain: CallChain, r: TextRange) -> Self {
    //     Self { kind: ASTKind::CallChain(Box::new(chain)), range: box_range(r) }
    // }
    //
    // pub fn call_index(index: &str, r: TextRange) -> Self {
    //     let n = BigInt::parse_bytes(index.as_bytes(), 10).unwrap_or_default();
    //     Self { kind: ASTKind::CallIndex(Box::new(n)), range: box_range(r) }
    // }
    //
    // pub fn template(value: Template, r: TextRange) -> Self {
    //     Self { kind: ASTKind::Template(Box::new(value)), range: box_range(r) }
    // }
    //
    ///
    pub fn null() -> Self {
        Self::Null
    }
    ///
    pub fn boolean(value: bool) -> Self {
        Self::Boolean(value)
    }
    ///
    pub fn string(value: Text) -> Self {
        Self::String(Box::new(value))
    }
    // pub fn string_escaped(value: String, r: TextRange) -> Self {
    //     Self { kind: ASTKind::EscapedText(value), range: box_range(r) }
    // }
    ///
    pub fn integer(value: &str) -> Self {
        let n = BigInt::from_str_radix(value, 10).unwrap_or_default();
        Self::Integer(Box::new(Integer::from(n)))
    }
    ///
    pub fn number(value: &str) -> Self {
        match parse_number(value) {
            Some(Value::Integer(n)) => Self::Integer(n),
            Some(Value::Decimal(n)) => Self::Decimal(n),
            _ => ASTKind::Null,
        }
    }
    ///
    pub fn namespace(value: Vec<AST>) -> Self {
        Self::Namespace(value)
    }
    ///
    pub fn list(value: Vec<AST>) -> Self {
        Self::List(value)
    }
    ///
    pub fn dict(pairs: Vec<AST>) -> Self {
        Self::Dict(pairs)
    }
    ///
    pub fn pair(lhs: AST, rhs: AST) -> Self {
        Self::Pair(Box::new(lhs), Box::new(rhs))
    }
}

impl Text {
    ///
    pub fn string_escaped(value: impl Into<String>, handler: impl Into<String>, delimiter: usize) -> Self {
        let handler = handler.into();
        let handler = match handler.len() {
            0 => None,
            _ => Some(handler),
        };
        Text { handler, delimiter: TextDelimiter::Quotation(delimiter), value: value.into() }
    }
    ///
    pub fn string_literal(value: impl Into<String>, handler: impl Into<String>, delimiter: usize) -> Self {
        let handler = handler.into();
        let handler = match handler.len() {
            0 => None,
            _ => Some(handler),
        };
        Text { handler, delimiter: TextDelimiter::Apostrophe(delimiter), value: value.into() }
    }
    ///
    pub fn string_bare(value: impl Into<String>) -> Self {
        Text { handler: None, delimiter: TextDelimiter::Bare, value: value.into() }
    }
}
