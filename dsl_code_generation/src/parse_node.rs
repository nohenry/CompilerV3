use std::ffi::CString;

use llvm_sys::core::{
    LLVMAddFunction, LLVMAppendBasicBlock, LLVMBuildAlloca, LLVMBuildLoad2, LLVMBuildRetVoid,
    LLVMBuildStore, LLVMFunctionType, LLVMGetLastInstruction, LLVMInt1Type, LLVMPositionBuilder,
    LLVMPositionBuilderAtEnd,
};

use dsl_lexer::ast::{FunctionDecleration, ParseNode, VariableDecleration};
use dsl_lexer::TokenKind;
use dsl_util::{c_str, cast, NULL_STR};

use super::module::Module;
use dsl_symbol::{SymbolValue, Type, Value};

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
                } else {
                    Type::Integer {
                        llvm_type: unsafe { LLVMInt1Type() },
                        signed: false,
                    }
                };
                let nop = unsafe {
                    LLVMGetLastInstruction(self.current_block.borrow().as_mut().unwrap())
                };

                let (place_var, ty) = if let Some((init, ..)) = possible_initializer {
                    let value = self.gen_expression(init.as_ref());

                    unsafe {
                        LLVMPositionBuilder(
                            self.builder,
                            self.current_block.borrow().as_mut().unwrap(),
                            nop,
                        )
                    }

                    let var = unsafe {
                        LLVMBuildAlloca(self.builder, value.get_type().get_type(), NULL_STR)
                    };

                    unsafe {
                        LLVMPositionBuilderAtEnd(
                            self.builder,
                            self.current_block.borrow().as_mut().unwrap(),
                        )
                    }

                    match value {
                        Value::Literal { llvm_value, .. } => unsafe {
                            LLVMBuildStore(self.builder, llvm_value, var);
                        },
                        Value::Variable {
                            llvm_value,
                            ref variable_type,
                        } => unsafe {
                            let load = LLVMBuildLoad2(
                                self.builder,
                                variable_type.get_type(),
                                llvm_value,
                                NULL_STR,
                            );
                            LLVMBuildStore(self.builder, load, var);
                        },
                        _ => (),
                    }

                    (var, value.get_type().clone())
                } else {
                    let place_var =
                        unsafe { LLVMBuildAlloca(self.builder, ty.get_type(), NULL_STR) };
                    (place_var, ty)
                };

                let name = cast!(&identifier.token_type, TokenKind::Ident);
                self.get_current_mut(|f| {
                    if let Some(sym) = f {
                        let var_value = Value::Variable {
                            llvm_value: place_var,
                            variable_type: ty.clone(),
                        };
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
                        LLVMFunctionType(return_type.get_type(), std::ptr::null_mut(), 0, 0);
                    let func = LLVMAddFunction(self.module, name.as_ptr(), func_type);

                    let block = LLVMAppendBasicBlock(func, c_str!(""));

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
