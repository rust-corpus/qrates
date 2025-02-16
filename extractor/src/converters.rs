// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

use corpus_database::types;
use rustc_hir as hir;
use rustc_middle::{
    mir,
    ty::{self, adjustment::PointerCoercion},
};

pub trait ConvertInto<T> {
    fn convert_into(&self) -> T;
}

impl ConvertInto<types::TyVisibility> for ty::Visibility<rustc_hir::def_id::DefId> {
    fn convert_into(&self) -> types::TyVisibility {
        match self {
            ty::Visibility::Public => types::TyVisibility::Public,
            ty::Visibility::Restricted(_) => types::TyVisibility::Restricted,
        }
    }
}

impl ConvertInto<types::Safety> for hir::Safety {
    fn convert_into(&self) -> types::Safety {
        match self {
            hir::Safety::Unsafe => types::Safety::Unsafe,
            hir::Safety::Safe => types::Safety::Safe,
        }
    }
}

impl ConvertInto<types::SpanExpansionKind> for rustc_span::hygiene::ExpnKind {
    fn convert_into(&self) -> types::SpanExpansionKind {
        use rustc_span::hygiene::AstPass;
        use rustc_span::hygiene::DesugaringKind;
        use rustc_span::hygiene::ExpnKind as EK;
        use rustc_span::hygiene::MacroKind;
        match self {
            EK::Root => types::SpanExpansionKind::Root,
            EK::Macro(MacroKind::Bang, _) => types::SpanExpansionKind::MacroBang,
            EK::Macro(MacroKind::Attr, _) => types::SpanExpansionKind::MacroAttr,
            EK::Macro(MacroKind::Derive, _) => types::SpanExpansionKind::MacroDerive,
            EK::AstPass(AstPass::StdImports) => types::SpanExpansionKind::AstPassStdImports,
            EK::AstPass(AstPass::TestHarness) => types::SpanExpansionKind::AstPassTestHarness,
            EK::AstPass(AstPass::ProcMacroHarness) => {
                types::SpanExpansionKind::AstPassProcMacroHarness
            }
            EK::Desugaring(DesugaringKind::CondTemporary) => {
                types::SpanExpansionKind::DesugaringCondTemporary
            }
            EK::Desugaring(DesugaringKind::QuestionMark) => {
                types::SpanExpansionKind::DesugaringQuestionMark
            }
            EK::Desugaring(DesugaringKind::TryBlock) => {
                types::SpanExpansionKind::DesugaringTryBlock
            }
            EK::Desugaring(DesugaringKind::OpaqueTy) => {
                types::SpanExpansionKind::DesugaringOpaqueTy
            }
            EK::Desugaring(DesugaringKind::Async) => types::SpanExpansionKind::DesugaringAsync,
            EK::Desugaring(DesugaringKind::Await) => types::SpanExpansionKind::DesugaringAwait,
            EK::Desugaring(DesugaringKind::ForLoop) => types::SpanExpansionKind::DesugaringForLoop,
            EK::Desugaring(DesugaringKind::WhileLoop) => {
                types::SpanExpansionKind::DesugaringWhileLoop
            }
            EK::Desugaring(DesugaringKind::YeetExpr) => {
                types::SpanExpansionKind::DesugaringYeetExpr
            }
            EK::Desugaring(DesugaringKind::BoundModifier) => {
                types::SpanExpansionKind::DesugaringYeetExpr
            }
        }
    }
}

impl ConvertInto<types::BlockCheckMode> for hir::BlockCheckMode {
    fn convert_into(&self) -> types::BlockCheckMode {
        use rustc_hir::BlockCheckMode::*;
        use rustc_hir::UnsafeSource::*;
        match self {
            DefaultBlock => types::BlockCheckMode::DefaultBlock,
            UnsafeBlock(CompilerGenerated) => types::BlockCheckMode::UnsafeBlockCompilerGenerated,
            UnsafeBlock(UserProvided) => types::BlockCheckMode::UnsafeBlockUserProvided,
        }
    }
}

impl ConvertInto<types::Mutability> for hir::Mutability {
    fn convert_into(&self) -> types::Mutability {
        match self {
            hir::Mutability::Mut => types::Mutability::Mutable,
            hir::Mutability::Not => types::Mutability::Immutable,
        }
    }
}

impl ConvertInto<types::Constness> for hir::Constness {
    fn convert_into(&self) -> types::Constness {
        match self {
            hir::Constness::Const => types::Constness::Const,
            hir::Constness::NotConst => types::Constness::NotConst,
        }
    }
}

impl ConvertInto<types::BorrowKind> for mir::BorrowKind {
    fn convert_into(&self) -> types::BorrowKind {
        match self {
            mir::BorrowKind::Shared => types::BorrowKind::Shared,
            mir::BorrowKind::Fake(mir::FakeBorrowKind::Shallow) => types::BorrowKind::Shallow,
            mir::BorrowKind::Fake(mir::FakeBorrowKind::Deep) => types::BorrowKind::Deep,
            mir::BorrowKind::Mut { kind } => match kind {
                mir::MutBorrowKind::Default => types::BorrowKind::Mut,
                mir::MutBorrowKind::TwoPhaseBorrow => types::BorrowKind::MutTwoPhase,
                mir::MutBorrowKind::ClosureCapture => types::BorrowKind::ClosureCapture,
            },
        }
    }
}

impl ConvertInto<types::CastKind> for mir::CastKind {
    fn convert_into(&self) -> types::CastKind {
        match self {
            mir::CastKind::PointerCoercion(coercion, _source) => match coercion {
                ty::adjustment::PointerCoercion::ReifyFnPointer => types::CastKind::ReifyFnPointer,
                ty::adjustment::PointerCoercion::UnsafeFnPointer => {
                    types::CastKind::UnsafeFnPointer
                }
                ty::adjustment::PointerCoercion::ClosureFnPointer(usafety) => match usafety {
                    hir::Safety::Unsafe => types::CastKind::UnsafeClosureFnPointer,
                    hir::Safety::Safe => types::CastKind::SafeClosureFnPointer,
                },
                ty::adjustment::PointerCoercion::MutToConstPointer => {
                    types::CastKind::MutToConstPointer
                }
                ty::adjustment::PointerCoercion::ArrayToPointer => types::CastKind::ArrayToPointer,
                ty::adjustment::PointerCoercion::Unsize => types::CastKind::UnsizePointer,
                ty::adjustment::PointerCoercion::DynStar => types::CastKind::DynStar,
            },
            mir::CastKind::PointerExposeProvenance => types::CastKind::PointerExposeProvenance,
            mir::CastKind::PointerWithExposedProvenance => {
                types::CastKind::PointerWithExposedProvenance
            }
            mir::CastKind::IntToInt => types::CastKind::IntToInt,
            mir::CastKind::FloatToInt => types::CastKind::FloatToInt,
            mir::CastKind::IntToFloat => types::CastKind::IntToFloat,
            mir::CastKind::FloatToFloat => types::CastKind::FloatToFloat,
            mir::CastKind::PtrToPtr => types::CastKind::PtrToPtr,
            mir::CastKind::FnPtrToPtr => types::CastKind::FnPtrToPtr,
            mir::CastKind::Transmute => types::CastKind::Transmute,
        }
    }
}

impl<'tcx> ConvertInto<types::AggregateKind> for mir::AggregateKind<'tcx> {
    fn convert_into(&self) -> types::AggregateKind {
        match self {
            mir::AggregateKind::Array(..) => types::AggregateKind::Array,
            mir::AggregateKind::Tuple => types::AggregateKind::Tuple,
            mir::AggregateKind::Adt(..) => types::AggregateKind::Adt,
            mir::AggregateKind::Closure(..) => types::AggregateKind::Closure,
            mir::AggregateKind::Coroutine(..) => types::AggregateKind::Coroutine,
            mir::AggregateKind::CoroutineClosure(..) => types::AggregateKind::CoroutineClosure,
            mir::AggregateKind::RawPtr(..) => types::AggregateKind::RawPtr,
        }
    }
}

// TODO - mir deletion: Rename ScopeSafety and 'scope' references to blocks
impl ConvertInto<types::ScopeSafety> for Option<rustc_middle::thir::BlockSafety> {
    fn convert_into(&self) -> types::ScopeSafety {
        match self {
            Some(rustc_middle::thir::BlockSafety::Safe) => types::ScopeSafety::Safe,
            Some(rustc_middle::thir::BlockSafety::BuiltinUnsafe) => {
                types::ScopeSafety::BuiltinUnsafe
            }
            // TODO - mir deletion: Remove FnUnsafe downstream
            // Some(rustc_middle::thir::BlockSafety::FnUnsafe) => types::ScopeSafety::FnUnsafe,
            Some(rustc_middle::thir::BlockSafety::ExplicitUnsafe(_)) => {
                types::ScopeSafety::ExplicitUnsafe
            }
            None => types::ScopeSafety::Unknown,
        }
    }
}

impl ConvertInto<types::ImplPolarity> for hir::ImplPolarity {
    fn convert_into(&self) -> types::ImplPolarity {
        match self {
            hir::ImplPolarity::Positive => types::ImplPolarity::Positive,
            hir::ImplPolarity::Negative(_) => types::ImplPolarity::Negative,
        }
    }
}

impl<'tcx> ConvertInto<types::TyPrimitive> for ty::TyKind<'tcx> {
    fn convert_into(&self) -> types::TyPrimitive {
        use types::TyPrimitive::*;
        match self {
            ty::TyKind::Bool => Bool,
            ty::TyKind::Char => Char,
            ty::TyKind::Int(int_ty) => match int_ty {
                ty::IntTy::Isize => Isize,
                ty::IntTy::I8 => I8,
                ty::IntTy::I16 => I16,
                ty::IntTy::I32 => I32,
                ty::IntTy::I64 => I64,
                ty::IntTy::I128 => I128,
            },
            ty::TyKind::Uint(uint_ty) => match uint_ty {
                ty::UintTy::Usize => Usize,
                ty::UintTy::U8 => U8,
                ty::UintTy::U16 => U16,
                ty::UintTy::U32 => U32,
                ty::UintTy::U64 => U64,
                ty::UintTy::U128 => U128,
            },
            ty::TyKind::Float(float_ty) => match float_ty {
                ty::FloatTy::F16 => F16,
                ty::FloatTy::F32 => F32,
                ty::FloatTy::F64 => F64,
                ty::FloatTy::F128 => F128,
            },
            ty::TyKind::Str => Str,
            ty::TyKind::Never => Never,
            x => unreachable!("Not primitive type: {:?}", x),
        }
    }
}

impl ConvertInto<bool> for hir::IsAuto {
    fn convert_into(&self) -> bool {
        match self {
            hir::IsAuto::Yes => true,
            hir::IsAuto::No => false,
        }
    }
}

impl ConvertInto<types::Defaultness> for hir::Defaultness {
    fn convert_into(&self) -> types::Defaultness {
        match self {
            hir::Defaultness::Default { has_value } => {
                if *has_value {
                    types::Defaultness::DefaultWithValue
                } else {
                    types::Defaultness::DefaultNoValue
                }
            }
            hir::Defaultness::Final => types::Defaultness::Final,
        }
    }
}

impl ConvertInto<types::AdtKind> for ty::AdtKind {
    fn convert_into(&self) -> types::AdtKind {
        match self {
            ty::AdtKind::Struct => types::AdtKind::Struct,
            ty::AdtKind::Union => types::AdtKind::Union,
            ty::AdtKind::Enum => types::AdtKind::Enum,
        }
    }
}

impl ConvertInto<types::AdtVariantIndex> for rustc_target::abi::VariantIdx {
    fn convert_into(&self) -> types::AdtVariantIndex {
        self.index().into()
    }
}

impl ConvertInto<types::FieldIndex> for rustc_target::abi::FieldIdx {
    fn convert_into(&self) -> types::FieldIndex {
        self.index().into()
    }
}

impl ConvertInto<types::PointerCoercion> for rustc_middle::ty::adjustment::PointerCoercion {
    fn convert_into(&self) -> types::PointerCoercion {
        match self {
            PointerCoercion::ReifyFnPointer => types::PointerCoercion::ReifyFnPointer,
            PointerCoercion::UnsafeFnPointer => types::PointerCoercion::UnsafeFnPointer,
            PointerCoercion::ClosureFnPointer(_) => types::PointerCoercion::ClosreFnPointer,
            PointerCoercion::MutToConstPointer => types::PointerCoercion::MutToConstPointer,
            PointerCoercion::ArrayToPointer => types::PointerCoercion::ArrayToPointer,
            PointerCoercion::Unsize => types::PointerCoercion::Unsize,
            PointerCoercion::DynStar => types::PointerCoercion::DynStar,
        }
    }
}

impl ConvertInto<types::MatchSource> for rustc_hir::MatchSource {
    fn convert_into(&self) -> types::MatchSource {
        match self {
            rustc_hir::MatchSource::Normal => types::MatchSource::Normal,
            rustc_hir::MatchSource::Postfix => types::MatchSource::Postfix,
            rustc_hir::MatchSource::ForLoopDesugar => types::MatchSource::ForLoopDesugar,
            rustc_hir::MatchSource::TryDesugar(_) => types::MatchSource::TryDesugar,
            rustc_hir::MatchSource::AwaitDesugar => types::MatchSource::AwaitDesugar,
            rustc_hir::MatchSource::FormatArgs => types::MatchSource::FormatArgs,
        }
    }
}

impl ConvertInto<types::LitKind> for rustc_ast::ast::LitKind {
    fn convert_into(&self) -> types::LitKind {
        match self {
            rustc_ast::ast::LitKind::Str(..) => types::LitKind::Str,
            rustc_ast::ast::LitKind::ByteStr(..) => types::LitKind::ByteStr,
            rustc_ast::ast::LitKind::CStr(..) => types::LitKind::CStr,
            rustc_ast::ast::LitKind::Byte(_) => types::LitKind::Byte,
            rustc_ast::ast::LitKind::Char(_) => types::LitKind::Char,
            rustc_ast::ast::LitKind::Int(_, _) => types::LitKind::Int,
            rustc_ast::ast::LitKind::Float(_, _) => types::LitKind::Float,
            rustc_ast::ast::LitKind::Bool(_) => types::LitKind::Bool,
            rustc_ast::ast::LitKind::Err(_) => types::LitKind::Err,
        }
    }
}

impl ConvertInto<types::Movability> for Option<rustc_ast::Movability> {
    fn convert_into(&self) -> types::Movability {
        match self {
            Some(rustc_ast::Movability::Static) => types::Movability::Static,
            Some(rustc_ast::Movability::Movable) => types::Movability::Movable,
            None => types::Movability::None,
        }
    }
}
