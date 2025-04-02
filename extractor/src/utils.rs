extern crate rustc_type_ir;

use itertools::Itertools;
use rustc_hir::definitions::DefPathData;
use rustc_hir::{def::DefKind, def_id::DefId};
use rustc_middle::ty::{self, GenericArg, TyCtxt};

pub fn pretty_description<'t>(
    tcx: TyCtxt<'t>,
    def_id: DefId,
    substs: &'t [GenericArg<'t>],
) -> PrettyDescription {
    let mut desc = PrettyDescription::new();
    build_pretty_description(tcx, def_id, substs, &mut desc);
    desc
}

pub fn pretty_type_description<'t>(tcx: TyCtxt<'t>, ty: &ty::Ty<'t>) -> String {
    let desc = match ty.kind() {
        ty::Adt(adt, substs) => {
            let desc = pretty_description(tcx, adt.did(), substs);
            if !desc.type_generics.is_empty() {
                format!("{}<{}>", desc.path, desc.type_generics)
            } else {
                desc.path
            }
        }
        ty::Slice(element) | ty::Array(element, _) => {
            format!("&[{}]", pretty_type_description(tcx, element))
        }
        ty::Tuple(elements) => {
            format!(
                "({})",
                elements
                    .iter()
                    .map(|element| pretty_type_description(tcx, &element))
                    .join(", ")
            )
        }
        ty::Ref(_, ty, _) => {
            format!("&{}", pretty_type_description(tcx, ty))
        }
        _ => ty.to_string(),
    };
    desc
}

#[derive(Debug, Default)]
pub struct PrettyDescription {
    pub path: String,
    pub function_generics: String,
    pub type_generics: String,
}

impl PrettyDescription {
    fn new() -> Self {
        Self::default()
    }
}

fn build_pretty_description<'t>(
    tcx: TyCtxt<'t>,
    def_id: DefId,
    substs: &'t [GenericArg<'t>],
    desc: &mut PrettyDescription,
) {
    let def_key = tcx.def_key(def_id);
    if let Some(parent) = def_key.parent {
        let parent_substs = if substs.is_empty() {
            &[]
        } else {
            // as seen in rustc_middle::ty::print::Printer::default_print_def_path
            let generics = tcx.generics_of(def_id);
            &substs[..generics.parent_count.min(substs.len())]
        };

        use DefPathData::*;
        match def_key.disambiguated_data.data {
            Impl => match tcx.impl_subject(def_id).skip_binder() {
                ty::ImplSubject::Inherent(ty) => {
                    let ty_desc = pretty_type_description(tcx, &ty);
                    desc.path.push_str(&ty_desc);
                }
                ty::ImplSubject::Trait(trait_ref) => {
                    build_pretty_description(tcx, trait_ref.def_id, parent_substs, desc);
                }
            },
            data => {
                let parent_id = DefId {
                    krate: def_id.krate,
                    index: parent,
                };
                build_pretty_description(tcx, parent_id, parent_substs, desc);
                let component_name = crate::mirai_utils::component_name(&data);
                desc.path.push_str("::");
                desc.path.push_str(component_name);
            }
        }

        if !substs.is_empty() {
            let generics = tcx.generics_of(def_id);

            let resolved_generics = generics
                .own_args_no_defaults(tcx, substs)
                .iter()
                .flat_map(|arg| {
                    use rustc_type_ir::GenericArgKind::*;
                    let subst_ty = match arg.unpack() {
                        Type(ty) => ty,
                        Lifetime(_) | Const(_) => return None,
                    };
                    let subst_desc = match subst_ty.kind() {
                        // for our evaluation, we don't care which function is passed, or even how it's referenced
                        ty::TyKind::Closure(..) | ty::TyKind::FnDef(..) | ty::TyKind::FnPtr(..) => {
                            "$fn".to_string()
                        }
                        _ => pretty_type_description(tcx, &subst_ty),
                    };
                    Some(subst_desc)
                })
                .join(", ");

            if !resolved_generics.is_empty() {
                let dest = match tcx.def_kind(def_id) {
                    DefKind::Fn | DefKind::AssocFn => &mut desc.function_generics,
                    _kind => &mut desc.type_generics,
                };
                assert!(dest.is_empty());
                *dest = resolved_generics.clone();
            }
        }
    } else {
        let crate_name = tcx.crate_name(def_id.krate);
        desc.path.push_str(crate_name.as_str());
    }
}
