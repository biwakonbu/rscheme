#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Function,
    Number,
    Symbol,
    SExpression,
}
