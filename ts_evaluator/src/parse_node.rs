use ts_lexer::ast::ParseNode;
use ts_symbol::Value;

use crate::module::Module;


impl Module {
    fn gen_parse_node(&self, node: &ParseNode) -> Value {
        match self {
            _ => {
                Value::Empty
            }
        }
    }
}