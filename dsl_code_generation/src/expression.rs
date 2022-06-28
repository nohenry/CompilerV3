use dsl_errors::{check, CodeGenError};
use dsl_llvm::IRBuilder;
use llvm_sys::core::LLVMGetParam;
use llvm_sys::{LLVMIntPredicate, LLVMNumberOfChildrenInBlock, LLVMOpcode, LLVMRealPredicate};

use dsl_lexer::ast::{
    BinaryExpression, Expression, FunctionCall, IfExpression, IndexExpression, Literal, Loop,
    LoopExpression, UnaryExpression,
};
use dsl_lexer::OperatorKind;

use super::module::Module;
use dsl_symbol::{Symbol, SymbolFlags, SymbolValue, Type, Value};

enum Traversal {
    Ident(String),
    Literal(Literal),
}

impl Module {
    pub(super) fn gen_expression(&self, expression: &Expression) -> Value {
        match expression {
            Expression::BinaryExpression(
                bin @ BinaryExpression {
                    left: oleft,
                    operator,
                    right: oright,
                    ..
                },
            ) => {
                use dsl_lexer::OperatorKind::*;
                let func = match operator {
                    Assignment => {
                        let left = self.gen_expression(oleft);
                        let right = self.gen_expression(oright);

                        if left.is_const() {
                            self.add_error("Attempted to assign to constant value!".into());
                            return Value::Empty;
                        }

                        check!(
                            self.errors.borrow_mut(),
                            self.builder.create_store(&left, &right, self.module),
                            Value
                        );

                        return Value::Empty;
                    }
                    Plus => LLVMOpcode::LLVMAdd,
                    Minus => LLVMOpcode::LLVMSub,
                    Mult => LLVMOpcode::LLVMMul,
                    Divide => LLVMOpcode::LLVMSDiv,
                    BitAnd => LLVMOpcode::LLVMAnd,
                    BitOr => LLVMOpcode::LLVMOr,
                    BitXor => LLVMOpcode::LLVMXor,
                    Percent => LLVMOpcode::LLVMSRem,
                    BitLeft => LLVMOpcode::LLVMShl,
                    BitRight => LLVMOpcode::LLVMAShr,
                    c => {
                        // Handle operator equal
                        let oper = match c {
                            PlusEqual => LLVMOpcode::LLVMAdd,
                            MinusEqual => LLVMOpcode::LLVMSub,
                            MultEqual => LLVMOpcode::LLVMMul,
                            DivideEqual => LLVMOpcode::LLVMSDiv,
                            BitAndEqual => LLVMOpcode::LLVMAnd,
                            BitOrEqual => LLVMOpcode::LLVMOr,
                            BitXorEqual => LLVMOpcode::LLVMXor,
                            PercentEqual => LLVMOpcode::LLVMSRem,
                            BitLeftEqual => LLVMOpcode::LLVMShl,
                            BitRightEqual => LLVMOpcode::LLVMAShr,
                            _ => {
                                match c {
                                    // Handle membor access
                                    OperatorKind::Dot => {
                                        let mut root_ch = self.symbol_root.borrow();
                                        let current = self.get_symbol(
                                            &mut root_ch,
                                            &self.current_symbol.borrow(),
                                        );

                                        fn trav<'a>(
                                            arr: &mut Vec<Traversal>,
                                            b: &BinaryExpression,
                                        ) {
                                            match (&*b.left, &*b.right, &b.operator) {
                                                (
                                                    Expression::Identifier(l),
                                                    Expression::Identifier(r),
                                                    OperatorKind::Dot,
                                                ) => {
                                                    arr.push(Traversal::Ident(
                                                        l.as_string().clone(),
                                                    ));
                                                    arr.push(Traversal::Ident(
                                                        r.as_string().clone(),
                                                    ));
                                                }
                                                (
                                                    Expression::Literal(Literal::SELF(_)),
                                                    Expression::Identifier(r),
                                                    OperatorKind::Dot,
                                                ) => {
                                                    arr.push(Traversal::Ident("self".to_string()));
                                                    arr.push(Traversal::Ident(
                                                        r.as_string().clone(),
                                                    ));
                                                }
                                                (
                                                    Expression::Literal(lit),
                                                    Expression::Identifier(r),
                                                    OperatorKind::Dot,
                                                ) => {
                                                    arr.push(Traversal::Literal(lit.clone()));
                                                    arr.push(Traversal::Ident(
                                                        r.as_string().clone(),
                                                    ));
                                                }
                                                (
                                                    Expression::BinaryExpression(l),
                                                    Expression::Identifier(r),
                                                    OperatorKind::Dot,
                                                ) => {
                                                    trav(arr, l);
                                                    arr.push(Traversal::Ident(
                                                        r.as_string().clone(),
                                                    ));
                                                }
                                                _ => (),
                                            }
                                        }

                                        let mut chain = Vec::new();

                                        trav(&mut chain, bin);

                                        let mut citer = chain.iter();

                                        let first = citer.next();
                                        let Some(first) = first else {
                                            return Value::Empty;
                                        };

                                        let root_sym = self.symbol_root.borrow();

                                        let find_syms = |var: &Value| -> Value {
                                            let path = var.get_type().resolve_path();
                                            let Some(path) = path else {
                                                self.add_error("Unable to resolve type in member access expresison! (1)".into());
                                                return Value::Empty
                                            };
                                            let Some(sym) = self.get_symbol(&root_sym, &path) else {
                                                self.add_error("Unable to resolve type in member access expresison! (2)".into());
                                                return Value::Empty
                                            };
                                            let mut var = var.clone();
                                            let mut sym = sym;
                                            // let mut fields = &sym.children;
                                            for m in citer {
                                                let Traversal::Ident(m) = m else {
                                                    self.add_error("Expected field or function name, found literal!".into());
                                                    return Value::Empty;
                                                };
                                                if let Some(child) = sym.children.get(m) {
                                                    match &child.value {
                                                        SymbolValue::Field(ty) => {
                                                            let pos = sym
                                                                .children
                                                                .keys()
                                                                .position(|f| f == m);
                                                            let Some(pos) = pos else {
                                                                    // TODO: errors
                                                                    return Value::Empty;
                                                                };
                                                            match ty {
                                                                Type::Template { path, .. } => {
                                                                    let Some(c) = self.get_symbol(&root_sym, path) else {
                                                                            return Value::Empty
                                                                        };
                                                                    sym = c;
                                                                }
                                                                _ => (),
                                                            }
                                                            var = check!(
                                                                self.errors.borrow_mut(),
                                                                self.builder.create_struct_gep(
                                                                    &var,
                                                                    ty.clone(),
                                                                    pos.try_into().unwrap(),
                                                                ),
                                                                Value
                                                            )
                                                        }
                                                        SymbolValue::Funtion(
                                                            func @ (Value::Function { .. }
                                                            | Value::FunctionTemplate {
                                                                ..
                                                            }),
                                                        ) => {
                                                            return Value::MemberFunction {
                                                                func: Box::new(func.clone()),
                                                                var: Box::new(var),
                                                            };
                                                        }

                                                        _ => (),
                                                    }
                                                }
                                                return var;
                                            }
                                            Value::Empty
                                        };

                                        match first {
                                            Traversal::Ident(ident) => {
                                                let Some(sym) = self.find_up_chain(
                                                    &root_sym,
                                                &self.current_symbol.borrow(),
                                                ident,
                                                ) else {
                                                    return Value::Empty
                                                };

                                                match &sym.value {
                                                    SymbolValue::Variable(
                                                        var @ Value::Variable {
                                                            variable_type, ..
                                                        },
                                                    ) => return (find_syms)(var),
                                                    _ => (),
                                                }
                                            }
                                            Traversal::Literal(lit) => {
                                                let val = self.gen_literal(lit);
                                                return (find_syms)(&val);
                                            }
                                        }

                                        return Value::Empty;
                                    }
                                    _ => (),
                                }
                                let left = self.gen_expression(oleft);
                                let right = self.gen_expression(oright);
                                match left.get_type() {
                                    Type::Integer { signed, .. } => {
                                        let func = match c {
                                            Eq => LLVMIntPredicate::LLVMIntEQ,
                                            NEq => LLVMIntPredicate::LLVMIntNE,
                                            Lt if *signed => LLVMIntPredicate::LLVMIntSLT,
                                            LtEq if *signed => LLVMIntPredicate::LLVMIntSLE,
                                            Gt if *signed => LLVMIntPredicate::LLVMIntSGT,
                                            GtEq if *signed => LLVMIntPredicate::LLVMIntSGE,
                                            NGt if *signed => LLVMIntPredicate::LLVMIntSLE,
                                            NLt if *signed => LLVMIntPredicate::LLVMIntSGE,

                                            Lt if !*signed => LLVMIntPredicate::LLVMIntULT,
                                            LtEq if !*signed => LLVMIntPredicate::LLVMIntULE,
                                            Gt if !*signed => LLVMIntPredicate::LLVMIntUGT,
                                            GtEq if !*signed => LLVMIntPredicate::LLVMIntUGE,
                                            NGt if !*signed => LLVMIntPredicate::LLVMIntULE,
                                            NLt if !*signed => LLVMIntPredicate::LLVMIntUGE,
                                            _ => {
                                                self.add_error(format!(
                                                    "Unsupported binary comparison operation {:?}",
                                                    c
                                                ));
                                                return Value::Empty;
                                            }
                                        };
                                        return check!(
                                            self.errors.borrow_mut(),
                                            self.builder.create_icompare(&left, &right, func),
                                            Value
                                        );
                                    }
                                    Type::Boolean { .. } => {
                                        let func = match c {
                                            Eq => LLVMIntPredicate::LLVMIntEQ,
                                            NEq => LLVMIntPredicate::LLVMIntNE,
                                            _ => {
                                                self.add_error(format!(
                                                    "Unsupported binary comparison operation {:?}",
                                                    c
                                                ));
                                                return Value::Empty;
                                            }
                                        };
                                        return check!(
                                            self.errors.borrow_mut(),
                                            self.builder.create_icompare(&left, &right, func),
                                            Value
                                        );
                                    }
                                    Type::Float { .. } => {
                                        let func = match c {
                                            Eq => LLVMRealPredicate::LLVMRealOEQ,
                                            _ => {
                                                self.add_error(format!(
                                                    "Unsupported binary operation {:?}",
                                                    c
                                                ));
                                                return Value::Empty;
                                            }
                                        };
                                        return check!(
                                            self.errors.borrow_mut(),
                                            self.builder.create_fcompare(&left, &right, func),
                                            Value
                                        );
                                    }
                                    _ => {
                                        self.add_error(format!(
                                            "Unsupported binary operation {:?}",
                                            c
                                        ));
                                        return Value::Empty;
                                    }
                                }
                            }
                        };

                        let left = self.gen_expression(oleft);
                        let right = self.gen_expression(oright);

                        if left.is_const() {
                            self.add_error("Attempted to assign to constant value!".into());
                            return Value::Empty;
                        }

                        let op = check!(
                            self.errors.borrow_mut(),
                            self.builder.create_bin_op(&left, &right, oper),
                            Value
                        );
                        check!(
                            self.errors.borrow_mut(),
                            self.builder.create_store(&left, &op, self.module),
                            Value
                        );

                        // load modify and store value for op=
                        return Value::Empty;
                    }
                };

                let left = self.gen_expression(oleft);
                let right = self.gen_expression(oright);

                check!(
                    self.errors.borrow_mut(),
                    self.builder.create_bin_op(&left, &right, func),
                    Value
                )
            }
            Expression::UnaryExpression(UnaryExpression {
                expression,
                operator,
                ..
            }) => match operator {
                OperatorKind::Minus => {
                    let expr = self.gen_expression(&expression);

                    check!(
                        self.errors.borrow_mut(),
                        self.builder.create_neg(&expr),
                        Value
                    )
                }
                OperatorKind::BitAnd => {
                    let value = self.gen_expression(&expression);

                    match value {
                        Value::Variable {
                            llvm_value,
                            variable_type,
                            constant,
                        } => Value::Literal {
                            llvm_value,
                            literal_type: IRBuilder::get_ptr(&variable_type, constant),
                        },
                        _ => Value::Empty,
                    }
                }
                OperatorKind::Mult => {
                    let value = self.gen_expression(&expression);
                    match &value {
                        Value::Variable { .. } => {
                            let value = check!(
                                self.errors.borrow_mut(),
                                self.builder.create_load(&value),
                                Value
                            );

                            match value {
                                Value::Literal {
                                    llvm_value,
                                    literal_type:
                                        Type::Reference {
                                            base_type,
                                            constant,
                                            ..
                                        },
                                } => Value::Variable {
                                    llvm_value,
                                    variable_type: *base_type,
                                    constant,
                                },
                                _ => Value::Empty,
                            }
                        }
                        _ => Value::Empty,
                    }
                }
                _ => {
                    self.add_error(String::from("Unsupproted unary operator!"));
                    Value::Empty
                }
            },
            Expression::Index(IndexExpression {
                index_expression,
                index_value,
                ..
            }) => {
                let left = self.gen_expression(&index_expression);
                let right = self.gen_expression(&index_value);

                let base_type = match (&left, &right) {
                    (
                        Value::Variable {
                            variable_type: Type::Array { base_type, .. },
                            ..
                        },
                        Value::Literal { .. },
                    ) => base_type.clone(),
                    _ => {
                        // TODO: ERROR
                        return Value::Empty;
                    }
                };

                let index0 = IRBuilder::create_literal(&IRBuilder::get_uint_64(), 0u64);
                let indicies = [index0, right];

                check!(
                    self.errors.borrow_mut(),
                    self.builder
                        .create_gep_inbound(&left, *base_type, &indicies),
                    Value
                )
            }
            Expression::Identifier(i) => {
                let str = i.as_string();

                let sym = self.symbol_root.borrow();
                let sym = self.find_up_chain(&sym, &self.current_symbol.borrow(), &str);

                if let Some(sym) = sym {
                    match &sym.value {
                        SymbolValue::Variable(v) => v.clone(),
                        SymbolValue::Funtion(v) => v.clone(),
                        _ => Value::Empty,
                    }
                } else {
                    Value::Empty
                }
            }
            Expression::Literal(literal) => self.gen_literal(literal),
            Expression::IfExpression(IfExpression {
                condition,
                body,
                else_clause,
                ..
            }) => {
                let condition = self.gen_expression(&condition);

                let if_body = check!(self.errors.borrow_mut(), self.builder.create_block(), Value);

                if let Some((_, ec)) = else_clause {
                    let else_body =
                        check!(self.errors.borrow_mut(), self.builder.create_block(), Value);

                    let (end, empty) = match &*self.jump_point.borrow() {
                        Value::Empty => {
                            let end = check!(
                                self.errors.borrow_mut(),
                                self.builder.create_block(),
                                Value
                            );
                            (end, true)
                        }
                        Value::Block { llvm_value } => (
                            Value::Block {
                                llvm_value: *llvm_value,
                            },
                            false,
                        ),
                        _ => {
                            self.add_error("Unable to get basic block".into());
                            return Value::Empty;
                        }
                    };
                    if empty {
                        self.jump_point.replace(end.clone());
                    }

                    self.builder.set_position_end(&if_body);
                    let if_ret = self.gen_parse_node(&body);
                    let if_n = match if_body {
                        Value::Block { llvm_value } => unsafe {
                            LLVMNumberOfChildrenInBlock(llvm_value)
                        },
                        _ => 0,
                    };

                    let if_const = check!(
                        self.errors.borrow_mut(),
                        self.builder.is_constant(&if_ret),
                        Value
                    );

                    self.builder.set_position_end(&else_body);
                    let else_ret = self.gen_parse_node(&ec);
                    let else_n = match else_body {
                        Value::Block { llvm_value } => unsafe {
                            LLVMNumberOfChildrenInBlock(llvm_value)
                        },
                        _ => 0,
                    };

                    let else_const = check!(
                        self.errors.borrow_mut(),
                        self.builder.is_constant(&if_ret),
                        Value
                    );

                    if !(if_const && else_const && if_n == 0 && else_n == 0
                        || if_n <= 1 && else_n <= 1)
                    {
                        self.builder.set_position_end(&*self.current_block.borrow());
                        check!(
                            self.errors.borrow_mut(),
                            self.builder
                                .create_cbranch(&condition, &if_body, &else_body),
                            Value
                        );

                        self.builder.set_position_end(&if_body);
                        check!(
                            self.errors.borrow_mut(),
                            self.builder.create_branch(&end),
                            Value
                        );

                        check!(
                            self.errors.borrow_mut(),
                            self.builder
                                .append_existing_block(&self.current_function.borrow(), &if_body),
                            Value
                        );
                        check!(
                            self.errors.borrow_mut(),
                            self.builder
                                .append_existing_block(&self.current_function.borrow(), &else_body),
                            Value
                        );

                        self.builder.set_position_end(&else_body);

                        if empty {
                            check!(
                                self.errors.borrow_mut(),
                                self.builder
                                    .append_existing_block(&self.current_function.borrow(), &end),
                                Value
                            );

                            check!(
                                self.errors.borrow_mut(),
                                self.builder.create_branch(&end),
                                Value
                            );
                        }

                        self.builder.set_position_end(&end);

                        match (&if_ret, &else_ret) {
                            (Value::Empty, Value::Empty) => (),
                            (Value::Empty, _) => (),
                            (_, Value::Empty) => (),
                            (a, b) => {
                                let p = check!(
                                    self.errors.borrow_mut(),
                                    self.builder.create_phi(a, b, &if_body, &else_body),
                                    Value
                                );
                                return p;
                            }
                        }
                    } else {
                        self.builder.set_position_end(&*self.current_block.borrow());

                        return check!(
                            self.errors.borrow_mut(),
                            self.builder.create_select(&condition, &if_ret, &else_ret),
                            Value
                        );
                    }

                    if empty {
                        self.jump_point.replace(Value::Empty);
                    }
                } else {
                    let (end, empty) = match &*self.jump_point.borrow() {
                        Value::Empty => {
                            let end = check!(
                                self.errors.borrow_mut(),
                                self.builder.append_block(&self.current_function.borrow()),
                                Value
                            );
                            (end, true)
                        }
                        Value::Block { llvm_value } => (
                            Value::Block {
                                llvm_value: *llvm_value,
                            },
                            false,
                        ),
                        _ => {
                            self.add_error("Unable to get basic block".into());
                            return Value::Empty;
                        }
                    };
                    if empty {
                        self.jump_point.replace(end.clone());
                    }

                    check!(
                        self.errors.borrow_mut(),
                        self.builder.create_cbranch(&condition, &if_body, &end),
                        Value
                    );

                    self.builder.set_position_end(&if_body);
                    self.gen_parse_node(&body);
                    check!(
                        self.errors.borrow_mut(),
                        self.builder.create_branch(&end),
                        Value
                    );

                    self.builder.set_position_end(&end);
                    if empty {
                        self.jump_point.replace(Value::Empty);
                    }
                }

                Value::Empty
            }
            Expression::LoopExpression(LoopExpression { loop_type, .. }) => match loop_type {
                Loop::Until(cond, body) => {
                    let condition_block = check!(
                        self.errors.borrow_mut(),
                        self.builder.append_block(&self.current_function.borrow()),
                        Value
                    );

                    let body_block =
                        check!(self.errors.borrow_mut(), self.builder.create_block(), Value);

                    let end = check!(self.errors.borrow_mut(), self.builder.create_block(), Value);

                    check!(
                        self.errors.borrow_mut(),
                        self.builder.create_branch(&condition_block),
                        Value
                    );

                    self.builder.set_position_end(&condition_block);
                    let cond = self.gen_expression(cond);

                    check!(
                        self.errors.borrow_mut(),
                        self.builder.create_cbranch(&cond, &body_block, &end),
                        Value
                    );

                    check!(
                        self.errors.borrow_mut(),
                        self.builder
                            .append_existing_block(&self.current_function.borrow(), &body_block),
                        Value
                    );

                    self.builder.set_position_end(&body_block);

                    self.gen_parse_node(body);

                    let val = check!(
                        self.errors.borrow_mut(),
                        self.builder.create_branch(&condition_block),
                        Value
                    );

                    check!(
                        self.errors.borrow_mut(),
                        self.builder
                            .append_existing_block(&self.current_function.borrow(), &end),
                        Value
                    );

                    self.builder.set_position_end(&end);

                    val
                }
                Loop::Infinite(body) => {
                    let loop_block = check!(
                        self.errors.borrow_mut(),
                        self.builder.append_block(&self.current_function.borrow()),
                        Value
                    );
                    check!(
                        self.errors.borrow_mut(),
                        self.builder.create_branch(&loop_block),
                        Value
                    );

                    self.builder.set_position_end(&loop_block);

                    self.gen_parse_node(body);

                    check!(
                        self.errors.borrow_mut(),
                        self.builder.create_branch(&loop_block),
                        Value
                    )
                }
            },
            Expression::FunctionCall(FunctionCall {
                arguments,
                expression_to_call,
                generic,
                ..
            }) => {
                let arguments: Vec<_> = arguments.iter().map(|f| self.gen_expression(f)).collect();
                let expr = self.gen_expression(&expression_to_call);
                if let Value::FunctionTemplate {
                    body,
                    path,
                    ty,
                    ty_params,
                    existing,
                    specialization,
                } = &expr
                {
                    let old_block = self.current_block.take();

                    let old_symbol = self.current_symbol.take();
                    let old_function = self.current_function.take();
                    self.current_symbol.replace(path.clone());

                    let mut fn_types = vec![];

                    // Generate type arguments
                    {
                        let mut sym = self.symbol_root.borrow_mut();
                        let current = self.get_symbol_mut(&mut sym, &self.current_symbol.borrow());

                        if let Some(current) = current {
                            let mut gens = generic
                                .as_ref()
                                .map_or(Some([].iter()), |f| Some(f.iter()))
                                .unwrap();
                            let mut pram_iter = ty_params.iter();

                            while let Some(p) = gens.next() {
                                let Some((name, bounds)) = pram_iter.next() else {
                                    self.add_error(format!("Extra generic parameter in function call!"));
                                    break;
                                };
                                if let Some(sym) = current.children.get_mut(name) {
                                    match &mut sym.value {
                                        SymbolValue::Generic(ty, _) => {
                                            *ty = self.gen_type(p);
                                            fn_types.push(ty.to_string())
                                        }
                                        _ => (),
                                    }
                                }
                            }

                            pram_iter.for_each(|(name, bounds)| {
                                let found = ty.parameters.iter().position(|f| {
                                    if let dsl_lexer::ast::Type::NamedType(t) = &f.symbol_type {
                                        if &t.as_string() == name {
                                            return true;
                                        }
                                    }
                                    return false;
                                });
                                if let Some(pos) = found {
                                    if let Some(sym) = current.children.get_mut(name) {
                                        match &mut sym.value {
                                            SymbolValue::Generic(ty, _) => {
                                                *ty = arguments[pos].get_type().clone();
                                                fn_types.push(ty.to_string())
                                            }
                                            _ => (),
                                        }
                                    }
                                } else {
                                    self.add_error(format!(
                                        "Unable to determine type for generic paramater: `{}` -- Please provide type parameter",
                                        name
                                    ))
                                }
                            })
                        }
                    }

                    let specialization = specialization.iter().find(|(f, _)| {
                        for (a, b) in f.iter().zip(fn_types.iter()) {
                            if a == "" {
                                return true;
                            } else if a != b {
                                return false;
                            } else {
                                return true;
                            }
                        }
                        true
                    });
                    let existing_impl = existing.iter().find(|(f, _)| *f == &fn_types);

                    let name = if let Some(last) = path.last() {
                        if let Some(ty) = existing_impl {
                            Some(ty.1.last().unwrap().clone())
                        } else {
                            let (return_type, types) = {
                                let sym = self.symbol_root.borrow();

                                let typ = if let Some(special) = specialization {
                                    let current = self.get_symbol(&sym, &special.1);

                                    // FIXME: idk check if three of these are really needed
                                    if let Some(Symbol {
                                        value:
                                            SymbolValue::Funtion(Value::FunctionTemplate {
                                                body,
                                                ty,
                                                ..
                                            }),
                                        ..
                                    }) = current
                                    {
                                        Some(ty)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                };

                                let ty = if let Some(ty) = typ { ty } else { ty };
                                let return_type = self.gen_type(&ty.return_type);

                                let types: Option<Vec<(String, Type)>> = ty
                                    .parameters
                                    .iter()
                                    .map(|f| {
                                        if &f.symbol.as_string() == "self" {
                                            let sym = self.symbol_root.borrow();
                                            let current = self
                                                .get_symbol(&sym, &self.current_symbol.borrow());

                                            if let Some(Symbol {
                                                value: SymbolValue::Template(t),
                                                flags,
                                                ..
                                            }) = current
                                            {
                                                Some((
                                                    f.symbol.as_string().clone(),
                                                    t.clone().get_ptr(
                                                        flags.contains(SymbolFlags::CONSTANT),
                                                    ),
                                                ))
                                            } else {
                                                None
                                            }
                                        } else {
                                            Some((
                                                f.symbol.as_string().clone(),
                                                self.gen_type(&f.symbol_type),
                                            ))
                                        }
                                    })
                                    .collect();
                                let Some(types) = types else {
                                    //TODO: add error 
                                    return Value::Empty
                                };

                                (return_type, types)
                            };

                            let path = &path[..path.len() - 1];
                            let name = self.get_next_name(path, last.clone());

                            let function_type = IRBuilder::get_fn(return_type.clone(), &types);

                            let mng = self.get_mangled_name_with_path(path, &name);
                            let function = check!(
                                self.errors.borrow_mut(),
                                self.builder.add_function(function_type, mng, self.module),
                                Value
                            );

                            let block = check!(
                                self.errors.borrow_mut(),
                                self.builder.append_block(&function),
                                Value
                            );

                            self.current_block.replace(block);
                            self.builder.set_position_end(&self.current_block.borrow());

                            self.current_function.replace(function.clone());

                            self.add_and_set_symbol_from_path(
                                &path,
                                &name,
                                SymbolValue::Funtion(function),
                            );

                            let pallocs: Result<Vec<Value>, _> = types
                                .iter()
                                .map(|(_name, ty)| self.builder.create_alloc(ty, true))
                                .collect();
                            let pallocs = check!(self.errors.borrow_mut(), pallocs, Value);

                            let res: Result<(), _> = pallocs
                                .iter()
                                .enumerate()
                                .map(|(i, alloc)| {
                                    let param = unsafe {
                                        if let Ok(p) = self
                                            .current_function
                                            .borrow()
                                            .get_value(self.builder.get_builder(), self.module)
                                        {
                                            LLVMGetParam(p, i.try_into().unwrap())
                                        } else {
                                            return Err(CodeGenError {
                                                message: "Unable to get function value!".to_owned(),
                                            });
                                        }
                                    };
                                    self.builder.create_store_raw_val(alloc, param)?;
                                    Ok(())
                                })
                                .collect();

                            check!(self.errors.borrow_mut(), res, Value);

                            {
                                let mut cur_sym = self.symbol_root.borrow_mut();
                                let current = self
                                    .get_symbol_mut(&mut cur_sym, &self.current_symbol.borrow());

                                if let Some(c) = current {
                                    for ((name, ty), alloc) in types.iter().zip(pallocs.into_iter())
                                    {
                                        c.add_child(&name, SymbolValue::Variable(alloc));
                                    }
                                }
                            }

                            let alloc = match return_type {
                                Type::Unit { .. } => None,
                                ty => Some(check!(
                                    self.errors.borrow_mut(),
                                    self.builder.create_alloc(&ty, false),
                                    Value
                                )),
                            };

                            let val = if let Some(special) = specialization {
                                let mut sym = self.symbol_root.borrow();
                                let current = self.get_symbol(&mut sym, &special.1);

                                if let Some(Symbol {
                                    value:
                                        SymbolValue::Funtion(Value::FunctionTemplate {
                                            body, ty, ..
                                        }),
                                    ..
                                }) = current
                                {
                                    self.gen_parse_node(body.as_ref())
                                } else {
                                    self.gen_parse_node(body.as_ref())
                                }
                            } else {
                                self.gen_parse_node(body.as_ref())
                            };

                            if let Some(alloc) = alloc {
                                if val.has_value() {
                                    check!(
                                        self.errors.borrow_mut(),
                                        self.builder.create_store(&alloc, &val, self.module),
                                        Value
                                    );
                                }

                                check!(
                                    self.errors.borrow_mut(),
                                    self.builder.create_ret(&alloc),
                                    Value
                                );
                            } else {
                                self.builder.create_ret_void();
                            }

                            Some(name)
                        }
                    } else {
                        None
                    };

                    self.current_block.replace(old_block);
                    self.current_symbol.replace(old_symbol);
                    self.current_function.replace(old_function);

                    self.builder.set_position_end(&self.current_block.borrow());

                    if let Some(name) = name {
                        let mut npath = Vec::from(&path[..path.len() - 1]);
                        npath.push(name);

                        let mut sym = self.symbol_root.borrow_mut();

                        // let mut sym = self.symbol_root.borrow_mut();
                        let fn_templ = self.get_symbol_mut(&mut sym, &path);
                        if let Some(Symbol {
                            value: SymbolValue::Funtion(Value::FunctionTemplate { existing, .. }),
                            ..
                        }) = fn_templ
                        {
                            existing.insert(fn_types, npath.clone());
                        }

                        let current = self.get_symbol(&mut sym, &npath);
                        if let Some(Symbol {
                            value: SymbolValue::Funtion(val),
                            ..
                        }) = current
                        {
                            check!(
                                self.errors.borrow_mut(),
                                self.builder.create_fn_call(val, arguments),
                                Value
                            )
                        } else {
                            Value::Empty
                        }
                    } else {
                        Value::Empty
                    }
                } else {
                    check!(
                        self.errors.borrow_mut(),
                        self.builder.create_fn_call(&expr, arguments),
                        Value
                    )
                }
            }
            Expression::Block(values, _) => {
                return values
                    .iter()
                    .map(|stmt| self.gen_parse_node(stmt))
                    .last()
                    .or(Some(Value::Empty))
                    .unwrap()
            }
            _ => {
                self.add_error(String::from("Unsupported expression"));
                Value::Empty
            }
        }
    }
}
