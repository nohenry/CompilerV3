use std::collections::HashMap;
use std::ffi::CString;

use dsl_errors::go;
use llvm_sys::core::{
    LLVMAddFunction, LLVMAppendBasicBlock, LLVMAppendExistingBasicBlock,
    LLVMCreateBasicBlockInContext, LLVMFunctionType, LLVMGetGlobalContext, LLVMInt1Type,
};

use dsl_lexer::ast::{FunctionDecleration, FunctionSignature, ParseNode, VariableDecleration};
use dsl_lexer::TokenKind;
use dsl_util::{cast, NULL_STR};
use llvm_sys::prelude::LLVMTypeRef;

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

                let place_var = if let Some((init, ..)) = possible_initializer {
                    let alloc_block = unsafe {
                        LLVMAppendBasicBlock(
                            self.current_function.borrow().as_mut().unwrap(),
                            NULL_STR,
                        )
                    };
                    let expr = unsafe {
                        LLVMAppendBasicBlock(
                            self.current_function.borrow().as_mut().unwrap(),
                            NULL_STR,
                        )
                    };
                    let end =
                        unsafe { LLVMCreateBasicBlockInContext(LLVMGetGlobalContext(), NULL_STR) };

                    go!(self, self.builder.create_branch(alloc_block), Value);

                    self.builder.set_position_end(expr);

                    self.current_block.replace(expr);
                    let value = self.gen_expression(init.as_ref());
                    go!(self, self.builder.create_branch(end), Value);

                    self.builder.set_position_end(alloc_block);

                    let var = go!(self, self.builder.create_alloc(&value.get_type()), Value);

                    go!(self, self.builder.create_branch(expr), Value);

                    unsafe {
                        LLVMAppendExistingBasicBlock(
                            self.current_function.borrow().as_mut().unwrap(),
                            end,
                        )
                    };
                    self.builder.set_position_end(end);

                    go!(self, self.builder.create_store(&var, &value), Value);

                    var
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
                function_type:
                    FunctionSignature {
                        parameters,
                        return_type,
                        ..
                    },
                generic,
                identifier,
                ..
            }) => {
                let return_type = self.gen_type(return_type);
                let name =
                    CString::new(cast!(&identifier.token_type, TokenKind::Ident).as_str()).unwrap();

                let types: Vec<(String, Type)> = parameters
                    .iter()
                    .map(|f| {
                        (
                            cast!(&f.symbol.token_type, TokenKind::Ident).clone(),
                            self.gen_type(&f.symbol_type),
                        )
                    })
                    .collect();
                let mut llvm_types: Vec<LLVMTypeRef> =
                    types.iter().map(|f| f.1.get_type()).collect();

                unsafe {
                    let function_type = LLVMFunctionType(
                        return_type.get_type(),
                        llvm_types.as_mut_ptr(),
                        llvm_types.len().try_into().unwrap(),
                        0,
                    );

                    let func = LLVMAddFunction(self.module, name.as_ptr(), function_type);

                    let block = LLVMAppendBasicBlock(func, NULL_STR);

                    self.current_block.replace(block);
                    self.builder
                        .set_position_end(self.current_block.borrow().as_mut().unwrap());

                    self.current_function.replace(func);

                    let mut parameters = HashMap::new();
                    for (n, t) in types {
                        parameters.insert(n, t);
                    }

                    let name = name.into_string().unwrap();

                    self.add_and_set_symbol(
                        &name,
                        SymbolValue::Funtion(Value::Function {
                            llvm_value: func,
                            function_type: Type::Function {
                                llvm_type: function_type,
                                parameters,
                                return_type: Box::new(return_type),
                            },
                        }),
                    )
                }

                self.gen_parse_node(body.as_ref());

                self.builder.create_ret_void();
            }

            _ => (),
        };
        Value::Empty
    }
}
