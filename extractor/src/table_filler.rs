// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

use crate::converters::ConvertInto;
use crate::mirai_utils;
use corpus_database::{tables::Tables, types};
use rustc_hir::{self as hir, HirId};
use rustc_middle::hir::map::Map as HirMap;
use rustc_middle::ty::{self, TyCtxt};
use rustc_session::Session;
use rustc_span::hygiene::ExpnKind;
use rustc_span::{Pos, Span};
use std::collections::HashMap;

/// A wrapper around `Tables` that keeps some local state.
pub(crate) struct TableFiller<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,
    hir_map: &'a HirMap<'tcx>,
    session: &'a Session,
    pub(crate) tables: Tables,
    span_registry: HashMap<Span, types::Span>,
    type_registry: HashMap<ty::Ty<'tcx>, types::Type>,
}

impl<'a, 'tcx> TableFiller<'a, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        hir_map: &'a HirMap<'tcx>,
        session: &'a Session,
        tables: Tables,
    ) -> Self {
        Self {
            tcx,
            hir_map,
            session,
            tables,
            span_registry: HashMap::new(),
            type_registry: HashMap::new(),
        }
    }
    pub fn resolve_hir_id(&mut self, id: HirId) -> types::DefPath {
        let def_id = self.hir_map.local_def_id(id);
        self.resolve_local_def_id(def_id)
    }
    pub fn resolve_local_def_id(
        &mut self,
        local_def_id: rustc_span::def_id::LocalDefId,
    ) -> types::DefPath {
        self.resolve_def_id(local_def_id.to_def_id())
    }
    pub fn resolve_def_id(&mut self, def_id: hir::def_id::DefId) -> types::DefPath {
        let crate_num = def_id.krate;
        let crate_name = &self.tcx.crate_name(crate_num).as_str();
        let crate_hash = self.tcx.crate_hash(crate_num).as_u64().into();
        let def_path_str = self.tcx.def_path_debug_str(def_id);
        let def_path_hash = self.tcx.def_path_hash(def_id).0.as_value().into();
        let summary_key_str = mirai_utils::summary_key_str(self.tcx, def_id);
        let summary_key_str_value = std::rc::Rc::try_unwrap(summary_key_str).unwrap();
        let def_path = self.tables.register_def_paths(
            crate_name.to_string(),
            crate_hash,
            def_path_str,
            def_path_hash,
            summary_key_str_value,
        );
        if def_id.is_local() {
            // This will panic if def_id is non-local
            let def_span = self.tcx.def_span(def_id);
            let span = self.register_span(def_span);
            self.tables.register_def_path_span(def_path, span);
        }
        def_path
    }
    pub fn register_span(&mut self, span: Span) -> types::Span {
        if self.span_registry.contains_key(&span) {
            self.span_registry[&span]
        } else {
            let source_map = self.session.source_map();
            let location = source_map.lookup_char_pos(span.lo());
            let expansion_data = span.ctxt().outer_expn_data();
            let call_site_span = if let ExpnKind::Root = expansion_data.kind {
                self.tables.get_root_parent_span()
            } else {
                self.register_span(expansion_data.call_site)
            };
            let (interned_span,) = self.tables.register_spans(
                call_site_span,
                expansion_data.kind.convert_into(),
                expansion_data.kind.descr().to_string(),
                location.file.name.to_string(),
                location.line as u16,
                location.col.to_usize() as u16,
            );
            if let ExpnKind::Macro(_, symbol) = expansion_data.kind {
                let def_site_location = source_map.lookup_char_pos(expansion_data.def_site.lo());
                self.tables.register_macro_expansions(
                    interned_span,
                    symbol.to_string(),
                    def_site_location.file.name.to_string(),
                    def_site_location.line as u16,
                    def_site_location.col.to_usize() as u16,
                );
            }
            self.span_registry.insert(span, interned_span);
            interned_span
        }
    }
    fn insert_new_type_into_table(&mut self, kind: &str, typ: ty::Ty<'tcx>) -> types::Type {
        assert!(!self.type_registry.contains_key(&typ));
        let (interned_type,) = self.tables.register_types(kind.to_string());
        self.type_registry.insert(&typ, interned_type);
        interned_type
    }
    pub fn register_type(&mut self, typ: ty::Ty<'tcx>) -> types::Type {
        let result = if let Some(interned_type) = self.type_registry.get(&typ) {
            *interned_type
        } else {
            assert!(!self.type_registry.contains_key(&typ));
            match typ.kind() {
                ty::TyKind::Bool
                | ty::TyKind::Char
                | ty::TyKind::Int(_)
                | ty::TyKind::Uint(_)
                | ty::TyKind::Float(_)
                | ty::TyKind::Str
                | ty::TyKind::Never => {
                    let primitive_kind = typ.kind().convert_into();
                    let interned_type =
                        self.insert_new_type_into_table(&primitive_kind.to_string(), typ);
                    self.tables
                        .register_types_primitive(interned_type, primitive_kind);
                    interned_type
                }
                ty::TyKind::Adt(adt_def, substs) => {
                    let interned_type = self.insert_new_type_into_table("Adt", typ);
                    let adt_def_path = self.resolve_def_id(adt_def.did);
                    self.tables.register_types_adt_def(
                        interned_type,
                        adt_def_path,
                        adt_def.adt_kind().convert_into(),
                        adt_def.repr.c(),
                        adt_def.is_phantom_data(),
                    );
                    for (i, variant) in adt_def.variants.iter_enumerated() {
                        let variant_def_path = self.resolve_def_id(variant.def_id);
                        let variant_index = i.convert_into();
                        self.tables.register_types_adt_variant(
                            interned_type,
                            variant_index,
                            variant_def_path,
                            variant.ident.to_string(),
                        );
                        for field in &variant.fields {
                            let field_def_path = self.resolve_def_id(field.did);
                            let field_type = self.register_type(field.ty(self.tcx, substs));
                            let (field_id,) = self.tables.register_types_adt_field(
                                interned_type,
                                variant_index,
                                field_def_path,
                                field.ident.to_string(),
                                field.vis.convert_into(),
                                field_type,
                            );
                            if let ty::Visibility::Restricted(module) = field.vis {
                                let module_def_path = self.resolve_def_id(module);
                                self.tables
                                    .register_types_adt_field_visible_in(field_id, module_def_path);
                            }
                        }
                    }
                    interned_type
                }
                ty::TyKind::Foreign(def_id) => {
                    let interned_type = self.insert_new_type_into_table("Foreign", typ);
                    let foreign_def_path = self.resolve_def_id(*def_id);
                    self.tables
                        .register_types_foreign(interned_type, foreign_def_path);
                    interned_type
                }
                ty::TyKind::Array(element_type, _len) => {
                    let interned_type = self.insert_new_type_into_table("Array", typ);
                    let element_interned_type = self.register_type(element_type);
                    self.tables
                        .register_types_array(interned_type, element_interned_type);
                    interned_type
                }
                ty::TyKind::Slice(element_type) => {
                    let interned_type = self.insert_new_type_into_table("Slice", typ);
                    let element_interned_type = self.register_type(element_type);
                    self.tables
                        .register_types_slice(interned_type, element_interned_type);
                    interned_type
                }
                ty::TyKind::RawPtr(ty::TypeAndMut { ty, mutbl }) => {
                    let interned_type = self.insert_new_type_into_table("RawPtr", typ);
                    let target_type = self.register_type(ty);
                    self.tables.register_types_raw_ptr(
                        interned_type,
                        target_type,
                        mutbl.convert_into(),
                    );
                    interned_type
                }
                ty::TyKind::Ref(_region, ty, mutbl) => {
                    let interned_type = self.insert_new_type_into_table("Ref", typ);
                    let target_type = self.register_type(ty);
                    self.tables.register_types_ref(
                        interned_type,
                        target_type,
                        mutbl.convert_into(),
                    );
                    interned_type
                }
                ty::TyKind::FnDef(def_id, _substs) => {
                    let interned_type = self.insert_new_type_into_table("FnDef", typ);
                    let fn_def_path = self.resolve_def_id(*def_id);
                    self.tables
                        .register_types_fn_def(interned_type, fn_def_path);
                    interned_type
                }
                ty::TyKind::FnPtr(_fn_sig) => {
                    let interned_type = self.insert_new_type_into_table("FnPtr", typ);
                    self.tables.register_types_fn_ptr(interned_type);
                    interned_type
                }
                ty::TyKind::Dynamic(binder, _region) => {
                    let interned_type = self.insert_new_type_into_table("Dynamic", typ);
                    self.tables.register_types_dynamic(interned_type);
                    for predicate in binder.skip_binder().iter() {
                        match predicate {
                            ty::ExistentialPredicate::Trait(trait_ref) => {
                                let def_path = self.resolve_def_id(trait_ref.def_id);
                                self.tables.register_types_dynamic_trait(
                                    interned_type,
                                    def_path,
                                    false,
                                );
                            }
                            ty::ExistentialPredicate::Projection(_projection) => {
                                // TODO
                            }
                            ty::ExistentialPredicate::AutoTrait(def_id) => {
                                let def_path = self.resolve_def_id(def_id);
                                self.tables.register_types_dynamic_trait(
                                    interned_type,
                                    def_path,
                                    true,
                                );
                            }
                        }
                    }
                    interned_type
                }
                ty::TyKind::Closure(def_id, _substs) => {
                    let interned_type = self.insert_new_type_into_table("Closure", typ);
                    let closure_def_path = self.resolve_def_id(*def_id);
                    self.tables
                        .register_types_closure(interned_type, closure_def_path);
                    interned_type
                }
                ty::TyKind::Generator(def_id, _substs, _movability) => {
                    let interned_type = self.insert_new_type_into_table("Generator", typ);
                    let generator_def_path = self.resolve_def_id(*def_id);
                    self.tables
                        .register_types_generator(interned_type, generator_def_path);
                    interned_type
                }
                ty::TyKind::GeneratorWitness(_binder) => {
                    let interned_type = self.insert_new_type_into_table("GeneratorWitness", typ);
                    self.tables.register_types_generator_witness(interned_type);
                    interned_type
                }
                ty::TyKind::Tuple(_substs) => {
                    let interned_type = self.insert_new_type_into_table("Tuple", typ);
                    self.tables.register_types_tuple(interned_type);
                    for (i, field_type) in typ.tuple_fields().enumerate() {
                        let element_type = self.register_type(field_type);
                        self.tables.register_types_tuple_element(
                            interned_type,
                            i.into(),
                            element_type,
                        );
                    }
                    interned_type
                }
                ty::TyKind::Projection(projection) => {
                    let interned_type = self.insert_new_type_into_table("Projection", typ);
                    let trait_def_id = projection.trait_ref(self.tcx).def_id;
                    let trait_def_path = self.resolve_def_id(trait_def_id);
                    let trait_item = self.resolve_def_id(projection.item_def_id);
                    self.tables.register_types_projection(
                        interned_type,
                        trait_def_path,
                        trait_item,
                    );
                    interned_type
                }
                ty::TyKind::Opaque(def_id, _substs) => {
                    let interned_type = self.insert_new_type_into_table("Opaque", typ);
                    let def_path = self.resolve_def_id(*def_id);
                    self.tables.register_types_opaque(interned_type, def_path);
                    interned_type
                }
                ty::TyKind::Param(param_ty) => {
                    let interned_type = self.insert_new_type_into_table("Param", typ);
                    self.tables.register_types_param(
                        interned_type,
                        param_ty.index,
                        param_ty.name.to_string(),
                    );
                    interned_type
                }
                ty::TyKind::Bound(..)
                | ty::TyKind::Placeholder(_)
                | ty::TyKind::Infer(_)
                | ty::TyKind::Error(_) => {
                    unreachable!();
                }
            }
        };
        result
    }
}
