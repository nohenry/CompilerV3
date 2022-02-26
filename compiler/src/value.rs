use crate::lexer::Operator;


#[derive(Debug)]
pub enum Value {
    None,
    UnsignedInteger(u64),
    SignedInteger(i64)
}

impl Value {
    fn can_apply_binary(&self, operator: &Operator, other: &Value) -> bool {
        match self {
            _ => false
        }
    }
}