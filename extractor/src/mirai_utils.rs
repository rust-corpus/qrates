// Copyright (c) Facebook, Inc. and its affiliates.
//
// This file is taken from
// https://raw.githubusercontent.com/facebookexperimental/MIRAI/8f294e6919ff3e61f4866593ede58851c875b49f/checker/src/utils.rs
//
// This source code is licensed under the MIT license found in
// https://github.com/facebookexperimental/MIRAI/blob/8f294e6919ff3e61f4866593ede58851c875b49f/LICENSE.

#![allow(dead_code)]

use log::debug;
use log_derive::{logfn, logfn_inputs};
use rustc_hir::def_id::DefId;
use rustc_hir::definitions::{DefPathData, DisambiguatedDefPathData};
use rustc_hir::{ItemKind, Node};
use rustc_middle::ty::subst::{GenericArgKind, SubstsRef};
use rustc_middle::ty::{self, DefIdTree, Ty, TyCtxt, TyKind};
use std::rc::Rc;

/// Returns the location of the rust system binaries that are associated with this build of Mirai.
/// The location is obtained by looking at the contents of the environmental variables that were
/// set at the time Mirai was compiled. If the rust compiler was installed by rustup, the variables
/// RUSTUP_HOME and RUSTUP_TOOLCHAIN are used and these are set by the compiler itself.
/// If the rust compiler was compiled and installed in some other way, for example from a source
/// enlistment, then the RUST_SYSROOT variable must be set in the environment from which Mirai
/// is compiled.
#[logfn_inputs(TRACE)]
pub fn find_sysroot() -> String {
    let home = option_env!("RUSTUP_HOME");
    let toolchain = option_env!("RUSTUP_TOOLCHAIN");
    match (home, toolchain) {
        (Some(home), Some(toolchain)) => format!("{}/toolchains/{}", home, toolchain),
        _ => option_env!("RUST_SYSROOT")
            .expect(
                "Could not find sysroot. Specify the RUST_SYSROOT environment variable, \
                 or use rustup to set the compiler to use for Mirai",
            )
            .to_owned(),
    }
}

/// Returns true if the function identified by def_id is a public function.
#[logfn(TRACE)]
pub fn is_public(def_id: DefId, tcx: TyCtxt<'_>) -> bool {
    if let Some(node) = tcx.hir().get_if_local(def_id) {
        let visibility = tcx.visibility(def_id);
        match node {
            Node::Expr(rustc_hir::Expr {
                kind: rustc_hir::ExprKind::Closure { .. },
                ..
            }) => {
                let parent_def_id = tcx.parent(def_id);
                is_public(parent_def_id, tcx)
            }
            Node::Item(item) => match item.kind {
                ItemKind::Fn(..) | ItemKind::Const(..) | ItemKind::Static(..) => {
                    visibility == ty::Visibility::Public
                }
                _ => {
                    debug!("def_id is unsupported item kind {:?}", item.kind);
                    false
                }
            },
            Node::ImplItem(_) => visibility == ty::Visibility::Public,
            Node::TraitItem(..) => true,
            Node::AnonConst(..) => false,
            _ => {
                debug!("def_id is not an item {:?}", node);
                false
            }
        }
    } else {
        false
    }
}

/// Returns a string that is a valid identifier, made up from the concatenation of
/// the string representations of the given list of generic argument types.
#[logfn(TRACE)]
pub fn argument_types_key_str<'tcx>(
    tcx: TyCtxt<'tcx>,
    generic_args: SubstsRef<'tcx>,
) -> Rc<String> {
    let mut result = "_".to_string();
    for generic_ty_arg in generic_args.types() {
        result.push('_');
        append_mangled_type(&mut result, generic_ty_arg, tcx);
    }
    Rc::new(result)
}

/// Appends a string to str with the constraint that it must uniquely identify ty and also
/// be a valid identifier (so that core library contracts can be written for type specialized
/// generic trait methods).
#[logfn(TRACE)]
fn append_mangled_type<'tcx>(str: &mut String, ty: Ty<'tcx>, tcx: TyCtxt<'tcx>) {
    match ty.kind() {
        TyKind::Bool => str.push_str("bool"),
        TyKind::Char => str.push_str("char"),
        TyKind::Int(int_ty) => {
            str.push_str(match int_ty {
                ty::IntTy::Isize => "isize",
                ty::IntTy::I8 => "i8",
                ty::IntTy::I16 => "i16",
                ty::IntTy::I32 => "i32",
                ty::IntTy::I64 => "i64",
                ty::IntTy::I128 => "i128",
            });
        }
        TyKind::Uint(uint_ty) => {
            str.push_str(match uint_ty {
                ty::UintTy::Usize => "usize",
                ty::UintTy::U8 => "u8",
                ty::UintTy::U16 => "u16",
                ty::UintTy::U32 => "u32",
                ty::UintTy::U64 => "u64",
                ty::UintTy::U128 => "u128",
            });
        }
        TyKind::Float(float_ty) => {
            str.push_str(match float_ty {
                ty::FloatTy::F32 => "f32",
                ty::FloatTy::F64 => "f64",
            });
        }
        TyKind::Adt(def, subs) => {
            str.push_str(qualified_type_name(tcx, def.did()).as_str());
            for sub in *subs {
                if let GenericArgKind::Type(ty) = sub.unpack() {
                    str.push('_');
                    append_mangled_type(str, ty, tcx);
                }
            }
        }
        TyKind::Closure(def_id, subs) => {
            str.push_str("closure_");
            str.push_str(qualified_type_name(tcx, *def_id).as_str());
            for sub in subs.as_closure().substs {
                if let GenericArgKind::Type(ty) = sub.unpack() {
                    str.push('_');
                    append_mangled_type(str, ty, tcx);
                }
            }
        }
        TyKind::Dynamic(..) => str.push_str(&format!("dyn_{:?}", ty)),
        TyKind::Foreign(def_id) => {
            str.push_str("extern_type_");
            str.push_str(qualified_type_name(tcx, *def_id).as_str());
        }
        TyKind::FnDef(def_id, subs) => {
            str.push_str("fn_");
            str.push_str(qualified_type_name(tcx, *def_id).as_str());
            for sub in *subs {
                if let GenericArgKind::Type(ty) = sub.unpack() {
                    str.push('_');
                    append_mangled_type(str, ty, tcx);
                }
            }
        }
        TyKind::Generator(def_id, subs, ..) => {
            str.push_str("generator_");
            str.push_str(qualified_type_name(tcx, *def_id).as_str());
            for sub in subs.as_generator().substs {
                if let GenericArgKind::Type(ty) = sub.unpack() {
                    str.push('_');
                    append_mangled_type(str, ty, tcx);
                }
            }
        }
        TyKind::GeneratorWitness(binder) => {
            for ty in binder.skip_binder().iter() {
                str.push('_');
                append_mangled_type(str, ty, tcx)
            }
        }
        TyKind::Opaque(def_id, subs) => {
            str.push_str("impl_");
            str.push_str(qualified_type_name(tcx, *def_id).as_str());
            for sub in *subs {
                if let GenericArgKind::Type(ty) = sub.unpack() {
                    str.push('_');
                    append_mangled_type(str, ty, tcx);
                }
            }
        }
        TyKind::Str => str.push_str("str"),
        TyKind::Array(ty, _) => {
            str.push_str("array_");
            append_mangled_type(str, *ty, tcx);
        }
        TyKind::Slice(ty) => {
            str.push_str("slice_");
            append_mangled_type(str, *ty, tcx);
        }
        TyKind::RawPtr(ty_and_mut) => {
            str.push_str("pointer_");
            match ty_and_mut.mutbl {
                rustc_hir::Mutability::Mut => str.push_str("mut_"),
                rustc_hir::Mutability::Not => str.push_str("const_"),
            }
            append_mangled_type(str, ty_and_mut.ty, tcx);
        }
        TyKind::Ref(_, ty, mutability) => {
            str.push_str("ref_");
            if *mutability == rustc_hir::Mutability::Mut {
                str.push_str("mut_");
            }
            append_mangled_type(str, *ty, tcx);
        }
        TyKind::FnPtr(poly_fn_sig) => {
            let fn_sig = poly_fn_sig.skip_binder();
            str.push_str("fn_ptr_");
            for arg_type in fn_sig.inputs() {
                append_mangled_type(str, *arg_type, tcx);
                str.push_str("_");
            }
            append_mangled_type(str, fn_sig.output(), tcx);
        }
        TyKind::Tuple(types) => {
            str.push_str("tuple_");
            str.push_str(&format!("{}", types.len()));
            types.iter().for_each(|t| {
                str.push('_');
                append_mangled_type(str, t, tcx);
            });
        }
        TyKind::Param(param_ty) => {
            let pty: Ty<'tcx> = param_ty.to_ty(tcx);
            if ty.eq(&pty) {
                str.push_str(&format!("{:?}", ty));
            } else {
                append_mangled_type(str, pty, tcx);
            }
        }
        TyKind::Projection(projection_ty) => {
            append_mangled_type(str, projection_ty.self_ty(), tcx);
            str.push_str("_as_");
            str.push_str(qualified_type_name(tcx, projection_ty.item_def_id).as_str());
        }
        _ => {
            //todo: add cases as the need arises, meanwhile make the need obvious.
            debug!("{:?}", ty);
            debug!("{:?}", ty.kind());
            str.push_str(&format!("default formatted {:?}", ty))
        }
    }
}

/// Pretty much the same as summary_key_str but with _ used rather than . so that
/// the result can be appended to a valid identifier.
#[logfn(TRACE)]
fn qualified_type_name(tcx: TyCtxt<'_>, def_id: DefId) -> String {
    let mut name = crate_name(tcx, def_id);
    for component in &tcx.def_path(def_id).data {
        name.push('_');
        push_component_name(component.data, &mut name);
        if component.disambiguator != 0 {
            name.push('_');
            let da = component.disambiguator.to_string();
            name.push_str(da.as_str());
        }
    }
    name
}

/// Constructs a name for the crate that contains the given def_id.
#[logfn(TRACE)]
fn crate_name(tcx: TyCtxt<'_>, def_id: DefId) -> String {
    tcx.crate_name(def_id.krate).as_str().to_string()
}

/// Constructs a string that uniquely identifies a definition to serve as a key to
/// the summary cache, which is a key value store. The string will always be the same as
/// long as the definition does not change its name or location, so it can be used to
/// transfer information from one compilation to the next, making incremental analysis possible.
#[logfn(TRACE)]
pub fn summary_key_str(tcx: TyCtxt<'_>, def_id: DefId) -> Rc<String> {
    let mut name = crate_name(tcx, def_id);
    let mut type_ns: Option<String> = None;
    for component in &tcx.def_path(def_id).data {
        if name.ends_with("foreign_contracts") {
            // By stripping off this special prefix, we allow this crate (or module) to define
            // functions that appear to be from other crates.
            // We use this to provide contracts for functions defined in crates we do not
            // wish to modify in place.
            name.clear();
        } else {
            name.push('.');
        }
        push_component_name(component.data, &mut name);
        if let DefPathData::TypeNs(sym) = component.data {
            type_ns = Some(sym.as_str().to_string());
        }
        if component.disambiguator != 0 {
            name.push('_');
            if component.data == DefPathData::Impl {
                let parent_def_id = tcx.parent(def_id);
                if let Some(type_ns) = &type_ns {
                    let def_kind = tcx.def_kind(parent_def_id);
                    use rustc_hir::def::DefKind::*;
                    if type_ns == "num"
                        && (def_kind == Struct || def_kind == Union || def_kind == Enum)
                    {
                        append_mangled_type(&mut name, tcx.type_of(parent_def_id), tcx);
                        continue;
                    }
                }
                if let Some(type_ns) = &type_ns {
                    name.push_str(&type_ns);
                    continue;
                }
            }
            let da = component.disambiguator.to_string();
            name.push_str(da.as_str());
        }
    }
    Rc::new(name)
}

/// Returns true if the first component is a module named "foreign_contracts".
pub fn is_foreign_contract(tcx: TyCtxt<'_>, def_id: DefId) -> bool {
    if let Some(DisambiguatedDefPathData {
        data: DefPathData::TypeNs(name),
        ..
    }) = &tcx.def_path(def_id).data.first()
    {
        name.as_str() == "foreign_contracts"
    } else {
        false
    }
}

fn push_component_name(component_data: DefPathData, target: &mut String) {
    use std::ops::Deref;
    use DefPathData::*;
    match component_data {
        TypeNs(name) | ValueNs(name) | MacroNs(name) | LifetimeNs(name) => {
            target.push_str(name.as_str().deref());
        }
        _ => target.push_str(match component_data {
            CrateRoot => "crate_root",
            Impl => "implement",
            ForeignMod => "foreign",
            Use => "use",
            GlobalAsm => "global_asm",
            ClosureExpr => "closure",
            Ctor => "ctor",
            AnonConst => "constant",
            ImplTrait => "implement_trait",
            _ => unreachable!(),
        }),
    };
}
