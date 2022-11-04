use rustc_hir::def_id::DefId;
use rustc_hir::definitions::DefPathData;
use rustc_middle::ty::{self, GenericArg, TyCtxt};

pub fn pretty_description<'t>(
    tcx: TyCtxt<'t>,
    def_id: DefId,
    substs: &'t [GenericArg<'t>],
) -> String {
    let mut desc = String::new();
    //let path = tcx.def_path(def_id);
    //println!();
    //println!(
    //    "building for {}",
    //    tcx.def_path_str_with_substs(def_id, substs)
    //);
    build_pretty_description(tcx, def_id, substs, &mut desc);
    //println!("pretty_description: {}", desc);
    //println!();
    desc
}

fn build_pretty_description(
    tcx: TyCtxt<'_>,
    def_id: DefId,
    substs: &[GenericArg],
    desc: &mut String,
) {
    let def_key = tcx.def_key(def_id);
    //println!("data: {:?}", def_key.disambiguated_data.data);
    if let Some(parent) = def_key.parent {
        //println!("desc: {}, last_component: {:?}", desc, last_component);
        //let generics = tcx.generics_of(def_id);
        //println!("generics: {:?}", generics);

        use DefPathData::*;
        match def_key.disambiguated_data.data {
            Impl => {
                match tcx.impl_subject(def_id) {
                    ty::ImplSubject::Inherent(ty) => {
						let parent_substs = if substs.is_empty() {
							&[]
						} else {
							// as seen in rustc_middle::ty::print::Printer::default_print_def_path
							let generics = tcx.generics_of(def_id);
							&substs[..generics.parent_count.min(substs.len())]
						};
						// get DefId of the type
						if let Some(ty_def) = ty.ty_adt_def() {
							build_pretty_description(tcx, ty_def.did(), parent_substs, desc);
						} else {
							let ty_desc = ty.to_string();
							desc.push_str(&ty_desc);
						}
                    }
                    ty::ImplSubject::Trait(_trait_ref) => {
                        desc.push_str("<APPARENTLY NOT UNREACHABLE>");
                    }
                }
            }
            data => {
                let parent_id = DefId {
                    krate: def_id.krate,
                    index: parent,
                };
                let parent_substs = if substs.is_empty() {
                    &[]
                } else {
					// as seen in rustc_middle::ty::print::Printer::default_print_def_path
                    let generics = tcx.generics_of(def_id);
                    &substs[..generics.parent_count.min(substs.len())]
                };
                build_pretty_description(tcx, parent_id, parent_substs, desc);
                desc.push_str("::");
                desc.push_str(crate::mirai_utils::component_name(&data));

                //println!("back to {:?}", def_key.disambiguated_data.data);
            }
        }
		
		if !substs.is_empty() {
			let generics = tcx.generics_of(def_id);
			//println!("generics: {:?}", generics);

			let start_index = if generics.has_self { 1 } else { 0 };
			let params: Vec<_> = generics
				.params
				.iter()
				.filter(|p| p.index >= start_index)
				.collect();

			if !params.is_empty() {
				desc.push('<');
				for generic in params {
					let subst = substs[generic.index as usize];
					//println!("generic: {} for {}", subst, generic.name);
					desc.push_str(&subst.to_string());
				}
				desc.push('>');
			}
		}
    } else {
        //println!("root");
        desc.push_str(tcx.crate_name(def_id.krate).as_str())
    }
}
