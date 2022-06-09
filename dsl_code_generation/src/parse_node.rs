use std::cell::RefCell;
use std::ffi::CString;

use dsl_errors::go;
use llvm_sys::core::{
    LLVMAddFunction, LLVMAppendBasicBlock, LLVMFunctionType, LLVMGetLastInstruction, LLVMInt1Type,
};

use dsl_lexer::ast::{Expression, FunctionDecleration, ParseNode, VariableDecleration};
use dsl_lexer::TokenKind;
use dsl_util::{c_str, cast};

use super::module::Module;
use dsl_symbol::{SymbolValue, Type, Value};

impl Module {
    pub(super) fn gen_parse_node(&self, node: &ParseNode) -> Value {
        match node {
            ParseNode::Expression(e, _) => {
                return self.gen_expression(e);
            }
            ParseNode::VariableDecleration(VariableDecleration {
                identifier,
                possible_initializer,
                variable_type,
                ..
            }) => {
                let ty = if let Some(ty) = variable_type {
                    self.gen_type(ty.as_ref())
                } else {
                    Type::Integer {
                        llvm_type: unsafe { LLVMInt1Type() },
                        signed: false,
                    }
                };
                let nop = unsafe {
                    LLVMGetLastInstruction(self.current_block.borrow().as_mut().unwrap())
                };

                let place_var = if let Some((init, ..)) = possible_initializer {
                    match &**init {
                        Expression::IfExpression(_) => {
                            let place_var = go!(self, self.builder.create_alloc(&ty), Value);
                            self.current_storage.replace(place_var);
                            self.gen_expression(init.as_ref());
                            let val = self.current_storage.borrow().to_owned();
                            self.current_storage.replace(Value::Empty);

                            val
                        }
                        _ => {
                            let value = self.gen_expression(init.as_ref());

                            self.builder.set_position(
                                unsafe { self.current_block.borrow().as_mut().unwrap() },
                                nop,
                            );

                            let var =
                                go!(self, self.builder.create_alloc(&value.get_type()), Value);

                            self.builder.set_position_end(unsafe {
                                self.current_block.borrow().as_mut().unwrap()
                            });

                            go!(self, self.builder.create_store(&var, &value), Value);
                            var
                        }
                    }
                } else {
                    let place_var = go!(self, self.builder.create_alloc(&ty), Value);
                    place_var
                };

                let name = cast!(&identifier.token_type, TokenKind::Ident);
                self.get_current_mut(|f| {
                    if let Some(sym) = f {
                        sym.add_child(name, SymbolValue::Variable(place_var.clone()));
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
                        LLVMFunctionType(return_type.get_type(), std::ptr::null_mut(), 0, 0);
                    let func = LLVMAddFunction(self.module, name.as_ptr(), func_type);

                    let block = LLVMAppendBasicBlock(func, c_str!(""));

                    self.current_block.replace(block);
                    self.builder
                        .set_position_end(self.current_block.borrow().as_mut().unwrap());

                    self.current_function.replace(func);
                }

                self.gen_parse_node(body.as_ref());

                self.builder.create_ret_void();
            }

            _ => (),
        };
        Value::Empty
    }
}
