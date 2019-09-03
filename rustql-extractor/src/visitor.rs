// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

use log::{debug, info, warn};
use rustc::hir;
use rustc::hir::def_id::DefId;
use rustc::hir::intravisit::*;
use rustc::hir::intravisit::{NestedVisitorMap, Visitor};
use rustc::hir::map::Map;
use rustc::hir::HirId;
use rustc::mir;
use rustc::ty;
use rustc::ty::TyCtxt;
use rustql_common::data;
use syntax::source_map::Span;

use std::collections::BTreeMap;

pub struct CrateVisitor<'tcx, 'a> {
    pub crate_data: data::Crate,

    // Currently visited function. Used to keep relevant state as we visit the hir data structure.
    pub current_function: Option<data::Function>,

    pub map: &'a Map<'tcx>,
    pub tcx: TyCtxt<'a>,

    /// Maps DefIds of local modules to their index in the `crate_data` vector they are stored.
    pub local_modules: BTreeMap<DefId, usize>,
}

impl<'tcx, 'a> CrateVisitor<'tcx, 'a> {
    ///
    /// TODO: Is this method still relevant or can it be removed?
    /// read out type information and store it in a `data::Type`
    ///
    pub fn create_type(&self, t: &hir::Ty) -> data::Type {
        match &t.node {
            hir::TyKind::Path(hir::QPath::Resolved(_ty, path)) => {
                let res = &path.res;
                match res {
                    hir::def::Res::PrimTy(pt) => data::Type::Native(format!("{:?}", pt)),
                    hir::def::Res::Def(hir::def::DefKind::Struct, id) => {
                        let path = self.tcx.def_path(*id);
                        data::Type::Struct(data::GlobalDefPath::new(
                            path.to_string_no_crate(),
                            self.create_identifier(id.krate),
                        ))
                    }
                    _ => data::Type::Other,
                }
            }
            hir::TyKind::Rptr(_, ty) => data::Type::Reference {
                to: box self.create_type(&ty.ty),
                is_mutable: ty.mutbl == hir::Mutability::MutMutable,
            },
            _ => data::Type::Other,
        }
    }

    ///
    /// Reads out type information and returns it in a `data::Type`.
    ///
    pub fn create_type2(&self, t: &ty::Ty) -> data::Type {
        match &t.sty {
            ty::TyKind::Adt(adef, _) => {
                let path = self.tcx.def_path(adef.did);
                data::Type::Struct(data::GlobalDefPath::new(
                    path.to_string_no_crate(),
                    self.create_identifier(path.krate),
                ))
            }
            ty::TyKind::Tuple(types) => data::Type::Tuple(
                types
                    .iter()
                    .map(|t| self.create_type2(&t.expect_ty()))
                    .collect(),
            ),
            ty::TyKind::Slice(ty) | ty::TyKind::Array(ty, _ /* len */) => {
                data::Type::Slice(box self.create_type2(ty))
            }
            ty::TyKind::Ref(_, ty, mutbl) => data::Type::Reference {
                to: box self.create_type2(&ty),
                is_mutable: *mutbl == hir::Mutability::MutMutable,
            },
            _ => data::Type::Other,
        }
    }

    pub fn create_identifier(&self, c: hir::def_id::CrateNum) -> data::CrateIdentifier {
        data::CrateIdentifier {
            name: self.tcx.original_crate_name(c).as_str().to_string(),
            config_hash: self.tcx.crate_hash(c).to_string(),
        }
    }

    pub fn get_parent_index(&self, id: HirId) -> usize {
        let parent = self.map.get_module_parent(id);
        self.local_modules.get(&parent).map(|x| *x).unwrap_or(0)
    }

    pub fn get_argument_types(&self, mir: &mir::Body) -> Vec<data::Type> {
        mir.args_iter()
            .map(|l| self.create_type2(&mir.local_decls[l].ty))
            .collect::<Vec<data::Type>>()
    }

    pub fn get_return_type(&self, mir: &mir::Body) -> data::Type {
        self.create_type2(&mir.return_ty())
    }

    ///
    /// Updates `current_function` with the provided function `func` after first storing the
    /// current_function in the `crate_data` functions.
    ///
    pub fn record_current_function(&mut self, mut func: Option<data::Function>) {
        // Update self.current_function with the provided function, and store the previous
        // current function to func.
        std::mem::swap(&mut self.current_function, &mut func);

        if let Some(f) = func {
            // If the previous current function (now func) is not None.
            self.crate_data.functions.push(f);
        }
    }

    ///
    /// Reads out a function or method definition and returns a `data::Function` capturing it.
    ///
    fn create_function(
        &self,
        id: HirId,
        header: hir::FnHeader,
        fn_name: String,
        is_method: bool,
    ) -> data::Function {
        let def_id = self.map.local_def_id(id);
        let parent_index = self.get_parent_index(id);

        let def_path = self.map.def_path_from_hir_id(id).unwrap();
        let def_path_as_string = def_path.to_string_no_crate();
        let mir = self.tcx.optimized_mir(def_id);
        let argument_types = self.get_argument_types(mir);
        let return_type = self.get_return_type(mir);

        let is_method_str = if is_method { "Method" } else { "Function" };
        info!("{}: {}", is_method_str.to_string(), def_path_as_string);
        debug!("--Argument types: {:?}", argument_types);
        debug!("--Return type: {:?}", return_type);

        data::Function {
            name: fn_name,
            is_unsafe: header.unsafety == rustc::hir::Unsafety::Unsafe,
            is_const: header.constness == rustc::hir::Constness::Const,
            is_async: header.asyncness == rustc::hir::IsAsync::Async,
            abi: header.abi.name().to_owned(),
            is_closure: false,
            calls: vec![],
            containing_mod: parent_index,
            def_path: def_path_as_string,
            argument_types,
            return_type,
        }
    }
}

impl<'tcx, 'a> Visitor<'tcx> for CrateVisitor<'tcx, 'a> {
    fn visit_mod(&mut self, m: &'tcx hir::Mod, _s: Span, id: HirId) {
        if let Some(hir::Node::Item(item)) = self.map.find(id) {
            // maybe_node is None for the root module of each crate
            let def_id = self.map.local_def_id(id);
            let local_parent_index = self.get_parent_index(id);

            self.local_modules
                .insert(def_id, self.crate_data.mods.len());
            self.crate_data.mods.push(data::Mod {
                name: item.ident.name.to_string(),
                parent_mod: Some(local_parent_index),
            });
        }

        walk_mod(self, m, id);

        // Record the last visited function of the module.
        self.record_current_function(None);
    }

    fn visit_item(&mut self, item: &'tcx hir::Item) {
        use rustc::hir::ItemKind::*;
        match item.node {
            Struct(_, _) | Union(_, _) | Enum(_, _) | Ty(_, _) => {
                let def_id = self.map.local_def_id(item.hir_id);
                let ty = self.tcx.type_of(def_id);

                match ty.sty {
                    ty::TyKind::Adt(def, subs) => {
                        let mut fields: Vec<_> = vec![];
                        for var_def in def.variants.iter() {
                            fields.extend(var_def.fields.iter().map(|field| {
                                (
                                    field.ident.name.as_str().get().to_owned(),
                                    self.create_type2(&field.ty(self.tcx, subs)),
                                )
                            }));
                        }

                        self.crate_data.structs.push(data::Struct {
                            name: item.ident.name.to_string(),
                            def_path: data::GlobalDefPath::new(
                                self.tcx.def_path(def_id).to_string_no_crate(),
                                self.crate_data.metadata.clone(),
                            ),
                            fields,
                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        walk_item(self, item);
    }

    fn visit_ty(&mut self, _t: &'tcx hir::Ty) {
        //walk_ty(self, t);
        //self.crate_data.types.push(self.create_type(t));
    }

    fn visit_fn(
        &mut self,
        fk: FnKind<'tcx>,
        fd: &'tcx hir::FnDecl,
        b: hir::BodyId,
        s: Span,
        id: HirId,
    ) {
        if let Some(hir::Node::Item(item)) = self.map.find(id) {
            self.record_current_function(match fk {
                FnKind::Method(_, sig, _, _) => {
                    Some(self.create_function(id, sig.header, item.ident.name.to_string(), false))
                }
                FnKind::Closure(_attributes) => None,
                FnKind::ItemFn(_, _, header, _, _) => {
                    Some(self.create_function(id, header, item.ident.name.to_string(), false))
                }
            });
        } else {
            let def_path = self.map.def_path_from_hir_id(id).unwrap();

            debug!(
                "Function {} is not of kind Node::Item",
                def_path.to_string_no_crate()
            );
        }

        walk_fn(self, fk, fd, b, s, id);
    }

    fn visit_impl_item(&mut self, item: &'tcx hir::ImplItem) {
        match &item.node {
            hir::ImplItemKind::Method(sig, _) => {
                self.record_current_function(Some(self.create_function(
                    item.hir_id,
                    sig.header,
                    item.ident.name.to_string(),
                    true,
                )));
            }
            _ => {}
        }

        walk_impl_item(self, item);
    }

    fn visit_body(&mut self, body: &'tcx hir::Body) {
        let id = body.id();
        let owner = self.map.body_owner_def_id(id);

        debug!("Visiting body of {:?}", owner);
        debug!("{:?}", self.tcx.def_path(owner).to_string_no_crate());

        self.tcx
            .optimized_mir(owner)
            .basic_blocks()
            .iter()
            .for_each(|bblock| {
                if let Some(mir::Terminator {
                    source_info: _,
                    kind: mir::TerminatorKind::Call { func, .. },
                }) = &bblock.terminator
                {
                    if let mir::Operand::Constant(box mir::Constant {
                        literal:
                            ty::Const {
                                ty:
                                    &ty::TyS {
                                        sty: ty::FnDef(def_id, ..),
                                        ..
                                    },
                                ..
                            },
                        ..
                    }) = func
                    {
                        let krate = def_id.krate;
                        let krate_name = self.tcx.original_crate_name(krate).to_string();
                        let def_path = self.tcx.def_path(def_id).to_string_no_crate();

                        debug!("Crate name: {:?}", krate_name);
                        debug!("Definition path: {:?}", def_path);

                        if let Some(ref mut f) = self.current_function {
                            // Add the calls found in the mir code to the data of the currently
                            // visited function, i.e., the caller.
                            f.calls.push(data::GlobalDefPath {
                                crate_ident: data::CrateIdentifier {
                                    name: krate_name,
                                    config_hash: self.tcx.crate_hash(krate).to_string(),
                                },
                                def_path,
                            });
                        } else {
                            warn!("Currently not in a function. Function call ignored.");
                        }
                    } else {
                        warn!(
                            "Function doesn't match mir::Operand::Constant. Function call ignored."
                        )
                    }
                }
            });
        walk_body(self, body);
    }

    fn visit_expr(&mut self, expr: &'tcx hir::Expr) {
        /*
        if let hir::ExprKind::Call(target, args) = &expr.node {
            //println!("found call to: {:?}", target);
            use self::hir::*;
            use self::hir::def::Def;
            let target_kind = &target.node;
            if let ExprKind::Path(QPath::Resolved(_, p)) = target_kind {
                if let Some(ref mut f) = self.current_function {
                    //let def_id = std::panic::catch_unwind(|| p.def.def_id());
                    let def_id = match p.def {
                        Def::Fn(id) |
                        Def::Variant(id) |
                        Def::StructCtor(id, _) |
                        Def::VariantCtor(id, _) |
                        Def::SelfCtor(id) |
                        Def::Method(id)
                        => Some(id),
                        Def::Local(node_id) |
                        Def::Upvar(node_id, _, _)
                        => { println!("semi-unknown call to {}", p.def.kind_name()); None }//Some(self.map.local_def_id(node_id)) },
                        _ => { println!("unknown call to {}", p.def.kind_name()); None }
                    };

                    if let Some(id) = def_id {
                        if (id.is_local()) {
                            //println!("DefPath of call: {}", self.map.def_path(id).to_string_no_crate());
                            f.calls.push(data::GlobalDefPath::new(self.map.def_path(id).to_string_no_crate(), &self.crate_data.metadata));
                        } else {
                            f.calls.push(data::GlobalDefPath::new(self.tcx.def_path(id).to_string_no_crate(), &self.crate_data.metadata));
                            //println!("non-local call detected: {:?}", p);
                        }
                    }
                    //f.calls.push(data::GlobalDefPath{ path: p.path, crate_ident: self.crate_data.metadata });
                }
                //println!("def id: {:?}", p.def);
                /*if let Some(id) = self.crate_data.get_function(p.def.def_id()) {
                    println!("found func: {:?} ", id);
                }*/
        }
        else {
        println!("call to function with unrecognized path.");
        }
        }
        else if let hir::ExprKind::MethodCall(path_seg, span, args) = &expr.node {
        let slf = &args[0];
        let method = path_seg;

        println!("unrecognized method call {:?}", path_seg);
        }*/
        walk_expr(self, expr);
    }

    fn nested_visit_map<'this>(&'this mut self) -> NestedVisitorMap<'this, 'tcx> {
        NestedVisitorMap::All(self.map)
    }
}
