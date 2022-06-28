use std::ffi::CString;

use conv::ApproxInto;
use dsl_errors::CodeGenError;
use dsl_util::NULL_STR;
use linked_hash_map::LinkedHashMap;
use llvm_sys::{
    core::{
        LLVMAddFunction, LLVMAddIncoming, LLVMAppendBasicBlock, LLVMAppendExistingBasicBlock,
        LLVMBuildAlloca, LLVMBuildBinOp, LLVMBuildBitCast, LLVMBuildBr, LLVMBuildCall2,
        LLVMBuildCondBr, LLVMBuildFCmp, LLVMBuildICmp, LLVMBuildInBoundsGEP2, LLVMBuildLoad2,
        LLVMBuildMemCpy, LLVMBuildNeg, LLVMBuildPhi, LLVMBuildRet, LLVMBuildRetVoid,
        LLVMBuildSelect, LLVMBuildStore, LLVMBuildStructGEP2, LLVMConstInt, LLVMConstReal,
        LLVMCreateBasicBlockInContext, LLVMDoubleType, LLVMFloatType, LLVMFunctionType,
        LLVMGetGlobalContext, LLVMInt16Type, LLVMInt1Type, LLVMInt32Type, LLVMInt64Type,
        LLVMInt8Type, LLVMIntType, LLVMIsConstant, LLVMPointerType, LLVMPositionBuilder,
        LLVMPositionBuilderAtEnd, LLVMStructCreateNamed, LLVMStructSetBody, LLVMVoidType,
    },
    prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef},
    target::LLVMGetModuleDataLayout,
    LLVMBuildAlignedLoad, LLVMIntPredicate, LLVMOpcode, LLVMRealPredicate, LLVMSetAllocatedType,
};

use dsl_symbol::{Type, Value};

use crate::target_data::TargetData;

pub struct IRBuilder {
    builder: LLVMBuilderRef,
    module: LLVMModuleRef,
}

impl IRBuilder {
    pub fn new(builder: LLVMBuilderRef, module: LLVMModuleRef) -> IRBuilder {
        IRBuilder { builder, module }
    }

    pub fn get_data_layout(&self) -> TargetData {
        TargetData::new(unsafe { LLVMGetModuleDataLayout(self.module) })
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

    pub fn get_uint(width: u32) -> Type {
        Type::Integer {
            llvm_type: unsafe { LLVMIntType(width) },
            signed: false,
        }
    }

    pub fn get_uint_8() -> Type {
        Type::Integer {
            llvm_type: unsafe { LLVMInt8Type() },
            signed: false,
        }
    }

    pub fn get_uint_16() -> Type {
        Type::Integer {
            llvm_type: unsafe { LLVMInt16Type() },
            signed: false,
        }
    }

    pub fn get_uint_32() -> Type {
        Type::Integer {
            llvm_type: unsafe { LLVMInt32Type() },
            signed: false,
        }
    }

    pub fn get_uint_64() -> Type {
        Type::Integer {
            llvm_type: unsafe { LLVMInt64Type() },
            signed: false,
        }
    }

    pub fn get_int_8() -> Type {
        Type::Integer {
            llvm_type: unsafe { LLVMInt8Type() },
            signed: true,
        }
    }

    pub fn get_int_16() -> Type {
        Type::Integer {
            llvm_type: unsafe { LLVMInt16Type() },
            signed: true,
        }
    }

    pub fn get_int_32() -> Type {
        Type::Integer {
            llvm_type: unsafe { LLVMInt32Type() },
            signed: true,
        }
    }

    pub fn get_int_64() -> Type {
        Type::Integer {
            llvm_type: unsafe { LLVMInt64Type() },
            signed: true,
        }
    }

    pub fn get_int(width: u32) -> Type {
        Type::Integer {
            llvm_type: unsafe { LLVMIntType(width) },
            signed: false,
        }
    }

    pub fn get_float32() -> Type {
        Type::Float {
            llvm_type: unsafe { LLVMFloatType() },
        }
    }

    pub fn get_float64() -> Type {
        Type::Float {
            llvm_type: unsafe { LLVMDoubleType() },
        }
    }

    pub fn get_bool() -> Type {
        Type::Boolean {
            llvm_type: unsafe { LLVMInt1Type() },
        }
    }

    pub fn get_unit() -> Type {
        Type::Unit {
            llvm_type: unsafe { LLVMVoidType() },
        }
    }

    pub fn get_ptr(base_type: &Type, constant: bool) -> Type {
        Type::Reference {
            llvm_type: unsafe { LLVMPointerType(base_type.get_type(), 0) },
            base_type: Box::new(base_type.clone()),
            constant,
        }
    }

    pub fn get_fn(return_type: Type, params: &Vec<(String, Type)>) -> Type {
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

    pub fn create_literal<T>(ty: &Type, value: T) -> Value
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

    pub fn create_alloc(&self, ty: &Type, constant: bool) -> Result<Value, CodeGenError> {
        let value = unsafe { LLVMBuildAlloca(self.builder, ty.get_type(), NULL_STR) };
        Ok(Value::Variable {
            llvm_value: value,
            variable_type: ty.clone(),
            constant,
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

    pub fn create_store(
        &self,
        ptr: &Value,
        value: &Value,
        module: LLVMModuleRef,
    ) -> Result<Value, CodeGenError> {
        match ptr {
            Value::Variable {
                llvm_value: ptr_value,
                variable_type,
                constant,
            } => match value {
                Value::Literal {
                    llvm_value: rvalue, ..
                } => {
                    let llvm_value = unsafe { LLVMBuildStore(self.builder, *rvalue, *ptr_value) };
                    Ok(Value::Instruction { llvm_value })
                }
                var @ Value::Variable {
                    llvm_value,
                    variable_type,
                    constant,
                } => {
                    let load = self.create_aligned_load(var)?.get_raw_value()?;

                    let llvm_value = unsafe { LLVMBuildStore(self.builder, load, *ptr_value) };
                    Ok(Value::Instruction { llvm_value })
                }
                Value::Function { llvm_value, .. } => {
                    let llvm_value =
                        unsafe { LLVMBuildStore(self.builder, *llvm_value, *ptr_value) };
                    Ok(Value::Instruction { llvm_value })
                }
                template @ Value::Template { .. } => {
                    let vcal = self.create_bitcast(
                        &template,
                        &IRBuilder::get_ptr(&IRBuilder::get_uint_8(), *constant),
                    )?;
                    self.create_memcpy(ptr, &vcal, module)
                }
                Value::TemplateFields {
                    fields: init_fields,
                    ..
                } => match variable_type {
                    Type::Template { fields, .. } => {
                        for field in init_fields {
                            let pos = fields.keys().position(|f| f == field.0).unwrap();

                            let ptr = self.create_struct_gep(
                                ptr,
                                field.1.get_type().clone(),
                                pos.try_into().unwrap(),
                            )?;

                            self.create_store(&ptr, &field.1, module)?;
                        }
                        Ok(Value::Empty)
                    }
                    _ => Err(CodeGenError {
                        message: "Mismatched types in template initializer".into(),
                    }),
                },
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
                constant,
            } => {
                let align = self.get_data_layout().get_preferred_align(variable_type);
                let value = unsafe {
                    LLVMBuildAlignedLoad(
                        self.builder,
                        variable_type.get_type(),
                        *ptr_value,
                        align,
                        NULL_STR,
                    )
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
            var @ Value::Variable {
                llvm_value,
                variable_type,
                constant,
            } => {
                let value = self.create_aligned_load(var)?.get_raw_value()?;
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

    pub fn create_aligned_load(&self, val: &Value) -> Result<Value, CodeGenError> {
        let align = self.get_data_layout().get_preferred_align(val.get_type());
        let value = unsafe {
            LLVMBuildAlignedLoad(
                self.builder,
                val.get_type().get_type(),
                val.get_raw_value()?,
                align,
                NULL_STR,
            )
        };
        Ok(Value::Load {
            llvm_value: value,
            load_type: val.get_type().clone(),
            constant: val.is_const(),
        })
    }

    pub fn create_bin_op(
        &self,
        left: &Value,
        right: &Value,
        op: LLVMOpcode,
    ) -> Result<Value, CodeGenError> {
        match (left, right) {
            (
                var @ Value::Variable {
                    llvm_value: lvalue,
                    variable_type,
                    constant,
                },
                Value::Literal {
                    llvm_value: rvalue, ..
                },
            ) => {
                let lvalue = self.create_aligned_load(var)?.get_raw_value()?;
                let llvm_value =
                    unsafe { LLVMBuildBinOp(self.builder, op, lvalue, *rvalue, NULL_STR) };
                Ok(Value::Literal {
                    llvm_value,
                    literal_type: variable_type.clone(),
                })
            }
            (
                lvar @ Value::Variable {
                    llvm_value: lvalue,
                    variable_type: ltype,
                    ..
                },
                rvar @ Value::Variable {
                    llvm_value: rvalue,
                    variable_type: rtype,
                    ..
                },
            ) => {
                let lvalue = self.create_aligned_load(lvar)?.get_raw_value()?;
                let rvalue = self.create_aligned_load(rvar)?.get_raw_value()?;

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
                var @ Value::Variable {
                    llvm_value: lvalue,
                    variable_type,
                    ..
                },
                Value::Literal {
                    llvm_value: rvalue, ..
                },
            ) => {
                let lvalue = self.create_aligned_load(var)?.get_raw_value()?;
                let llvm_value =
                    unsafe { LLVMBuildICmp(self.builder, op, lvalue, *rvalue, NULL_STR) };
                Ok(Value::Literal {
                    llvm_value,
                    literal_type: variable_type.clone(),
                })
            }
            (
                lvar @ Value::Variable {
                    llvm_value: lvalue,
                    variable_type: ltype,
                    ..
                },
                rvar @ Value::Variable {
                    llvm_value: rvalue,
                    variable_type: rtype,
                    ..
                },
            ) => {
                let lvalue = self.create_aligned_load(lvar)?.get_raw_value()?;
                let rvalue = self.create_aligned_load(rvar)?.get_raw_value()?;

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
                lvar @ Value::Variable {
                    llvm_value: lvalue,
                    variable_type,
                    ..
                },
                Value::Literal {
                    llvm_value: rvalue, ..
                },
            ) => {
                let lvalue = self.create_aligned_load(lvar)?.get_raw_value()?;
                let llvm_value =
                    unsafe { LLVMBuildFCmp(self.builder, op, lvalue, *rvalue, NULL_STR) };
                Ok(Value::Literal {
                    llvm_value,
                    literal_type: variable_type.clone(),
                })
            }
            (
                lvar @ Value::Variable {
                    llvm_value: lvalue,
                    variable_type: ltype,
                    ..
                },
                rvar @ Value::Variable {
                    llvm_value: rvalue,
                    variable_type: rtype,
                    ..
                },
            ) => {
                let lvalue = self.create_aligned_load(lvar)?.get_raw_value()?;
                let rvalue = self.create_aligned_load(rvar)?.get_raw_value()?;

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
                constant,
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
                    constant: *constant,
                })
            }
            _ => Err(CodeGenError {
                message: format!("Trying to index non pointer like value"),
            }),
        }
    }

    pub fn create_struct_gep(
        &self,
        ptr: &Value,
        base_type: Type,
        index: u32,
    ) -> Result<Value, CodeGenError> {
        let ptr = ptr.resolve_base_value(self.builder);
        println!("{}", ptr.llvm_string());
        println!("{}", ptr.get_type().llvm_string());
        match ptr {
            Value::Load {
                llvm_value,
                load_type,
                constant,
            } => {
                let value = unsafe {
                    LLVMBuildStructGEP2(
                        self.builder,
                        load_type.resolve_base_type().get_type(),
                        llvm_value,
                        index,
                        NULL_STR,
                    )
                };
                Ok(Value::Variable {
                    llvm_value: value,
                    variable_type: base_type,
                    constant,
                })
            }
            Value::Variable {
                llvm_value,
                variable_type,
                constant,
            } => {
                let value = unsafe {
                    LLVMBuildStructGEP2(
                        self.builder,
                        variable_type.get_type(),
                        llvm_value,
                        index,
                        NULL_STR,
                    )
                };
                Ok(Value::Variable {
                    llvm_value: value,
                    variable_type: base_type,
                    constant,
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

    pub fn create_fn_call(
        &self,
        value: &Value,
        mut args: Vec<Value>,
    ) -> Result<Value, CodeGenError> {
        fn gen(
            args: &Vec<Value>,
            parameters: &LinkedHashMap<String, Type>,
            return_type: &Type,
            llvm_type: LLVMTypeRef,
            llvm_value: LLVMValueRef,
            builder: LLVMBuilderRef,
            module: LLVMModuleRef,
        ) -> Result<Value, CodeGenError> {
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
                    let nty = val.weak_cast(ty, builder);
                    match nty {
                        Err(true) => {
                            return Err(CodeGenError {
                                message: format!(
                                    "Function argument doesn't match type of parameter `{}`",
                                    name
                                ),
                            })
                        }
                        Err(false) => Ok(val.clone()),
                        Ok(v) => Ok(v),
                    }
                })
                .collect();
            let args = res?;

            let fargs: Result<Vec<LLVMValueRef>, CodeGenError> =
                args.iter().map(|f| f.get_value(builder, module)).collect();
            let mut fargs = fargs?;

            Ok(Value::Literal {
                llvm_value: unsafe {
                    LLVMBuildCall2(
                        builder,
                        llvm_type,
                        llvm_value,
                        fargs.as_mut_ptr(),
                        fargs.len().try_into().unwrap(),
                        NULL_STR,
                    )
                },
                literal_type: return_type.clone(),
            })
        }
        match value {
            Value::MemberFunction {
                func:
                    box Value::Function {
                        llvm_value,
                        function_type:
                            Type::Function {
                                return_type,
                                llvm_type,
                                parameters,
                            },
                    },
                var,
            } => {
                args.push(var.get_ptr(self.module));
                gen(
                    &args,
                    parameters,
                    return_type,
                    *llvm_type,
                    *llvm_value,
                    self.builder,
                    self.module,
                )
            }
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
                ..
            } => gen(
                &args,
                parameters,
                return_type,
                *llvm_type,
                *llvm_value,
                self.builder,
                self.module,
            ),
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
                let val = a.weak_cast(b.get_type(), self.builder);
                let if_wval = match &val {
                    Ok(v) => Some(v),
                    Err(false) => Some(a),
                    Err(true) => None,
                };
                let val = b.weak_cast(a.get_type(), self.builder);
                let (a, b) = if if_wval.is_none() {
                    let else_wval = match &val {
                        Ok(v) => v,
                        Err(false) => b,
                        Err(true) => {
                            return Err(CodeGenError {
                                message: "Types do not match in if statement!".into(),
                            });
                        }
                    };
                    (a, else_wval)
                } else {
                    (if_wval.unwrap(), b)
                };

                let phi_node =
                    unsafe { LLVMBuildPhi(self.builder, a.get_type().get_type(), NULL_STR) };

                let a_node = match &a {
                    var @ Value::Variable {
                        llvm_value,
                        variable_type,
                        ..
                    } => self.create_aligned_load(var)?.get_raw_value()?,
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
                    var @ Value::Variable {
                        llvm_value,
                        variable_type,
                        ..
                    } => self.create_aligned_load(var)?.get_raw_value()?,
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

    pub fn create_select(
        &self,
        cond: &Value,
        if_val: &Value,
        else_val: &Value,
    ) -> Result<Value, CodeGenError> {
        let val = if_val.weak_cast(else_val.get_type(), self.builder);
        let if_wval = match &val {
            Ok(v) => Some(v),
            Err(false) => Some(if_val),
            Err(true) => None,
        };
        let val = else_val.weak_cast(if_val.get_type(), self.builder);
        let (if_val, else_val) = if if_wval.is_none() {
            let else_wval = match &val {
                Ok(v) => v,
                Err(false) => else_val,
                Err(true) => {
                    return Err(CodeGenError {
                        message: "Types do not match in if statement!".into(),
                    });
                }
            };
            (if_val, else_wval)
        } else {
            (if_wval.unwrap(), else_val)
        };

        if if_val.get_type() != else_val.get_type() {}
        let llvm_value = unsafe {
            LLVMBuildSelect(
                self.builder,
                cond.get_value(self.builder, self.module)?,
                if_val.get_value(self.builder, self.module)?,
                else_val.get_value(self.builder, self.module)?,
                NULL_STR,
            )
        };
        Ok(Value::Literal {
            llvm_value,
            literal_type: if_val.get_type().clone(),
        })
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
            var @ Value::Variable {
                llvm_value,
                variable_type,
                ..
            } => {
                let ld = self.create_aligned_load(var)?.get_raw_value()?;
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

    pub fn create_struct_named(
        &self,
        path: &[String],
        name: &String,
    ) -> Result<Type, CodeGenError> {
        let name = CString::new(name.as_str()).unwrap();
        let value = unsafe { LLVMStructCreateNamed(LLVMGetGlobalContext(), name.as_ptr()) };

        // path.push(name.into_string().unwrap());
        Ok(Type::Template {
            llvm_type: value,
            fields: LinkedHashMap::new(),
            path: Vec::from(path),
        })
    }

    pub fn set_struct_body(&self, struc: &mut Type, fields: LinkedHashMap<String, Type>) {
        let mut field_types: Vec<_> = fields.values().map(|f| f.get_type()).collect();

        match struc {
            Type::Template {
                llvm_type,
                fields: flds,
                ..
            } => unsafe {
                LLVMStructSetBody(
                    *llvm_type,
                    field_types.as_mut_ptr(),
                    field_types.len().try_into().unwrap(),
                    0,
                );

                *flds = fields
            },
            _ => (),
        }
    }

    pub fn create_struct(
        &self,
        path: &[String],
        name: &String,
        fields: LinkedHashMap<String, Type>,
    ) -> Result<Type, CodeGenError> {
        let name = CString::new(name.as_str()).unwrap();
        let mut field_types: Vec<_> = fields.values().map(|f| f.get_type()).collect();

        let value = unsafe {
            let ty = LLVMStructCreateNamed(LLVMGetGlobalContext(), name.as_ptr());

            LLVMStructSetBody(
                ty,
                field_types.as_mut_ptr(),
                field_types.len().try_into().unwrap(),
                0,
            );

            ty
        };

        Ok(Type::Template {
            llvm_type: value,
            fields,
            path: Vec::from(path),
        })
    }

    pub fn create_bitcast(&self, value: &Value, ty: &Type) -> Result<Value, CodeGenError> {
        let value = value.get_raw_value()?;
        let lty = ty.get_type();
        let llvm_value = unsafe { LLVMBuildBitCast(self.builder, value, lty, NULL_STR) };

        Ok(Value::Literal {
            literal_type: ty.clone(),
            llvm_value,
        })
    }

    pub fn create_memcpy(
        &self,
        dest: &Value,
        src: &Value,
        module: LLVMModuleRef,
    ) -> Result<Value, CodeGenError> {
        let dsize = dest.get_size(module)?;
        let ssize = src.get_size(module)?;
        if dsize != ssize {
            return Err(CodeGenError {
                message: "Sizes in memcpy are not equal!".into(),
            });
        }
        let size = IRBuilder::create_literal(&IRBuilder::get_uint_64(), dsize);
        let llvm_value = unsafe {
            LLVMBuildMemCpy(
                self.builder,
                dest.get_raw_value()?,
                dest.get_alignment()?.try_into().unwrap(),
                src.get_raw_value()?,
                4.try_into().unwrap(),
                // src.get_alignment()?.try_into().unwrap(),
                size.get_raw_value()?,
            )
        };

        Ok(Value::Instruction { llvm_value })
    }

    pub fn set_allocated_type(
        &self,
        alloc: &mut Value,
        module: LLVMModuleRef,
        src: &Value,
        ty: &Type,
    ) -> Result<(), CodeGenError> {
        match alloc {
            Value::Variable {
                llvm_value,
                variable_type,
                ..
            } => {
                unsafe {
                    LLVMSetAllocatedType(*llvm_value, module, ty.get_type());
                }
                *variable_type = ty.clone();
            }
            _ => {
                return Err(CodeGenError {
                    message: "Tried to set alloc inst type on non alloca instruction or value!"
                        .into(),
                })
            }
        }
        Ok(())
    }

    pub fn is_constant(&self, val: &Value) -> Result<bool, CodeGenError> {
        let value = val.get_raw_value()?;
        Ok(if (unsafe { LLVMIsConstant(value) }) == 0 {
            false
        } else {
            true
        })
    }
}
