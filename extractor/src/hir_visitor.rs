// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

use crate::converters::ConvertInto;
use crate::mir_visitor::MirVisitor;
use crate::table_filler::TableFiller;
use corpus_database::{tables::Tables, types};
use rustc_hir::{
    self as hir,
    intravisit::{self, Visitor},
    HirId,
};
use rustc_middle::hir::map::Map as HirMap;
use rustc_middle::mir::{self, HasLocalDecls};
use rustc_middle::ty::TyCtxt;
use rustc_session::Session;
use rustc_span::source_map::Span;
use std::mem;

pub(crate) struct HirVisitor<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,
    hir_map: &'a HirMap<'tcx>,
    filler: TableFiller<'a, 'tcx>,
    current_module: types::Module,
    current_item: Option<types::Item>,
}

impl<'a, 'tcx> HirVisitor<'a, 'tcx> {
    pub fn new(
        mut tables: Tables,
        build: types::Build,
        session: &'a Session,
        hir_map: &'a HirMap<'tcx>,
        tcx: TyCtxt<'tcx>,
    ) -> Self {
        let (root_module,) = tables.register_root_modules(build);
        let filler = TableFiller::new(tcx, hir_map, session, tables);
        Self {
            tcx,
            hir_map,
            filler,
            current_module: root_module,
            current_item: None,
        }
    }
    pub fn filler(self) -> TableFiller<'a, 'tcx> {
        self.filler
    }
    fn visit_submodule(
        &mut self,
        def_id: types::DefPath,
        name: &str,
        visibility: types::Visibility,
        module: &'tcx hir::Mod,
        id: HirId,
    ) {
        let parent_module = self.current_module;
        let (new_module,) = self.filler.tables.register_submodules(
            def_id,
            parent_module,
            name.to_string(),
            visibility,
            String::from("NONE"),
        );
        self.current_module = new_module;
        intravisit::walk_mod(self, module, id);
        self.current_module = parent_module;
    }
    fn visit_foreign_submodule(
        &mut self,
        def_id: types::DefPath,
        name: &str,
        visibility: types::Visibility,
        module: &'tcx hir::ForeignMod,
        id: HirId,
    ) {
        let parent_module = self.current_module;
        let (new_module,) = self.filler.tables.register_submodules(
            def_id,
            parent_module,
            name.to_string(),
            visibility,
            module.abi.to_string(),
        );
        self.current_module = new_module;
        self.visit_id(id);
        rustc_ast::walk_list!(self, visit_foreign_item, module.items);
        self.current_module = parent_module;
    }
    fn visit_static(
        &mut self,
        def_id: types::DefPath,
        name: &str,
        visibility: types::Visibility,
        mutability: types::Mutability,
        typ: &'tcx hir::Ty,
        id: HirId,
        body_id: hir::BodyId,
    ) {
        let (item,) = self.filler.tables.register_static_definitions(
            def_id,
            self.current_module,
            name.to_string(),
            visibility,
            mutability,
        );
        let old_item = mem::replace(&mut self.current_item, Some(item));
        self.visit_id(id);
        self.visit_ty(typ);
        self.visit_nested_body(body_id);
        self.current_item = old_item;
    }
    /// Extract information from unoptmized MIR.
    fn visit_mir(&mut self, body_id: rustc_span::def_id::LocalDefId, body: &mir::Body<'tcx>) {
        let error = format!("Mir outside of an item: {:?}", body.span);
        let item = self.current_item.expect(&error);
        let mut mir_visitor = MirVisitor::new(self.tcx, item, body_id, body, &mut self.filler);
        mir_visitor.visit();
    }
    fn visit_type(
        &mut self,
        item: &'tcx hir::Item,
        def_path: types::DefPath,
        def_id: rustc_span::def_id::LocalDefId,
        name: &str,
        visibility: types::Visibility,
        kind: types::TyDefKind,
    ) {
        let typ = self.filler.register_type(self.tcx.type_of(def_id));
        let (item_id,) = self.filler.tables.register_type_defs(
            typ,
            def_path,
            name.to_string(),
            visibility,
            kind,
        );
        let old_item = mem::replace(&mut self.current_item, Some(item_id));
        intravisit::walk_item(self, item);
        self.current_item = old_item;
    }
    /// Retrieves the parameter types and the return type for the function with `def_id`.
    fn get_fn_param_and_return_types(&mut self, id: HirId) -> (Vec<types::Type>, types::Type) {
        let def_id = self.hir_map.local_def_id(id);
        let mir = self.tcx.optimized_mir(def_id);
        let return_type = self.filler.register_type(mir.return_ty());
        let local_decls = mir.local_decls();
        let param_types = mir
            .args_iter()
            .map(|param| self.filler.register_type(local_decls[param].ty))
            .collect();
        (param_types, return_type)
    }
}

impl<'a, 'tcx> Visitor<'tcx> for HirVisitor<'a, 'tcx> {
    type Map = HirMap<'tcx>;
    fn visit_item(&mut self, item: &'tcx hir::Item) {
        let name: &str = &item.ident.name.as_str();
        let visibility: types::Visibility = item.vis.convert_into();
        let def_path = self.filler.resolve_hir_id(item.hir_id);
        let def_id = self.hir_map.local_def_id(item.hir_id);
        match &item.kind {
            hir::ItemKind::Mod(ref module) => {
                // This avoids visiting the root module.
                self.visit_submodule(def_path, name, visibility, module, item.hir_id);
            }
            hir::ItemKind::ForeignMod(ref module) => {
                self.visit_foreign_submodule(def_path, name, visibility, module, item.hir_id);
            }
            hir::ItemKind::Static(ref typ, ref mutability, body_id) => {
                self.visit_static(
                    def_path,
                    name,
                    visibility,
                    mutability.convert_into(),
                    typ,
                    item.hir_id,
                    *body_id,
                );
            }
            hir::ItemKind::Const(ref typ, body_id) => {
                self.visit_static(
                    def_path,
                    name,
                    visibility,
                    types::Mutability::Const,
                    typ,
                    item.hir_id,
                    *body_id,
                );
            }
            hir::ItemKind::Impl {
                unsafety,
                polarity,
                defaultness,
                constness,
                of_trait,
                ..
            } => {
                let interned_type = self.filler.register_type(self.tcx.type_of(def_id));
                let (item_id,) = self.filler.tables.register_impl_definitions(
                    def_path,
                    self.current_module,
                    name.to_string(),
                    visibility,
                    unsafety.convert_into(),
                    polarity.convert_into(),
                    defaultness.convert_into(),
                    constness.convert_into(),
                    interned_type,
                );
                if let Some(trait_ref) = of_trait {
                    if let Some(trait_def_id) = trait_ref.trait_def_id() {
                        let trait_def_path = self.filler.resolve_def_id(trait_def_id);
                        self.filler.tables.register_trait_impls(
                            item_id,
                            interned_type,
                            trait_def_path,
                        );
                    }
                }
                let old_item = mem::replace(&mut self.current_item, Some(item_id));
                intravisit::walk_item(self, item);
                self.current_item = old_item;
            }
            hir::ItemKind::GlobalAsm(_) => {
                self.filler.tables.register_global_asm_blocks(
                    def_path,
                    self.current_module,
                    name.to_string(),
                    visibility,
                );
            }
            hir::ItemKind::TyAlias(..) => self.visit_type(
                item,
                def_path,
                def_id,
                name,
                visibility,
                types::TyDefKind::TyAlias,
            ),
            hir::ItemKind::OpaqueTy(..) => self.visit_type(
                item,
                def_path,
                def_id,
                name,
                visibility,
                types::TyDefKind::OpaqueTy,
            ),
            hir::ItemKind::Enum(..) => self.visit_type(
                item,
                def_path,
                def_id,
                name,
                visibility,
                types::TyDefKind::Enum,
            ),
            hir::ItemKind::Struct(..) => self.visit_type(
                item,
                def_path,
                def_id,
                name,
                visibility,
                types::TyDefKind::Struct,
            ),
            hir::ItemKind::Union(..) => self.visit_type(
                item,
                def_path,
                def_id,
                name,
                visibility,
                types::TyDefKind::Union,
            ),
            hir::ItemKind::Trait(is_auto, unsafety, _, _, trait_items) => {
                let is_marker = self.tcx.trait_def(def_id).is_marker;
                let (item_id,) = self.filler.tables.register_traits(
                    def_path,
                    name.to_string(),
                    visibility,
                    is_auto.convert_into(),
                    is_marker,
                    unsafety.convert_into(),
                );
                for trait_item in *trait_items {
                    let trait_item_def_path = self.filler.resolve_hir_id(trait_item.id.hir_id);
                    self.filler.tables.register_trait_items(
                        item_id,
                        trait_item_def_path,
                        trait_item.defaultness.convert_into(),
                    )
                }
                let old_item = mem::replace(&mut self.current_item, Some(item_id));
                intravisit::walk_item(self, item);
                self.current_item = old_item;
            }
            hir::ItemKind::ExternCrate(_)
            | hir::ItemKind::Use(_, _)
            | hir::ItemKind::Fn(_, _, _)
            | hir::ItemKind::TraitAlias(_, _) => {
                let (item_id,) = self.filler.tables.register_items(
                    def_path,
                    self.current_module,
                    name.to_string(),
                    visibility,
                );
                let old_item = mem::replace(&mut self.current_item, Some(item_id));
                intravisit::walk_item(self, item);
                self.current_item = old_item;
            }
        }
    }
    fn visit_fn(
        &mut self,
        fn_kind: intravisit::FnKind<'tcx>,
        fn_def: &'tcx hir::FnDecl,
        body_id: hir::BodyId,
        span: Span,
        id: HirId,
    ) {
        let def_path = self.filler.resolve_hir_id(id);
        let (param_types, return_type) = self.get_fn_param_and_return_types(id);
        let (function,) = match fn_kind {
            intravisit::FnKind::Method(_name, method_sig, visibility, _attributes) => {
                self.filler.tables.register_function_definitions(
                    def_path,
                    self.current_module,
                    visibility.convert_into(),
                    method_sig.header.unsafety.convert_into(),
                    method_sig.header.abi.name().to_string(),
                    return_type,
                )
            }
            intravisit::FnKind::ItemFn(_name, _generics, header, visibility, _block) => {
                self.filler.tables.register_function_definitions(
                    def_path,
                    self.current_module,
                    visibility.convert_into(),
                    header.unsafety.convert_into(),
                    header.abi.name().to_string(),
                    return_type,
                )
            }
            intravisit::FnKind::Closure(_) => self.filler.tables.register_function_definitions(
                def_path,
                self.current_module,
                types::Visibility::Unknown,
                types::Unsafety::Unknown,
                "Closure".to_string(),
                return_type,
            ),
        };
        let old_item = mem::replace(&mut self.current_item, Some(function));
        intravisit::walk_fn(self, fn_kind, fn_def, body_id, span, id);
        self.current_item = old_item;
        for (i, param_type) in param_types.into_iter().enumerate() {
            self.filler
                .tables
                .register_function_parameter_types(function, i.into(), param_type);
        }
    }
    fn visit_foreign_item(&mut self, item: &'tcx hir::ForeignItem) {
        let def_path = self.filler.resolve_hir_id(item.hir_id);
        let visibility = item.vis.convert_into();
        let item_id = match item.kind {
            hir::ForeignItemKind::Fn(..) => {
                let def_id = self.hir_map.local_def_id(item.hir_id);
                let fn_sig = self.tcx.fn_sig(def_id);
                let fn_sig = fn_sig.skip_binder();
                let return_type = self.filler.register_type(fn_sig.output());
                let (function,) = self.filler.tables.register_function_definitions(
                    def_path,
                    self.current_module,
                    visibility,
                    types::Unsafety::Unsafe,
                    "ForeignItem".to_string(),
                    return_type,
                );
                for (i, input) in fn_sig.inputs().iter().enumerate() {
                    let param_type = self.filler.register_type(input);
                    self.filler.tables.register_function_parameter_types(
                        function,
                        i.into(),
                        param_type,
                    );
                }
                Some(function)
            }
            hir::ForeignItemKind::Static(_, mutability) => {
                let name: &str = &item.ident.name.as_str();
                let (item,) = self.filler.tables.register_static_definitions(
                    def_path,
                    self.current_module,
                    name.to_string(),
                    visibility,
                    mutability.convert_into(),
                );
                Some(item)
            }
            hir::ForeignItemKind::Type => None,
        };
        if let Some(item_id) = item_id {
            let old_item = mem::replace(&mut self.current_item, Some(item_id));
            intravisit::walk_foreign_item(self, item);
            self.current_item = old_item;
        } else {
            intravisit::walk_foreign_item(self, item);
        }
    }
    fn visit_body(&mut self, body: &'tcx hir::Body) {
        intravisit::walk_body(self, body);
        let id = body.id();
        let owner = self.hir_map.body_owner_def_id(id);
        let mir_body = self.tcx.optimized_mir(owner);
        self.visit_mir(owner, mir_body);
    }
    fn nested_visit_map<'this>(&'this mut self) -> intravisit::NestedVisitorMap<Self::Map> {
        intravisit::NestedVisitorMap::All(self.tcx.hir())
    }
}
