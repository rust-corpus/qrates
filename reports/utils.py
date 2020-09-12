import pandas as pd

def count_builds_per_type(builds):
    """Count how many builds we have of each crate type.

    **Note:** Some builds have no crate types associated with them. This
    happens, for example, when a `build.rs` script manually invokes the
    Rust compiler.
    """
    crate_type_counts = {}
    for (crate_types, count) in builds.crate_types.value_counts().items():
        for crate_type in crate_types.split(', '):
            if crate_type not in crate_type_counts:
                crate_type_counts[crate_type] = 0
            crate_type_counts[crate_type] += count
    return crate_type_counts

def load(path, names):
    import os
    dtype = {}
    if 'package' in names:
        dtype['package'] = 'str'
    if 'crate' in names:
        dtype['crate'] = 'str'
    if 'package_name' in names:
        dtype['package_name'] = 'str'
    if 'crate_name' in names:
        dtype['crate_name'] = 'str'
    if 'resolved_crate' in names:
        dtype['resolved_crate'] = 'str'
    if 'field_crate' in names:
        dtype['field_crate'] = 'str'
    if 'trait_crate' in names:
        dtype['trait_crate'] = 'str'
    if 'target_crate' in names:
        dtype['target_crate'] = 'str'
    return pd.read_csv(
        os.path.join('../data', path),
        header=None,
        names=names,
        dtype=dtype,
        keep_default_na=False,
        na_values=[],
    )

def load_selected_builds():
    # Added
    return load(
        'prepare-builds/selected_builds.csv',
        [
            "build", "package", "version", "crate", "crate_hash", "edition", "crate_types"
        ],
    )

# Builds

def load_original_crates_list():
    return load(
        'prepare-builds/original_crates_list.csv',
        [
            "package", "version",
        ],
    )

# RQ1

def load_unsafe_blocks():
    return load(
        'q-counters/unsafe_blocks.csv',
        [
            "build",
            "crate", "crate_hash", "def_path", "def_path_hash", "summary_key",
            "scope", "check_mode",
            "span", "expansion_kind", "expansion_kind_descr", "file_name", "line", "col"
        ],
    )

def load_selected_traits():
    return load(
        'traits/selected_traits.csv',
        [
            "build",
            "crate", "crate_hash", "def_path", "def_path_hash", "summary_key",
            "item", "visibility", "unsafety", "is_auto", "is_marker", "impl_count"
        ],
    )

def load_selected_impl_definitions():
    return load(
        'traits/selected_impl_definitions.csv',
        [
            "build",
            "crate", "crate_hash", "def_path", "def_path_hash", "summary_key",
            "item", "visibility", "unsafety", "polarity", "defaultness", "constness",
            "trait_crate", "trait_crate_hash", "trait_def_path", "trait_def_path_hash", "trait_summary_key",
        ],
    )

def load_selected_function_definitions():
    return load(
        'q-counters/selected_function_definitions.csv',
        [
            "build",
            "crate", "crate_hash", "def_path", "def_path_hash", "summary_key",
            "item", "def_path_id", "module", "visibility", "unsafety", "abi",
            "uses_unsafe", "unsafe_block_count", "user_unsafe_block_count",
            "is_trait_item"
        ],
    )

def load_selected_build_sizes():
    return load(
        'function-size/selected_build_sizes.csv',
        [
            "package_name", "package_version", "crate_name", "build_crate_hash", "edition",
            "statement_count",
            "unsafe_statement_count",
            "user_unsafe_statement_count",
        ],
    )

def load_selected_function_sizes():
    return load(
        'function-size/selected_function_sizes.csv',
        [
            "package_name", "package_version", "crate_name", "build_crate_hash", "edition",
            "item",
            "crate", "crate_hash", "def_path", "def_path_hash", "summary_key",
            "visibility", "unsafety", "abi",
            "uses_unsafe",
            "statement_count",
            "unsafe_statement_count",
            "user_unsafe_statement_count",
        ],
    )

# RQ2

def load_unsafe_block_sizes():
    return load(
        'q-size/unsafe_block_sizes.csv',
        [
            "build",
            "package_name", "package_version", "crate_name", "crate_hash", "edition",
            "unsafe_scope", "check_mode", "statement_count", "terminator_count"
        ],
    )

# RQ3

def load_unsafe_block_calls_extended():
    return load(
        'unsafe-block-calls/unsafe_block_calls.csv',
        [
            "build",
            "package_name", "package_version", "crate_name", "build_crate_hash", "edition",
            "block", "unsafe_scope",
            "unsafe_scope_span", "unsafe_scope_expansion_kind", "unsafe_scope_expansion_kind_descr", "file_name", "line", "col",
            "check_mode", "call", "unsafety", "abi",
            "target_crate", "target_crate_hash", "call_target_def_path", "call_target_summary_key", "is_target_trait_item",
        ],
    )

def load_all_calls():
    return load(
        'unsafe-block-calls/all_calls.csv',
        [
            "call",
            "function",
            "unsafety",
            "abi",
            "call_target_summary_key",
            "is_target_trait_item",
        ],
    )

# RQ4

def load_const_calls():
    return load(
        'unsafe-block-groups/const_calls.csv',
        [
            "build",
            "package_name", "package_version", "crate_name", "build_crate_hash", "edition",
            "crate", "crate_hash", "def_path", "def_path_hash", "summary_key",
            "block", "unsafe_scope", "check_mode", "call", "unsafety", "abi",
       ],
    )

def load_categories():
    return load(
        'build-meta/categories.csv',
        [
            "build",
            "package_name", "package_version", "crate_name", "build_crate_hash", "edition",
            "category",
        ],
    )

def load_keywords():
    return load(
        'build-meta/keywords.csv',
        [
            "build",
            "package_name", "package_version", "crate_name", "build_crate_hash", "edition",
            "keyword",
        ],
    )

def load_unsafe_function_spans():
    return load(
        'unsafe-spans/unsafe_function_spans.csv',
        [
            "build",
            "package_name", "package_version", "crate_name", "build_crate_hash", "edition",
            "crate", "crate_hash", "def_path", "def_path_hash", "summary_key",
            "visibility", "abi",
            "uses_unsafe",
            "span", "expansion_kind", "expansion_kind_descr", "file_name", "line", "col",
        ],
    )

# RQ5

def load_unsafe_reasons_in_function_definitions():
    return load(
        'unsafe-reasons/function_unsafe_reasons.csv',
        [
            "crate_name", "crate_hash", "def_path", "def_path_hash", "summary_key",
            "unsafe_reason"
        ],
    )

def load_unsafe_block_calls():
    return load(
        'unsafe-block-groups/unsafe_block_calls.csv',
        [
            "build",
            "package_name", "package_version", "crate_name", "build_crate_hash", "edition",
            "block", "unsafe_scope", "check_mode", "call", "unsafety", "abi",
        ],
    )

def load_selected_type_defs():
    return load(
        'types/selected_type_defs.csv',
        [
            "package_name", "package_version", "crate_name", "build_crate_hash", "edition",
            "item", "typ",
            "crate", "crate_hash", "def_path", "def_path_hash", "summary_key",
            "name", "visibility", "type_kind", "def_kind",
       ],
    )

def load_selected_adts():
    return load(
        'types/selected_adts.csv',
        [
            "package_name", "package_version", "crate_name", "build_crate_hash", "edition",
            "item", "typ",
            "crate", "crate_hash", "def_path", "def_path_hash", "summary_key",
            "resolved_crate", "resolved_crate_hash", "resolved_def_path", "resolved_def_path_hash", "resolved_summary_key",
            "name", "visibility", "type_kind", "def_kind", "kind", "c_repr", "is_phantom"
        ],
    )

def load_selected_adt_field_types():
    return load(
        'types/selected_adt_field_types.csv',
        [
            "package_name", "package_version", "crate_name", "build_crate_hash", "edition",
            "item", "typ", "variant",
            "crate", "crate_hash", "def_path", "def_path_hash", "summary_key",
            "resolved_crate", "resolved_crate_hash", "resolved_def_path", "resolved_def_path_hash", "resolved_summary_key",
            "field_crate", "field_crate_hash", "field_def_path", "field_def_path_hash", "field_summary_key",
            "name", "visibility", "type_kind", "def_kind", "kind", "c_repr", "is_phantom",
            "field_name", "field_visibility", "field_type", "field_type_kind"
        ],
    )

def load_non_tree_adts():
    return load(
        'non-tree-types/non_tree_adts.csv',
        [
            "build",
            "package_name", "package_version", "crate_name", "build_crate_hash", "edition",
            "item",
            "type",
            "crate", "crate_hash", "def_path", "def_path_hash", "summary_key",
            "resolved_crate", "resolved_crate_hash", "resolved_def_path", "resolved_def_path_hash", "resolved_summary_key",
            "name",
            "visibility",
            "type_kind",
            "def_kind",
            "kind",
            "c_repr",
            "is_phantom",
        ],
    )
