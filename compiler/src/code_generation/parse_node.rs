use std::ffi::CString;

use llvm_sys::{
    c_str,
    core::{
        LLVMAddFunction, LLVMAppendBasicBlock, LLVMBuildAlloca, LLVMBuildRetVoid, LLVMBuildStore,
        LLVMFunctionType, LLVMPositionBuilderAtEnd,
    },
};

use crate::{
    ast::{FunctionDecleration, ParseNode, VariableDecleration},
    cast,
    lexer::TokenKind,
};

use super::{module::Module, SymbolValue, Value};

impl Module {
    pub(super) fn gen_parse_node(&self, node: &ParseNode) {
        match node {
            ParseNode::Expression(e, _) => {
                self.gen_expression(e);
            }
            ParseNode::VariableDecleration(VariableDecleration {
                identifier,
                possible_initializer,
                variable_type,
                ..
            }) => {
                let ty = if let Some(ty) = variable_type {
                    self.gen_type(ty.as_ref())
                } else if let Some((init, ..)) = possible_initializer {
                    self.gen_node_type(init.as_ref())
                } else {
                    return;
                };

                let var = unsafe { LLVMBuildAlloca(self.builder, ty.llvm_type, c_str!("")) };

                if let Some((init, ..)) = possible_initializer {
                    let value = self.gen_expression(init.as_ref());
                    unsafe {
                        LLVMBuildStore(self.builder, value.llvm_value, var);
                    }
                }

                let name = cast!(&identifier.token_type, TokenKind::Ident);
                self.get_current_mut(|f| {
                    if let Some(sym) = f {
                        let var_value = Value::new(var, Box::new(ty));
                        sym.add_child(name, SymbolValue::Variable(var_value));
                    }
                })
            }
            ParseNode::FunctionDecleration(FunctionDecleration {
                body,
                function_type,
                generic,
                identifier,
                ..
            }) => {
                let return_type = self.gen_type(function_type.return_type.as_ref());
                let name =
                    CString::new(cast!(&identifier.token_type, TokenKind::Ident).as_str()).unwrap();

                unsafe {
                    let func_type =
                        LLVMFunctionType(return_type.llvm_type, std::ptr::null_mut(), 0, 0);
                    let func = LLVMAddFunction(self.module, name.as_ptr(), func_type);

                    let block = LLVMAppendBasicBlock(func, c_str!(""));
                    // self.current_block = block;
                    self.current_block.replace(block);
                    LLVMPositionBuilderAtEnd(
                        self.builder,
                        self.current_block.borrow().as_mut().unwrap(),
                    );
                }

                self.gen_parse_node(body.as_ref());

                unsafe {
                    LLVMBuildRetVoid(self.builder);
                }
            }
            ParseNode::Block(values, _) => values.iter().for_each(|stmt| self.gen_parse_node(stmt)),
            _ => (),
        };
    }
}
