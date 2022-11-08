//! Report information about calls in our codebase.
//! For trait methods whose receiver is statically known, report this resolved type rather than the trait.

use crate::write_csv;
use corpus_database::tables::Loader;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn query(loader: &Loader, report_path: &Path) {
    let terminators_call_const_target = loader.load_terminators_call_const_target_as_map();
    let terminators_call_const_target_self =
        loader.load_terminators_call_const_target_self_as_map();
    let terminators_call_const_target_desc: HashMap<_, _> = loader
        .load_terminators_call_const_target_desc()
        .iter()
        .copied()
        .map(|(call, desc, function_generics, type_generics)| {
            (call, (desc, function_generics, type_generics))
        })
        .collect();
    let strings = loader.load_strings();
    let trait_items = loader.load_trait_items();
    let trait_items: HashSet<_> = trait_items
        .iter()
        .map(|(_trait_id, def_path, _defaultness)| def_path)
        .collect();
    let def_paths = loader.load_def_paths();
    let crate_names = loader.load_crate_names();
    let type_descriptions: HashMap<_, _> = loader
        .load_type_description()
        .iter()
        .copied()
        .map(|(ty, desc, generics)| (ty, (desc, generics)))
        .collect();
    let basic_block_def_paths: HashMap<_, _> = loader
        .load_basic_blocks()
        .iter()
        .map(|&(bb, def_path, _kind)| (bb, def_path))
        .collect();

    //let type_name_resolver = TypeNameResolver::new(loader);

    let all_calls = loader.load_terminators_call();
    let all_calls = all_calls.iter().filter_map(
        |&(block, call, func, _unsafety, _abi, _return_ty, _destination, _cleanup, _span)| {
            let (caller_crate, _, _, _, _) = def_paths[basic_block_def_paths[&block]];
            let caller_crate_name = &strings[crate_names[caller_crate]];

            let target = terminators_call_const_target.get(&call)?; // none for function pointers
            let (target_desc, function_generics, type_generics) =
                terminators_call_const_target_desc[&call];
            let (target_crate, _, _, _, _) = def_paths[*target];
            let target_crate_name = &strings[crate_names[target_crate]];
            let is_trait_item = trait_items.contains(target);

            let (receiver_name, receiver_generics) =
                terminators_call_const_target_self.get(&call).map_or_else(
                    || ("", ""),
                    |typ| {
                        let (desc, generics) = type_descriptions[typ];
                        (&strings[desc], &strings[generics])
                    },
                );

            Some((
                call,
                func,
                receiver_name,
                receiver_generics,
                &strings[target_desc],
                &strings[type_generics],
                &strings[function_generics],
                caller_crate_name,
                target_crate_name,
                is_trait_item,
            ))
        },
    );
    write_csv!(report_path, all_calls);
}

/*
struct TypeNameResolver<'b> {
    def_path_resolver: DefPathResolver<'b>,
    types: HashMap<Type, TyKind>,
    types_adt_def: HashMap<Type, DefPath>,
    types_primitive: HashMap<Type, TyPrimitive>,
    types_ref: HashMap<Type, (Type, Mutability)>,
    type_kinds: Ref<'b, InterningTable<TyKind, InternedString>>,
    strings: Ref<'b, InterningTable<InternedString, String>>,
}

impl<'b> TypeNameResolver<'b> {
    fn new(loader: &'b Loader) -> Self {
        Self {
            def_path_resolver: DefPathResolver::new(loader),
            types: loader.load_types().iter().copied().collect(),
            types_adt_def: loader
                .load_types_adt_def()
                .iter()
                .copied()
                .map(|(typ, def_path, _kind, _c_repr, _is_phantom)| (typ, def_path))
                .collect(),
            types_primitive: loader.load_types_primitive().iter().copied().collect(),
            types_ref: loader
                .load_types_ref()
                .iter()
                .copied()
                .map(|(typ, ty, mutability)| (typ, (ty, mutability)))
                .collect(),
            type_kinds: loader.load_type_kinds(),
            strings: loader.load_strings(),
        }
    }

    fn resolve(&self, typ: &Type) -> String {
        if let Some(adt_def) = self.types_adt_def.get(&typ) {
            let (_crate_name, _, _def_path, _, summary) = self.def_path_resolver.resolve(*adt_def);
            summary.replace(".", "::")
        } else if let Some(primitive) = self.types_primitive.get(&typ) {
            format!("$primitive::{}", primitive.to_string().to_lowercase())
        } else if let Some((ty, mutability)) = self.types_ref.get(&typ) {
            let mutability = match mutability {
                Mutability::Mutable => "&mut ",
                Mutability::Immutable => "&",
                Mutability::Const => "&const ",
                Mutability::Unknown => "&?",
            };
            format!("{}{}", mutability, self.resolve(ty))
        } else {
            let kind = self.type_kinds[self.types[typ]];
            format!("$other::{}", self.strings[kind])
        }
    }
}
*/
