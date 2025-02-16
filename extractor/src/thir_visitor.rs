use crate::{converters::ConvertInto, utils::pretty_description};
use corpus_database::types::{self, Safety, ThirBlock};
use rustc_hir as hir;
use rustc_middle::{
    thir::{ExprId, Thir},
    ty::{self, TyCtxt},
};

use crate::table_filler::TableFiller;

pub(crate) struct ThirVisitor<'a, 'b, 'thir, 'tcx> {
    tcx: TyCtxt<'tcx>,
    thir: &'thir Thir<'tcx>,
    body_id: ExprId,
    current_block: ThirBlock,
    closest_unsafe_block: ThirBlock,
    filler: &'a mut TableFiller<'b, 'tcx>,
}

impl<'a, 'b, 'thir, 'tcx: 'thir> ThirVisitor<'a, 'b, 'thir, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        thir: &'thir Thir<'tcx>,
        body_id: ExprId,
        root_block: ThirBlock,
        filler: &'a mut TableFiller<'b, 'tcx>,
    ) -> Self {
        Self {
            tcx,
            thir,
            body_id,
            current_block: root_block,
            closest_unsafe_block: filler.tables.get_no_thir_block(),
            filler,
        }
    }

    // Returns the guard and body expr of the match arm
    fn visit_match_arm_and_intern(
        &mut self,
        arm: &'thir rustc_middle::thir::Arm<'tcx>,
    ) -> (types::ThirExpr, types::ThirExpr) {
        let interned_guard = if let Some(guard) = &arm.guard {
            self.visit_expr_and_intern(&self.thir[*guard])
        } else {
            self.filler.tables.get_no_thir_expr()
        };

        let interned_body = self.visit_expr_and_intern(&self.thir[arm.body]);

        (interned_guard, interned_body)
    }

    fn visit_expr_and_intern(
        &mut self,
        expr: &'thir rustc_middle::thir::Expr<'tcx>,
    ) -> types::ThirExpr {
        let interned_expr_type = self.filler.register_type(expr.ty);

        let (interned_expr,) = match &expr.kind {
            rustc_middle::thir::ExprKind::Scope { value, .. } => {
                let interned_value = self.visit_expr_and_intern(&self.thir[*value]);
                self.filler.tables.register_thir_exprs_scope(interned_value)
            }
            rustc_middle::thir::ExprKind::Box { value } => {
                let interned_value = self.visit_expr_and_intern(&self.thir[*value]);
                self.filler.tables.register_thir_exprs_box(interned_value)
            }
            rustc_middle::thir::ExprKind::If {
                cond,
                then,
                else_opt,
                ..
            } => {
                let interned_cond = self.visit_expr_and_intern(&self.thir[*cond]);
                let interned_then = self.visit_expr_and_intern(&self.thir[*then]);
                let interned_else = if let Some(else_expr) = else_opt {
                    self.visit_expr_and_intern(&self.thir[*else_expr])
                } else {
                    self.filler.tables.get_no_thir_expr()
                };
                self.filler.tables.register_thir_exprs_if(
                    interned_cond,
                    interned_then,
                    interned_else,
                )
            }
            rustc_middle::thir::ExprKind::Call { ty, fun, args, .. } => {
                let interned_ty = self.filler.register_type(*ty);
                let interned_fun = self.visit_expr_and_intern(&self.thir[*fun]);

                let (safety, abi) = match ty.kind() {
                    ty::TyKind::FnDef(..) | ty::TyKind::FnPtr(..) => {
                        let sig = ty.fn_sig(self.tcx);
                        (sig.safety().convert_into(), sig.abi().name().to_string())
                    }
                    ty::TyKind::Closure(_def_id, args) => {
                        let sig = args.as_closure().sig();
                        (sig.safety().convert_into(), sig.abi().name().to_string())
                    }
                    _ => (Safety::Unknown, "Unknown".to_string()),
                };

                let (interned_call_expr,) = self.filler.tables.register_thir_exprs_call(
                    interned_ty,
                    interned_fun,
                    safety,
                    abi,
                    interned_expr_type,
                );
                for (i, arg) in args.iter().enumerate() {
                    let interned_arg = self.visit_expr_and_intern(&self.thir[*arg]);
                    self.filler.tables.register_thir_exprs_call_arg(
                        interned_call_expr,
                        i.try_into().unwrap(),
                        interned_arg,
                    );
                }

                match ty.kind() {
                    // We only care about const targets
                    ty::TyKind::FnDef(target_id, substs) => {
                        let generics = self.tcx.generics_of(*target_id);
                        if generics.has_self {
                            let self_ty = substs.type_at(0);
                            let interned_type = self.filler.register_type(self_ty);
                            self.filler
                                .tables
                                .register_thir_exprs_call_const_target_self(
                                    interned_fun,
                                    interned_type,
                                );
                        }
                        let desc = pretty_description(self.tcx, *target_id, substs);
                        let def_path = self.filler.resolve_def_id(*target_id);
                        self.filler
                            .tables
                            .register_thir_exprs_call_const_target(interned_fun, def_path);
                        self.filler
                            .tables
                            .register_thir_exprs_call_const_target_desc(
                                interned_fun,
                                desc.path,
                                desc.function_generics,
                                desc.type_generics,
                            );
                    }
                    _ => {}
                }

                (interned_call_expr,)
            }
            rustc_middle::thir::ExprKind::Deref { arg } => {
                let interned_arg = self.visit_expr_and_intern(&self.thir[*arg]);
                self.filler.tables.register_thir_exprs_deref(interned_arg)
            }
            rustc_middle::thir::ExprKind::Binary { op, lhs, rhs } => {
                let op = format!("{:?}", op);
                let interned_lhs = self.visit_expr_and_intern(&self.thir[*lhs]);
                let interned_rhs = self.visit_expr_and_intern(&self.thir[*rhs]);
                self.filler
                    .tables
                    .register_thir_exprs_binary(op, interned_lhs, interned_rhs)
            }
            rustc_middle::thir::ExprKind::LogicalOp { op, lhs, rhs } => {
                let op = format!("{:?}", op);
                let interned_lhs = self.visit_expr_and_intern(&self.thir[*lhs]);
                let interned_rhs = self.visit_expr_and_intern(&self.thir[*rhs]);
                self.filler
                    .tables
                    .register_thir_exprs_logical_op(op, interned_lhs, interned_rhs)
            }
            rustc_middle::thir::ExprKind::Unary { op, arg } => {
                let op = format!("{:?}", op);
                let interned_arg = self.visit_expr_and_intern(&self.thir[*arg]);
                self.filler
                    .tables
                    .register_thir_exprs_unary(op, interned_arg)
            }
            rustc_middle::thir::ExprKind::Cast { source } => {
                let interned_source = self.visit_expr_and_intern(&self.thir[*source]);
                self.filler.tables.register_thir_exprs_cast(interned_source)
            }
            rustc_middle::thir::ExprKind::Use { source } => {
                let interned_source = self.visit_expr_and_intern(&self.thir[*source]);
                self.filler.tables.register_thir_exprs_use(interned_source)
            }
            rustc_middle::thir::ExprKind::NeverToAny { source } => {
                let interned_source = self.visit_expr_and_intern(&self.thir[*source]);
                self.filler
                    .tables
                    .register_thir_exprs_never_to_any(interned_source)
            }
            rustc_middle::thir::ExprKind::PointerCoercion {
                cast,
                source,
                is_from_as_cast,
            } => {
                let interned_source = self.visit_expr_and_intern(&self.thir[*source]);
                self.filler.tables.register_thir_exprs_pointer_coercion(
                    cast.convert_into(),
                    interned_source,
                    *is_from_as_cast,
                )
            }
            rustc_middle::thir::ExprKind::Loop { body } => {
                let interned_body = self.visit_expr_and_intern(&self.thir[*body]);
                self.filler.tables.register_thir_exprs_loop(interned_body)
            }
            rustc_middle::thir::ExprKind::Let { expr, pat } => {
                let interned_expr = self.visit_expr_and_intern(&self.thir[*expr]);
                let interned_pat_ty = self.filler.register_type(pat.ty);
                let interned_pat_span = self.filler.register_span(pat.span);
                let (interned_pat,) = self
                    .filler
                    .tables
                    .register_thir_pats(interned_pat_ty, interned_pat_span);
                self.filler
                    .tables
                    .register_thir_exprs_let(interned_expr, interned_pat)
            }
            rustc_middle::thir::ExprKind::Match {
                scrutinee,
                arms,
                match_source,
                ..
            } => {
                let interned_scrutinee = self.visit_expr_and_intern(&self.thir[*scrutinee]);

                let (match_expr,) = self
                    .filler
                    .tables
                    .register_thir_exprs_match(interned_scrutinee, match_source.convert_into());

                for (i, arm) in arms.iter().enumerate() {
                    let (interned_guard, interned_body) =
                        self.visit_match_arm_and_intern(&self.thir[*arm]);
                    self.filler.tables.register_thir_match_arms(
                        match_expr,
                        i.into(),
                        interned_guard,
                        interned_body,
                    );
                }

                (match_expr,)
            }
            rustc_middle::thir::ExprKind::Block { block } => {
                let interned_block = self.visit_block_and_intern(&self.thir[*block]);
                self.filler.tables.register_thir_exprs_block(interned_block)
            }
            rustc_middle::thir::ExprKind::Assign { lhs, rhs } => {
                let interned_lhs = self.visit_expr_and_intern(&self.thir[*lhs]);
                let interned_rhs = self.visit_expr_and_intern(&self.thir[*rhs]);
                self.filler
                    .tables
                    .register_thir_exprs_assign(interned_lhs, interned_rhs)
            }
            rustc_middle::thir::ExprKind::AssignOp { op, lhs, rhs } => {
                let op = format!("{:?}", op);
                let interned_lhs = self.visit_expr_and_intern(&self.thir[*lhs]);
                let interned_rhs = self.visit_expr_and_intern(&self.thir[*rhs]);
                self.filler
                    .tables
                    .register_thir_exprs_assign_op(op, interned_lhs, interned_rhs)
            }
            rustc_middle::thir::ExprKind::Field {
                lhs, variant_index, ..
            } => {
                let interned_lhs = self.visit_expr_and_intern(&self.thir[*lhs]);
                self.filler
                    .tables
                    .register_thir_exprs_field(interned_lhs, variant_index.convert_into())
            }
            rustc_middle::thir::ExprKind::Index { lhs, index } => {
                let interned_lhs = self.visit_expr_and_intern(&self.thir[*lhs]);
                let interned_index = self.visit_expr_and_intern(&self.thir[*index]);
                self.filler
                    .tables
                    .register_thir_exprs_index(interned_lhs, interned_index)
            }
            rustc_middle::thir::ExprKind::VarRef { .. } => {
                self.filler.tables.register_thir_exprs_var_ref()
            }
            rustc_middle::thir::ExprKind::UpvarRef { closure_def_id, .. } => {
                let interned_def_path = self.filler.resolve_def_id(*closure_def_id);
                self.filler
                    .tables
                    .register_thir_exprs_upvar_ref(interned_def_path)
            }
            rustc_middle::thir::ExprKind::Borrow { borrow_kind, arg } => {
                let interned_arg = self.visit_expr_and_intern(&self.thir[*arg]);
                self.filler
                    .tables
                    .register_thir_exprs_borrow(borrow_kind.convert_into(), interned_arg)
            }
            rustc_middle::thir::ExprKind::RawBorrow { mutability, arg } => {
                let interned_arg = self.visit_expr_and_intern(&self.thir[*arg]);
                self.filler
                    .tables
                    .register_thir_exprs_raw_borrow(mutability.convert_into(), interned_arg)
            }
            rustc_middle::thir::ExprKind::Break { value, .. } => {
                let interned_value = if let Some(value) = value {
                    self.visit_expr_and_intern(&self.thir[*value])
                } else {
                    self.filler.tables.get_no_thir_expr()
                };
                self.filler.tables.register_thir_exprs_break(interned_value)
            }
            rustc_middle::thir::ExprKind::Continue { .. } => {
                self.filler.tables.register_thir_exprs_continue()
            }
            rustc_middle::thir::ExprKind::Return { value } => {
                let interned_value = if let Some(value) = value {
                    self.visit_expr_and_intern(&self.thir[*value])
                } else {
                    self.filler.tables.get_no_thir_expr()
                };
                self.filler
                    .tables
                    .register_thir_exprs_return(interned_value)
            }
            rustc_middle::thir::ExprKind::Become { value } => {
                let interned_value = self.visit_expr_and_intern(&self.thir[*value]);
                self.filler
                    .tables
                    .register_thir_exprs_become(interned_value)
            }
            rustc_middle::thir::ExprKind::ConstBlock { did, .. } => {
                let interned_def_path = self.filler.resolve_def_id(*did);
                self.filler
                    .tables
                    .register_thir_exprs_const_block(interned_def_path)
            }
            rustc_middle::thir::ExprKind::Repeat { value, .. } => {
                let interned_value = self.visit_expr_and_intern(&self.thir[*value]);
                self.filler
                    .tables
                    .register_thir_exprs_repeat(interned_value)
            }
            rustc_middle::thir::ExprKind::Array { fields } => {
                let (interned_array_expr,) = self.filler.tables.register_thir_exprs_array();

                for (i, field) in fields.iter().enumerate() {
                    let field_expr = self.visit_expr_and_intern(&self.thir[*field]);
                    self.filler.tables.register_thir_array_elements(
                        interned_array_expr,
                        i.try_into().unwrap(),
                        field_expr,
                    );
                }

                (interned_array_expr,)
            }
            rustc_middle::thir::ExprKind::Tuple { fields } => {
                let (interned_tuple_expr,) = self.filler.tables.register_thir_exprs_tuple();

                for (i, field) in fields.iter().enumerate() {
                    let field_expr = self.visit_expr_and_intern(&self.thir[*field]);
                    self.filler.tables.register_thir_tuple_elements(
                        interned_tuple_expr,
                        i.try_into().unwrap(),
                        field_expr,
                    );
                }

                (interned_tuple_expr,)
            }
            rustc_middle::thir::ExprKind::Adt(adt_expr) => {
                let base = if let Some(base) = &adt_expr.base {
                    self.visit_expr_and_intern(&self.thir[base.base])
                } else {
                    self.filler.tables.get_no_thir_expr()
                };

                let (interned_adt_expr,) = self
                    .filler
                    .tables
                    .register_thir_exprs_adt(base, adt_expr.variant_index.convert_into());

                for field in &adt_expr.fields {
                    let field_expr = self.visit_expr_and_intern(&self.thir[field.expr]);
                    self.filler.tables.register_thir_adt_field_expr(
                        interned_adt_expr,
                        field.name.convert_into(),
                        field_expr,
                    );
                }

                (interned_adt_expr,)
            }
            rustc_middle::thir::ExprKind::PlaceTypeAscription {
                source,
                user_ty_span,
                ..
            } => {
                let interned_source = self.visit_expr_and_intern(&self.thir[*source]);
                let interned_user_ty_span = self.filler.register_span(*user_ty_span);
                self.filler
                    .tables
                    .register_thir_exprs_place_type_ascription(
                        interned_source,
                        interned_user_ty_span,
                    )
            }
            rustc_middle::thir::ExprKind::ValueTypeAscription {
                source,
                user_ty_span,
                ..
            } => {
                let interned_source = self.visit_expr_and_intern(&self.thir[*source]);
                let interned_user_ty_span = self.filler.register_span(*user_ty_span);
                self.filler
                    .tables
                    .register_thir_exprs_value_type_ascription(
                        interned_source,
                        interned_user_ty_span,
                    )
            }
            rustc_middle::thir::ExprKind::Closure(closure_expr) => {
                let closure_def_id = self.filler.resolve_def_id(closure_expr.closure_id.into());

                let (interned_closure_expr,) = self.filler.tables.register_thir_exprs_closure(
                    closure_def_id,
                    closure_expr.movability.convert_into(),
                );

                for (i, upvar) in closure_expr.upvars.iter().enumerate() {
                    let interned_upvar = self.visit_expr_and_intern(&self.thir[*upvar]);
                    self.filler.tables.register_thir_closure_upvars(
                        interned_closure_expr,
                        i.try_into().unwrap(),
                        interned_upvar,
                    );
                }

                (interned_closure_expr,)
            }
            rustc_middle::thir::ExprKind::Literal { lit, neg } => self
                .filler
                .tables
                .register_thir_exprs_literal(lit.node.convert_into(), *neg),
            rustc_middle::thir::ExprKind::NonHirLiteral { lit, .. } => {
                let db_lit = lit.to_bits_unchecked();
                self.filler
                    .tables
                    .register_thir_exprs_non_hir_literal(db_lit)
            }
            rustc_middle::thir::ExprKind::ZstLiteral { .. } => {
                self.filler.tables.register_thir_exprs_zst_literal()
            }
            rustc_middle::thir::ExprKind::NamedConst { def_id, .. } => {
                let interned_def_path = self.filler.resolve_def_id(*def_id);
                self.filler
                    .tables
                    .register_thir_exprs_named_const(interned_def_path)
            }
            rustc_middle::thir::ExprKind::ConstParam { def_id, .. } => {
                let interned_def_path = self.filler.resolve_def_id(*def_id);
                self.filler
                    .tables
                    .register_thir_exprs_const_param(interned_def_path)
            }
            rustc_middle::thir::ExprKind::StaticRef { ty, def_id, .. } => {
                let interned_def_path = self.filler.resolve_def_id(*def_id);
                let interned_ty = self.filler.register_type(*ty);
                self.filler
                    .tables
                    .register_thir_exprs_static_ref(interned_ty, interned_def_path)
            }
            rustc_middle::thir::ExprKind::InlineAsm(_) => {
                self.filler.tables.register_thir_exprs_inline_asm()
            }
            rustc_middle::thir::ExprKind::OffsetOf { container, .. } => {
                let interned_container = self.filler.register_type(*container);
                self.filler
                    .tables
                    .register_thir_exprs_offset_of(interned_container)
            }
            rustc_middle::thir::ExprKind::ThreadLocalRef(def_id) => {
                let interned_def_path = self.filler.resolve_def_id(*def_id);
                self.filler
                    .tables
                    .register_thir_exprs_thread_local_ref(interned_def_path)
            }
            rustc_middle::thir::ExprKind::Yield { value } => {
                let interned_value = self.visit_expr_and_intern(&self.thir[*value]);
                self.filler.tables.register_thir_exprs_yield(interned_value)
            }
        };

        let interned_span = self.filler.register_span(expr.span);
        self.filler.tables.register_thir_exprs(
            interned_expr,
            self.current_block,
            self.closest_unsafe_block,
            interned_expr_type,
            interned_span,
        );

        interned_expr
    }

    fn visit_stmt_and_intern(
        &mut self,
        stmt: &'thir rustc_middle::thir::Stmt,
        idx: usize,
    ) -> types::ThirStmt {
        let (interned_stmt,) = match &stmt.kind {
            rustc_middle::thir::StmtKind::Expr { expr, scope: _ } => {
                let interned_expr = self.visit_expr_and_intern(&self.thir[*expr]);
                self.filler.tables.register_thir_stmts_expr(interned_expr)
            }
            rustc_middle::thir::StmtKind::Let {
                initializer,
                else_block,
                span,
                ..
            } => {
                let init_expr = if let Some(init) = initializer {
                    self.visit_expr_and_intern(&self.thir[*init])
                } else {
                    self.filler.tables.get_no_thir_expr()
                };

                let interned_span = self.filler.register_span(*span);

                let interned_else_block = if let Some(else_block) = else_block {
                    self.visit_block_and_intern(&self.thir[*else_block])
                } else {
                    self.filler.tables.get_no_thir_block()
                };

                self.filler.tables.register_thir_stmts_let(
                    init_expr,
                    interned_else_block,
                    interned_span,
                )
            }
        };

        self.filler.tables.register_thir_stmts(
            interned_stmt,
            self.current_block,
            self.closest_unsafe_block,
            idx.into(),
        );

        interned_stmt
    }

    fn visit_block_and_intern(&mut self, block: &'thir rustc_middle::thir::Block) -> ThirBlock {
        let safety = block.safety_mode;
        let check_mode;
        if let rustc_middle::thir::BlockSafety::ExplicitUnsafe(hir_id) = safety {
            match self.tcx.hir_node(hir_id) {
                hir::Node::Block(block) => {
                    check_mode = block.rules.convert_into();
                }
                _ => unreachable!("Unexpected HIR node type."),
            }
        } else {
            check_mode = types::BlockCheckMode::DefaultBlock;
        }

        let parent_block = self.current_block;
        let parent_closest_unsafe_block = self.closest_unsafe_block;

        let span = self.filler.register_span(block.span);
        let (the_block,) = self.filler.tables.register_thir_blocks(
            parent_block,
            Some(safety).convert_into(),
            check_mode,
            span,
        );
        self.current_block = the_block;
        // We're entering a new closest unsafe block
        if !matches!(safety, rustc_middle::thir::BlockSafety::Safe) {
            self.closest_unsafe_block = the_block;
        }

        for (idx, stmt) in block.stmts.iter().enumerate() {
            self.visit_stmt_and_intern(&self.thir[*stmt], idx);
        }
        if let Some(expr) = block.expr {
            let interned_expr = self.visit_expr_and_intern(&self.thir[expr]);
            self.filler
                .tables
                .register_thir_block_expr(the_block, interned_expr);
        }

        // reset state
        self.current_block = parent_block;
        self.closest_unsafe_block = parent_closest_unsafe_block;

        the_block
    }

    pub fn visit(&mut self) {
        self.visit_expr_and_intern(&self.thir[self.body_id]);
    }
}
