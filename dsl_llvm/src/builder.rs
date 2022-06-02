// pub struct BasicBlock {
//     basic_block: LLVMBasicBlockRef,
// }

// impl BasicBlock {
//     pub fn create(name: &str, context: &Context, function: &Function) -> Self {
//         let basic_block = unsafe {
//             LLVMAppendBasicBlockInContext(
//                 context.context(),
//                 function.function(),
//                 &name.as_bytes()[0] as *const u8,
//             )
//         };
//         BasicBlock { basic_block }
//     }

//     pub fn basic_block(&self) -> LLVMBasicBlockRef {
//         self.basic_block
//     }
// }

// pub struct Builder {
//     builder: LLVMBasicBlockRef,
// }

// impl Builder {
//     pub fn create(context: &Context) -> Self {
//         let builder = unsafe { LLVMCreateBuilderInContext(context.context()) };
//         Builder { builder }
//     }

//     pub fn position_at_end(&self, basic_block: &BasicBlock) {
//         unsafe { LLVMPositionBuilderAtEnd(self.builder, basic_block.basic_block()) }
//     }
// }

use dsl_errors::CodeGenError;
use dsl_util::NULL_STR;
use llvm_sys::{
    core::{
        LLVMBuildAlloca, LLVMBuildBinOp, LLVMBuildInBoundsGEP2, LLVMBuildLoad2, LLVMBuildStore,
        LLVMConstInt, LLVMInt64Type, LLVMPositionBuilder, LLVMPositionBuilderAtEnd, LLVMConstReal,
    },
    prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMValueRef},
    LLVMOpcode,
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

    pub fn create_lteral<T>(&self, ty: &Type, value: T) -> Value
    where
        T: Into<u64> + Into<f64>,
    {
        match ty {
            Type::Integer { llvm_type, signed } => Value::Literal {
                llvm_value: unsafe {
                    LLVMConstInt(*llvm_type, value.into(), if *signed { 1 } else { 0 })
                },
                literal_type: ty.clone(),
            },
            Type::Float { llvm_type } => Value::Literal {
                llvm_value: unsafe {
                    LLVMConstReal(*llvm_type, value.into())
                },
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
                Ok(Value::Load {
                    llvm_value: value,
                    load_type: variable_type.clone(),
                })
            }
            _ => Err(CodeGenError {
                message: format!("Attempted to load a non variable or pointer value"),
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
}
