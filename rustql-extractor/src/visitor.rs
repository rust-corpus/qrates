// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

use log::info;
use rustc::hir;
use rustc::hir::HirId;
use rustc::hir::def_id::DefId;
use rustc::hir::intravisit::*;
use rustc::hir::intravisit::{NestedVisitorMap, Visitor};
use rustc::hir::map::Map;
use rustc::mir;
use rustc::ty;
use rustc::ty::TyCtxt;
use rustql_common::data;
use syntax::source_map::Span;

use std::collections::BTreeMap;

pub struct CrateVisitor<'tcx, 'a> {
    pub crate_data: data::Crate,
    pub current_function: Option<data::Function>,
    pub map: &'a Map<'tcx>,
    pub tcx: TyCtxt<'a>,

    /// maps DefIds of local modules to their index in `crate_data`
    pub local_modules: BTreeMap<DefId, usize>,
}

impl<'tcx, 'a> CrateVisitor<'tcx, 'a> {
    ///
    /// read out type information and store it in a `data::Type`
    ///
    pub fn create_type(&self, t: &hir::Ty) -> data::Type {
        // LocalDecl
        // use mir::args_iter()
        //
        // rustc::ty::TyS

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
            /*hir::TyKind::Ptr(ty) |*/
            hir::TyKind::Rptr(_, ty) => data::Type::Reference {
                to: box self.create_type(&ty.ty),
                is_mutable: ty.mutbl == hir::Mutability::MutMutable,
            },
            _ => data::Type::Other,
        }
    }

    ///
    /// read out type information and store it in a `data::Type`
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
            ty::TyKind::Tuple(types) => {
                data::Type::Tuple(types.iter().map(|t| self.create_type2(&t.expect_ty())).collect())
            }
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
}

impl<'tcx, 'a> Visitor<'tcx> for CrateVisitor<'tcx, 'a> {
    fn visit_mod(&mut self, m: &'tcx hir::Mod, _s: Span, id: HirId) {
        let maybe_node = self.map.find(id);
        if let Some(hir::Node::Item(item)) = maybe_node {
            // maybe_node is None for the root module of each crate
            let name: &str = &item.ident.name.as_str();
            let def_id = self.map.local_def_id(id);
            let parent = self.map.get_module_parent(id);
            let local_parent_index = self.local_modules.get(&parent).map(|x| *x).unwrap_or(0);

            self.local_modules
                .insert(def_id, self.crate_data.mods.len());
            self.crate_data.mods.push(data::Mod {
                name: String::from(name),
                parent_mod: Some(local_parent_index),
            });
        }

        walk_mod(self, m, id);

        let mut func: Option<data::Function> = None;
        std::mem::swap(&mut self.current_function, &mut func);
        if let Some(f) = func {
            self.crate_data.functions.push(f);
        }
    }

    fn visit_item(&mut self, item: &'tcx hir::Item) {
        use rustc::hir::ItemKind::*;
        match item.node {
            Struct(_, _) | Union(_, _) | Enum(_, _) | Ty(_, _) => {
                let def_id = self.map.local_def_id(item.hir_id);
                let ty = self.tcx.type_of(def_id);

                let _my_ty = self.create_type2(&ty);

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
                        let path = self.tcx.def_path(self.map.local_def_id(item.hir_id));
                        self.crate_data.structs.push(data::Struct {
                            name: item.ident.name.to_string(),
                            def_path: data::GlobalDefPath::new(
                                path.to_string_no_crate(),
                                self.crate_data.metadata.clone(),
                            ),
                            fields: fields,
                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        /*
        if let hir::ItemKind::Struct(var_data, generics) = &item.node {
            let fields: Vec<_> = match var_data {
                hir::VariantData::Struct(fields, node_id) |
                    hir::VariantData::Tuple(fields, node_id) => {fields.iter().map(|sf| (sf.ident.name.as_str().get().to_owned(), self.create_type(&sf.ty))).collect()},
                _ => {vec![]}
            };
            println!("struct found: {:?}", item.name);
            let path = self.tcx.def_path(self.map.local_def_id(item.id));
            self.crate_data.structs.push(data::Struct {
                name: item.name.to_string(),
                def_path: data::GlobalDefPath::new(path.to_string_no_crate(), self.crate_data.metadata.clone()),
                fields: fields
            });
        }*/
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
        let def_id = self.map.local_def_id(id);
        //let def = self.tcx.absolute_item_path_str(def_id);

        let def_path = self.map.def_path_from_hir_id(id).unwrap();

        let parent = self.map.get_module_parent(id);
        let local_parent_index = self.local_modules.get(&parent).map(|x| *x).unwrap_or(0);

        let maybe_node = self.map.find(id);
        let mir = self.tcx.optimized_mir(def_id);

        //let argument_types = fd.inputs.iter().map(|t| self.create_type(t)).collect::<Vec<_>>();
        let argument_types = mir
            .args_iter()
            .map(|local| self.create_type2(&mir.local_decls[local].ty))
            .collect::<Vec<_>>();

        //println!("{:?}", mir.args_iter().collect::<Vec<_>>());

        //println!("{:?}", argument_types2);

        let return_type = self.create_type2(&mir.return_ty());
        /*let return_type = match &fd.output {
            hir::FunctionRetTy::DefaultReturn(_) => { println!("default return type not supported"); data::Type::Other },
            hir::FunctionRetTy::Return(pty) => { self.create_type(&pty) }
        };*/

        let mut add_function = |mut func| {
            std::mem::swap(&mut self.current_function, &mut func);
            if let Some(f) = func {
                self.crate_data.functions.push(f);
            }
        };

        if let Some(hir::Node::Item(item)) = maybe_node {
            info!("Function: {}", def_path.to_string_no_crate());
            info!("--Function argument types: {:?}", argument_types);

            add_function(match fk {
                FnKind::Method(_name, method_sig, _vis, _attr) => {
                    Option::Some(data::Function {
                        name: item.ident.name.to_string(),
                        is_unsafe: method_sig.header.unsafety == rustc::hir::Unsafety::Unsafe,
                        is_const: method_sig.header.constness == rustc::hir::Constness::Const,
                        is_async: method_sig.header.asyncness == rustc::hir::IsAsync::Async,
                        abi: method_sig.header.abi.name().to_owned(),
                        is_closure: false,
                        calls: vec![],
                        containing_mod: local_parent_index,
                        def_path: def_path.to_string_no_crate(),
                        argument_types: argument_types,
                        return_type: return_type, //def_id: //data::DefIdWrapper(def_id)
                    })
                }
                FnKind::Closure(_attributes) => {
                    None
                    //Option::Some(
                    //    data::Function {

                    //    }
                    //)
                }
                FnKind::ItemFn(_name, _generics, header, _vis, _block) => {
                    Option::Some(data::Function {
                        name: item.ident.name.to_string(),
                        is_unsafe: header.unsafety == rustc::hir::Unsafety::Unsafe,
                        is_const: header.constness == rustc::hir::Constness::Const,
                        is_async: header.asyncness == rustc::hir::IsAsync::Async,
                        abi: header.abi.name().to_owned(),
                        is_closure: false,
                        calls: vec![],
                        //containing_mod: Some(def_path),
                        containing_mod: local_parent_index, //Some(data::GlobalDefPath::new(&def_path, &self.crate_data.metadata)),
                        def_path: def_path.to_string_no_crate(),
                        argument_types: argument_types,
                        return_type: return_type, //def_id: //data::DefIdWrapper(def_id)
                    })
                }
            });
        } else {
            info!("Function {} is not of kind Node::Item", def_path.to_string_no_crate());
        }
        walk_fn(self, fk, fd, b, s, id);
    }

    fn visit_impl_item(&mut self, ii: &'tcx hir::ImplItem) {
        //println!("visited impl item: {:?}", ii);

        let def_id = self.map.local_def_id(ii.hir_id);
        let def_path = self.map.def_path_from_hir_id(ii.hir_id).unwrap();
        let parent = self.map.get_module_parent(ii.hir_id);
        //info!("parent: {:?}", parent);
        let local_parent_index = self.local_modules.get(&parent).map(|x| *x).unwrap_or(0);

        match &ii.node {
            hir::ImplItemKind::Method(sig, _body_id) => {
                let mir = self.tcx.optimized_mir(def_id);
                let argument_types = mir
                    .args_iter()
                    .map(|local| self.create_type2(&mir.local_decls[local].ty))
                    .collect::<Vec<_>>();
                let return_type = self.create_type2(&mir.return_ty());
                /*let return_type = match &sig.decl.output {
                    hir::FunctionRetTy::DefaultReturn(_) => { println!("default return type not supported"); data::Type::Other },
                    hir::FunctionRetTy::Return(pty) => { self.create_type(&pty) }
                };*/
                let mut func = Option::Some(data::Function {
                    name: ii.ident.to_string(),
                    is_unsafe: sig.header.unsafety == rustc::hir::Unsafety::Unsafe,
                    is_const: sig.header.constness == rustc::hir::Constness::Const,
                    is_async: sig.header.asyncness == rustc::hir::IsAsync::Async,
                    abi: sig.header.abi.name().to_owned(),
                    is_closure: false,
                    calls: vec![],
                    containing_mod: local_parent_index,
                    def_path: def_path.to_string_no_crate(),
                    argument_types: argument_types,
                    return_type: return_type,
                });

                std::mem::swap(&mut self.current_function, &mut func);
                if let Some(f) = func {
                    self.crate_data.functions.push(f);
                }
            }
            _ => {}
        }
        walk_impl_item(self, ii);
    }

    fn visit_body(&mut self, body: &'tcx hir::Body) {
        let id = body.id();
        let owner = self.map.body_owner_def_id(id);
        /*if let Some(function) = self.crate_data.get_function(owner) {

            //println!("found body of {:?}: {:?}", function, owner);
        }*/
        //println!("found body of {:?}: {:?}", function, owner);

        self.tcx
            .optimized_mir(owner)
            .basic_blocks()
            .iter()
            .for_each(|bbdata| {
                if let Some(mir::Terminator {
                    source_info: _,
                    kind: mir::TerminatorKind::Call { func, .. },
                }) = &bbdata.terminator
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
                        //print!("{:?}", self.tcx.original_crate_name(def_id.krate));
                        //println!("{:?}", self.tcx.def_path(def_id).to_string_no_crate());
                        let name = self.tcx.original_crate_name(def_id.krate).to_string();
                        let config_hash = self.tcx.crate_hash(def_id.krate).to_string();
                        let def_path = self.tcx.def_path(def_id).to_string_no_crate();
                        //println!("pretty: {}", self.tcx.def_path_debug_str(def_id));
                        if let Some(ref mut f) = self.current_function {
                            f.calls.push(data::GlobalDefPath {
                                crate_ident: data::CrateIdentifier { name, config_hash },
                                def_path,
                            });
                        } else {
                            //println!("ignored function call");
                        }
                    } else {
                        //println!("ignored function call");
                    }
                } else {
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
        //NestedVisitorMap::None
    }
}
