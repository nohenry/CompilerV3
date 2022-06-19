use std::ffi::CString;

use conv::ApproxInto;
use dsl_errors::CodeGenError;
use dsl_util::NULL_STR;
use linked_hash_map::LinkedHashMap;
use llvm_sys::{
    core::{
        LLVMAddFunction, LLVMAddIncoming, LLVMAppendBasicBlock, LLVMAppendExistingBasicBlock,
        LLVMBuildAlloca, LLVMBuildBinOp, LLVMBuildBr, LLVMBuildCall2, LLVMBuildCondBr,
        LLVMBuildFCmp, LLVMBuildICmp, LLVMBuildInBoundsGEP2, LLVMBuildLoad2, LLVMBuildNeg,
        LLVMBuildPhi, LLVMBuildRet, LLVMBuildRetVoid, LLVMBuildStore, LLVMConstInt, LLVMConstReal,
        LLVMCreateBasicBlockInContext, LLVMFunctionType, LLVMGetGlobalContext, LLVMInt64Type,
        LLVMPointerType, LLVMPositionBuilder, LLVMPositionBuilderAtEnd,
    },
    prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef},
    LLVMIntPredicate, LLVMOpcode, LLVMRealPredicate,
};

use dsl_symbol::{Type, Value};

pub struct IRBuilder {
    builder: LLVMBuilderRef,
}

impl IRBuilder {
    pub fn new(builder: LLVMBuilderRef) -> IRBuilder {
        IRBuilder { builder }
    }

    pub fn get_builder(&self) -> LLVMBuilderRef {
        self.builder
    }

    pub fn set_position(&self, block: LLVMBasicBlockRef, instr: LLVMValueRef) {
        unsafe { LLVMPositionBuilder(self.builder, block, instr) }
    }

    pub fn set_position_end(&self, block: &Value) {
        match block {
            Value::Block { llvm_value } => unsafe {
                LLVMPositionBuilderAtEnd(self.builder, *llvm_value)
            },
            _ => {}
        }
    }

    pub fn get_int_64(&self) -> Type {
        Type::Integer {
            llvm_type: unsafe { LLVMInt64Type() },
            signed: true,
        }
    }

    pub fn get_uint_64(&self) -> Type {
        Type::Integer {
            llvm_type: unsafe { LLVMInt64Type() },
            signed: false,
        }
    }

    pub fn get_ptr(&self, base_type: &Type) -> Type {
        Type::Reference {
            llvm_type: unsafe { LLVMPointerType(base_type.get_type(), 0) },
            base_type: Box::new(base_type.clone()),
        }
    }

    pub fn get_fn(&self, return_type: Type, params: &Vec<(String, Type)>) -> Type {
        let mut llvm_types: Vec<LLVMTypeRef> = params.iter().map(|f| f.1.get_type()).collect();

        let mut parameters = LinkedHashMap::new();
        for (n, t) in params {
            parameters.insert(n.clone(), t.clone());
        }

        Type::Function {
            llvm_type: unsafe {
                LLVMFunctionType(
                    return_type.get_type(),
                    llvm_types.as_mut_ptr(),
                    llvm_types.len().try_into().unwrap(),
                    0,
                )
            },

            parameters,
            return_type: Box::new(return_type),
        }
    }

    pub fn create_literal<T>(&self, ty: &Type, value: T) -> Value
    where
        T: ApproxInto<u64> + ApproxInto<f64>,
    {
        match ty {
            Type::Integer { llvm_type, signed } => Value::Literal {
                llvm_value: unsafe {
                    LLVMConstInt(
                        *llvm_type,
                        value.approx_into().unwrap(),
                        if *signed { 1 } else { 0 },
                    )
                },
                literal_type: ty.clone(),
            },
            Type::Float { llvm_type } => Value::Literal {
                llvm_value: unsafe { LLVMConstReal(*llvm_type, value.approx_into().unwrap()) },
                literal_type: ty.clone(),
            },
            _ => panic!("Unknown literal"),
        }
    }

    pub fn create_alloc(&self, ty: &Type) -> Result<Value, CodeGenError> {
        let value = unsafe { LLVMBuildAlloca(self.builder, ty.get_type(), NULL_STR) };
        Ok(Value::Variable {
            llvm_value: value,
            variable_type: ty.clone(),
        })
    }

    pub fn create_store_raw_val(
        &self,
        ptr: &Value,
        value: LLVMValueRef,
    ) -> Result<Value, CodeGenError> {
        match ptr {
            Value::Variable {
                llvm_value: ptr_value,
                ..
            } => {
                let llvm_value = unsafe { LLVMBuildStore(self.builder, value, *ptr_value) };
                Ok(Value::Instruction { llvm_value })
            }
            _ => Err(CodeGenError {
                message: format!(
                    "Attempted to create store on a value without a location (not a variable)"
                ),
            }),
        }
    }

    pub fn create_store(&self, ptr: &Value, value: &Value) -> Result<Value, CodeGenError> {
        match ptr {
            Value::Variable {
                llvm_value: ptr_value,
                ..
            } => match value {
                Value::Literal {
                    llvm_value: rvalue, ..
                } => {
                    let llvm_value = unsafe { LLVMBuildStore(self.builder, *rvalue, *ptr_value) };
                    Ok(Value::Instruction { llvm_value })
                }
                Value::Variable {
                    llvm_value,
                    variable_type,
                } => {
                    let load = unsafe {
                        LLVMBuildLoad2(
                            self.builder,
                            variable_type.get_type(),
                            *llvm_value,
                            NULL_STR,
                        )
                    };

                    let llvm_value = unsafe { LLVMBuildStore(self.builder, load, *ptr_value) };
                    Ok(Value::Instruction { llvm_value })
                }
                Value::Function { llvm_value, .. } => {
                    let llvm_value =
                        unsafe { LLVMBuildStore(self.builder, *llvm_value, *ptr_value) };
                    Ok(Value::Instruction { llvm_value })
                }
                _ => Err(CodeGenError {
                    message: format!("Unable to store value"),
                }),
            },
            _ => Err(CodeGenError {
                message: format!(
                    "Attempted to create store on a value without a location (not a variable)"
                ),
            }),
        }
    }

    pub fn create_load(&self, ptr: &Value) -> Result<Value, CodeGenError> {
        match ptr {
            Value::Variable {
                llvm_value: ptr_value,
                variable_type,
            } => {
                let value = unsafe {
                    LLVMBuildLoad2(self.builder, variable_type.get_type(), *ptr_value, NULL_STR)
                };
                Ok(Value::Literal {
                    llvm_value: value,
                    literal_type: variable_type.clone(),
                })
            }
            _ => Err(CodeGenError {
                message: format!("Attempted to load a non variable or pointer value"),
            }),
        }
    }

    /**
     * Operations
     */

    pub fn create_neg(&self, value: &Value) -> Result<Value, CodeGenError> {
        match value {
            Value::Variable {
                llvm_value,
                variable_type,
            } => {
                let value = unsafe {
                    LLVMBuildLoad2(
                        self.builder,
                        variable_type.get_type(),
                        *llvm_value,
                        NULL_STR,
                    )
                };
                let value = unsafe { LLVMBuildNeg(self.builder, value, NULL_STR) };
                Ok(Value::Literal {
                    llvm_value: value,
                    literal_type: variable_type.clone(),
                })
            }
            _ => Err(CodeGenError {
                message: format!("Unsupported value for negative"),
            }),
        }
    }

    pub fn create_bin_op(
        &self,
        left: &Value,
        right: &Value,
        op: LLVMOpcode,
    ) -> Result<Value, CodeGenError> {
        match (left, right) {
            (
                Value::Variable {
                    llvm_value: lvalue,
                    variable_type,
                },
                Value::Literal {
                    llvm_value: rvalue, ..
                },
            ) => {
                let lvalue = unsafe {
                    LLVMBuildLoad2(self.builder, variable_type.get_type(), *lvalue, NULL_STR)
                };
                let llvm_value =
                    unsafe { LLVMBuildBinOp(self.builder, op, lvalue, *rvalue, NULL_STR) };
                Ok(Value::Literal {
                    llvm_value,
                    literal_type: variable_type.clone(),
                })
            }
            (
                Value::Variable {
                    llvm_value: lvalue,
                    variable_type: ltype,
                },
                Value::Variable {
                    llvm_value: rvalue,
                    variable_type: rtype,
                },
            ) => {
                let lvalue =
                    unsafe { LLVMBuildLoad2(self.builder, ltype.get_type(), *lvalue, NULL_STR) };
                let rvalue =
                    unsafe { LLVMBuildLoad2(self.builder, rtype.get_type(), *rvalue, NULL_STR) };

                let llvm_value =
                    unsafe { LLVMBuildBinOp(self.builder, op, lvalue, rvalue, NULL_STR) };

                Ok(Value::Literal {
                    llvm_value,
                    literal_type: ltype.clone(),
                })
            }
            _ => Err(CodeGenError {
                message: format!("Unsupported operands for binary expression"),
            }),
        }
    }

    pub fn create_icompare(
        &self,
        left: &Value,
        right: &Value,
        op: LLVMIntPredicate,
    ) -> Result<Value, CodeGenError> {
        match (left, right) {
            (
                Value::Literal {
                    llvm_value: lvalue,
                    literal_type,
                },
                Value::Literal {
                    llvm_value: rvalue, ..
                },
            ) => {
                let llvm_value =
                    unsafe { LLVMBuildICmp(self.builder, op, *lvalue, *rvalue, NULL_STR) };
                Ok(Value::Literal {
                    llvm_value,
                    literal_type: literal_type.clone(),
                })
            }
            (
                Value::Variable {
                    llvm_value: lvalue,
                    variable_type,
                },
                Value::Literal {
                    llvm_value: rvalue, ..
                },
            ) => {
                let lvalue = unsafe {
                    LLVMBuildLoad2(self.builder, variable_type.get_type(), *lvalue, NULL_STR)
                };
                let llvm_value =
                    unsafe { LLVMBuildICmp(self.builder, op, lvalue, *rvalue, NULL_STR) };
                Ok(Value::Literal {
                    llvm_value,
                    literal_type: variable_type.clone(),
                })
            }
            (
                Value::Variable {
                    llvm_value: lvalue,
                    variable_type: ltype,
                },
                Value::Variable {
                    llvm_value: rvalue,
                    variable_type: rtype,
                },
            ) => {
                let lvalue =
                    unsafe { LLVMBuildLoad2(self.builder, ltype.get_type(), *lvalue, NULL_STR) };
                let rvalue =
                    unsafe { LLVMBuildLoad2(self.builder, rtype.get_type(), *rvalue, NULL_STR) };

                let llvm_value =
                    unsafe { LLVMBuildICmp(self.builder, op, lvalue, rvalue, NULL_STR) };

                Ok(Value::Literal {
                    llvm_value,
                    literal_type: ltype.clone(),
                })
            }
            _ => Err(CodeGenError {
                message: format!("Unsupported operands for binary expression"),
            }),
        }
    }

    pub fn create_fcompare(
        &self,
        left: &Value,
        right: &Value,
        op: LLVMRealPredicate,
    ) -> Result<Value, CodeGenError> {
        match (left, right) {
            (
                Value::Variable {
                    llvm_value: lvalue,
                    variable_type,
                },
                Value::Literal {
                    llvm_value: rvalue, ..
                },
            ) => {
                let lvalue = unsafe {
                    LLVMBuildLoad2(self.builder, variable_type.get_type(), *lvalue, NULL_STR)
                };
                let llvm_value =
                    unsafe { LLVMBuildFCmp(self.builder, op, lvalue, *rvalue, NULL_STR) };
                Ok(Value::Literal {
                    llvm_value,
                    literal_type: variable_type.clone(),
                })
            }
            (
                Value::Variable {
                    llvm_value: lvalue,
                    variable_type: ltype,
                },
                Value::Variable {
                    llvm_value: rvalue,
                    variable_type: rtype,
                },
            ) => {
                let lvalue =
                    unsafe { LLVMBuildLoad2(self.builder, ltype.get_type(), *lvalue, NULL_STR) };
                let rvalue =
                    unsafe { LLVMBuildLoad2(self.builder, rtype.get_type(), *rvalue, NULL_STR) };

                let llvm_value =
                    unsafe { LLVMBuildFCmp(self.builder, op, lvalue, rvalue, NULL_STR) };

                Ok(Value::Literal {
                    llvm_value,
                    literal_type: ltype.clone(),
                })
            }
            _ => Err(CodeGenError {
                message: format!("Unsupported operands for binary expression"),
            }),
        }
    }

    pub fn create_gep_inbound(
        &self,
        ptr: &Value,
        base_type: Type,
        indicies: &[Value],
    ) -> Result<Value, CodeGenError> {
        match ptr {
            Value::Variable {
                llvm_value,
                variable_type,
            } => {
                let mut ind: Vec<_> = indicies
                    .iter()
                    .filter_map(|i| match i {
                        Value::Variable { llvm_value, .. } => Some(*llvm_value),
                        Value::Literal { llvm_value, .. } => Some(*llvm_value),
                        _ => None,
                    })
                    .collect();

                let value = unsafe {
                    LLVMBuildInBoundsGEP2(
                        self.builder,
                        variable_type.get_type(),
                        *llvm_value,
                        ind.as_mut_ptr(),
                        ind.len() as _,
                        NULL_STR,
                    )
                };
                Ok(Value::Variable {
                    llvm_value: value,
                    variable_type: base_type,
                })
            }
            _ => Err(CodeGenError {
                message: format!("Trying to index non pointer like value"),
            }),
        }
    }

    pub fn create_cbranch(
        &self,
        condition: &Value,
        if_clause: &Value,
        else_clause: &Value,
    ) -> Result<Value, CodeGenError> {
        match (if_clause, else_clause) {
            (
                Value::Block {
                    llvm_value: if_clause,
                },
                Value::Block {
                    llvm_value: else_clause,
                },
            ) => match condition {
                Value::Literal { llvm_value, .. } => {
                    let value = unsafe {
                        LLVMBuildCondBr(self.builder, *llvm_value, *if_clause, *else_clause)
                    };
                    Ok(Value::Instruction { llvm_value: value })
                }
                _ => Err(CodeGenError {
                    message: format!("Trying to branch with bad condition"),
                }),
            },
            _ => Err(CodeGenError {
                message: format!("Trying to branch with bad bodies"),
            }),
        }
    }

    pub fn create_branch(&self, branch: &Value) -> Result<Value, CodeGenError> {
        match branch {
            Value::Block { llvm_value } => {
                let value = unsafe { LLVMBuildBr(self.builder, *llvm_value) };
                Ok(Value::Instruction { llvm_value: value })
            }
            _ => Err(CodeGenError {
                message: format!("Tying to branch to non block value"),
            }),
        }
    }

    pub fn create_fn_call(&self, value: &Value, args: Vec<Value>) -> Result<Value, CodeGenError> {
        match value {
            Value::Function {
                llvm_value,
                function_type:
                    Type::Function {
                        return_type,
                        llvm_type,
                        parameters,
                    },
            }
            | Value::Variable {
                llvm_value,
                variable_type:
                    Type::Function {
                        return_type,
                        llvm_type,
                        parameters,
                    },
            } => unsafe {
                if args.len() < parameters.len() {
                    let aa: String = parameters
                        .keys()
                        .rev()
                        .take(parameters.len() - args.len())
                        .cloned()
                        .intersperse(", ".to_string())
                        .collect();

                    return Err(CodeGenError {
                        message: format!("Expected arguments `{}` in call!", aa),
                    });
                } else if args.len() > parameters.len() {
                    return Err(CodeGenError {
                        message: format!("Extra arguments in function call!",),
                    });
                }

                let res: Result<Vec<Value>, _> = parameters
                    .iter()
                    .zip(args)
                    .map(|((name, ty), val)| {
                        let nty = val.weak_cast(ty, self.builder);
                        match nty {
                            Err(true) => {
                                return Err(CodeGenError {
                                    message: format!(
                                        "Function argument doesn't match type of parameter `{}`",
                                        name
                                    ),
                                })
                            }
                            Err(false) => Ok(val),
                            Ok(v) => Ok(v),
                        }
                    })
                    .collect();
                let args = res?;

                let mut fargs: Result<Vec<LLVMValueRef>, ()> =
                    args.iter().map(|f| f.get_value(self.builder)).collect();
                let fargs = if let Ok(args) = &mut fargs {
                    args
                } else {
                    return Err(CodeGenError {
                        message: format!("Unable to resolve function call parameters"),
                    });
                };

                Ok(Value::Literal {
                    llvm_value: LLVMBuildCall2(
                        self.builder,
                        *llvm_type,
                        *llvm_value,
                        fargs.as_mut_ptr(),
                        fargs.len().try_into().unwrap(),
                        NULL_STR,
                    ),
                    literal_type: *(*return_type).clone(),
                })
            },
            _ => Err(CodeGenError {
                message: format!("Attempted to call non function value!"),
            }),
        }
    }

    pub fn create_phi(
        &self,
        a: &Value,
        b: &Value,
        a_block: &Value,
        b_block: &Value,
    ) -> Result<Value, CodeGenError> {
        match (a_block, b_block) {
            (
                Value::Block {
                    llvm_value: a_block,
                },
                Value::Block {
                    llvm_value: b_block,
                },
            ) => {
                if a.get_type() != b.get_type() {
                    return Err(CodeGenError {
                        message: "Incompatible types in branch return!".to_string(),
                    });
                }
                let phi_node =
                    unsafe { LLVMBuildPhi(self.builder, a.get_type().get_type(), NULL_STR) };

                let a_node = match &a {
                    Value::Variable {
                        llvm_value,
                        variable_type,
                    } => unsafe {
                        LLVMBuildLoad2(
                            self.builder,
                            variable_type.get_type(),
                            *llvm_value,
                            NULL_STR,
                        )
                    },
                    Value::Literal { llvm_value, .. } => *llvm_value,
                    Value::Instruction { llvm_value } => *llvm_value,
                    Value::Load { llvm_value, .. } => *llvm_value,
                    _ => {
                        return Err(CodeGenError {
                            message: "Unable to return value from branch".to_string(),
                        })
                    }
                };

                let b_node = match &b {
                    Value::Variable {
                        llvm_value,
                        variable_type,
                    } => unsafe {
                        LLVMBuildLoad2(
                            self.builder,
                            variable_type.get_type(),
                            *llvm_value,
                            NULL_STR,
                        )
                    },
                    Value::Literal { llvm_value, .. } => *llvm_value,
                    Value::Instruction { llvm_value } => *llvm_value,
                    Value::Load { llvm_value, .. } => *llvm_value,
                    _ => {
                        return Err(CodeGenError {
                            message: "Unable to return value from branch".to_string(),
                        })
                    }
                };

                let mut vals = [a_node, b_node];
                let mut blocks = [*a_block, *b_block];
                unsafe { LLVMAddIncoming(phi_node, vals.as_mut_ptr(), blocks.as_mut_ptr(), 2) }

                Ok(Value::Literal {
                    llvm_value: phi_node,
                    literal_type: a.get_type().clone(),
                })
            }
            _ => {
                return Err(CodeGenError {
                    message: "Unable to return value from branch".to_string(),
                })
            }
        }
    }

    pub fn append_block(&self, func: &Value) -> Result<Value, CodeGenError> {
        match func {
            Value::Function { llvm_value, .. } => Ok(Value::Block {
                llvm_value: unsafe { LLVMAppendBasicBlock(*llvm_value, NULL_STR) },
            }),
            _ => {
                return Err(CodeGenError {
                    message: "Expected function type to add function to module!".to_string(),
                })
            }
        }
    }

    pub fn append_existing_block(&self, func: &Value, block: &Value) -> Result<(), CodeGenError> {
        match (func, block) {
            (
                Value::Function {
                    llvm_value: func, ..
                },
                Value::Block { llvm_value: block },
            ) => {
                unsafe {
                    LLVMAppendExistingBasicBlock(*func, *block);
                }
                Ok(())
            }
            _ => {
                return Err(CodeGenError {
                    message: "Expected function type to add function to module!".to_string(),
                })
            }
        }
    }

    pub fn create_block(&self) -> Result<Value, CodeGenError> {
        Ok(Value::Block {
            llvm_value: unsafe { LLVMCreateBasicBlockInContext(LLVMGetGlobalContext(), NULL_STR) },
        })
    }

    pub fn create_ret_void(&self) -> Value {
        Value::Instruction {
            llvm_value: unsafe { LLVMBuildRetVoid(self.builder) },
        }
    }

    pub fn create_ret(&self, value: &Value) -> Result<Value, CodeGenError> {
        match value {
            Value::Literal { llvm_value, .. } => Ok(Value::Instruction {
                llvm_value: unsafe { LLVMBuildRet(self.builder, *llvm_value) },
            }),
            Value::Variable {
                llvm_value,
                variable_type,
            } => {
                let ld = unsafe {
                    LLVMBuildLoad2(
                        self.builder,
                        variable_type.get_type(),
                        *llvm_value,
                        NULL_STR,
                    )
                };
                let ret = unsafe { LLVMBuildRet(self.builder, ld) };
                Ok(Value::Instruction { llvm_value: ret })
            }
            _ => {
                return Err(CodeGenError {
                    message: "".to_string(),
                })
            }
        }
    }

    pub fn add_function(
        &self,
        ty: Type,
        name: String,
        module: LLVMModuleRef,
    ) -> Result<Value, CodeGenError> {
        let name = CString::new(name).unwrap();
        match &ty {
            Type::Function { llvm_type, .. } => unsafe {
                Ok(Value::Function {
                    llvm_value: LLVMAddFunction(module, name.as_ptr(), *llvm_type),
                    function_type: ty,
                })
            },
            _ => {
                return Err(CodeGenError {
                    message: "Expected function type to add function to module!".to_string(),
                })
            }
        }
    }
}
