use conv::ApproxInto;
use dsl_errors::CodeGenError;
use dsl_util::NULL_STR;
use llvm_sys::{
    core::{
        LLVMAppendBasicBlock, LLVMBuildAlloca, LLVMBuildBinOp, LLVMBuildBr, LLVMBuildCondBr,
        LLVMBuildFCmp, LLVMBuildICmp, LLVMBuildInBoundsGEP2, LLVMBuildLoad2, LLVMBuildNeg,
        LLVMBuildPhi, LLVMBuildRetVoid, LLVMBuildStore, LLVMConstInt, LLVMConstReal, LLVMInt64Type,
        LLVMPointerType, LLVMPositionBuilder, LLVMPositionBuilderAtEnd, LLVMAddIncoming,
    },
    prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMValueRef},
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

    pub fn set_position_end(&self, block: LLVMBasicBlockRef) {
        unsafe { LLVMPositionBuilderAtEnd(self.builder, block) }
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
        if_clause: LLVMBasicBlockRef,
        else_clause: LLVMBasicBlockRef,
    ) -> Result<Value, CodeGenError> {
        match condition {
            Value::Literal { llvm_value, .. } => {
                let value =
                    unsafe { LLVMBuildCondBr(self.builder, *llvm_value, if_clause, else_clause) };
                Ok(Value::Instruction { llvm_value: value })
            }
            _ => Err(CodeGenError {
                message: format!("Tryin to branch with bad condition"),
            }),
        }
    }

    pub fn create_branch(&self, branch: LLVMBasicBlockRef) -> Result<Value, CodeGenError> {
        let value = unsafe { LLVMBuildBr(self.builder, branch) };
        Ok(Value::Instruction { llvm_value: value })
    }

    pub fn create_phi(&self, a: &Value, b: &Value, a_block: LLVMBasicBlockRef, b_block: LLVMBasicBlockRef) -> Result<Value, CodeGenError> {
        if a.get_type() != b.get_type() {
            return Err(CodeGenError {
                message: "Incompatible types in branch return!".to_string(),
            });
        }
        let phi_node = unsafe { LLVMBuildPhi(self.builder, a.get_type().get_type(), NULL_STR) };

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
        let mut blocks = [a_block, b_block];
        unsafe {
            LLVMAddIncoming(phi_node, vals.as_mut_ptr(), blocks.as_mut_ptr(), 2)
        }

        Ok(Value::Literal {
            llvm_value: phi_node,
            literal_type: a.get_type().clone(),
        })
    }

    pub fn append_block(&self) -> Result<Value, CodeGenError> {
        unsafe {
            // LLVMAppendBasicBlock(Fn, Name)
        }
        Ok(Value::Empty)
    }

    pub fn create_ret_void(&self) -> Value {
        Value::Instruction {
            llvm_value: unsafe { LLVMBuildRetVoid(self.builder) },
        }
    }
}
