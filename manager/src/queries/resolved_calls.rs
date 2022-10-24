//! Report information about calls in our codebase.
//! For trait methods whose receiver is statically known, report this resolved type rather than the trait.

use super::utils::DefPathResolver;
use crate::write_csv;
use corpus_database::tables::Loader;
use corpus_database::{types::*, InterningTable};
use std::cell::Ref;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn query(loader: &Loader, report_path: &Path) {
    let terminators_call_const_target: HashMap<_, _> = loader
        .load_terminators_call_const_target()
        .iter()
        .copied()
        .collect();
    let terminators_call_const_target_self: HashMap<_, _> = loader
        .load_terminators_call_const_target_self()
        .iter()
        .copied()
        .collect();
    let strings = loader.load_strings();
    let abis = loader.load_abis();
    let trait_items = loader.load_trait_items();
    let trait_items: HashSet<_> = trait_items
        .iter()
        .map(|(_trait_id, def_path, _defaultness)| def_path)
        .collect();
    let def_path_descriptions = loader.load_def_path_descriptions();
    let pretty_descriptions: HashMap<_, _> = loader
        .load_def_path_description()
        .iter()
        .map(|&(def_path, description)| (def_path, &strings[def_path_descriptions[description]]))
        .collect();

    let type_name_resolver = TypeNameResolver::new(loader);

    let all_calls = loader.load_terminators_call();
    let all_calls = all_calls.iter().map(
        |&(_block, call, func, _unsafety, abi, _return_ty, _destination, _cleanup, _span)| {
            // TODO: crate of call site
            let (call_target, is_trait_item) = if let Some(target) =
                terminators_call_const_target.get(&call)
            {
                (
                    pretty_descriptions[target].as_ref(),
                    trait_items.contains(target),
                )
            } else {
                ("non-const", false)
            };
            let call_receiver_name = terminators_call_const_target_self
                .get(&call)
                .map_or_else(|| "".to_string(), |typ| type_name_resolver.resolve(typ));
            (
                call,
                func,
                strings[abis[abi]].to_string(),
                call_receiver_name,
                call_target,
                is_trait_item,
            )
        },
    );
    write_csv!(report_path, all_calls);
}

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
