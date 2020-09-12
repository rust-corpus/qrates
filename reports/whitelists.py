trusted_crate_names = set([
    "std", "core", "alloc", "proc_macro"
])

def get_sys_crate_names(all_crate_names):
    res = set([
        x for x in all_crate_names
        if x.endswith("_sys") or
           x.endswith("_bindgen") or
           x.endswith("_ffi") or
           x.endswith("_bindings")
    ])
    # Bindings to C libraries
    res |= set(["libc", "gl", "gl32", "glow"])
    # Wrappers around C/C++ libraries
    res |= set(["glib", "gdnative_common", "gdnative_core", "qt_core"])
    # Calls to C/C++ wrappers around C/C++ libraries
    res |= set(["nsf_imgui_raw", "x11", "gfx_gl"])
    # Other C/C++ interoperability (e.g. manual conversions across types)
    res |= set(["cpp_core"])
    return res

# Adding dynamic features absent in Rust (loading libraries at runtime/reflective features)
dynamic_features_crate_names = set([
    "libloading", "shared_library", "dl_api", "dlopen",
    # TODO: double check
    #"alpm_sys_fork",
])

hardware_crate_names = set([
    "stm32f4", "oxcc_stm32f767", "mkl26z4", "stm32l4", "atsamd51j20a", "gd32vf103_pac",
    "stm32l4x6", "stm32l0", "e310x",
    # Hardware Interaction
    "vks", 
])

def get_unchecked_function_names(all_function_names):
    return set([
        def_path
        for def_path in all_function_names
        if "unchecked" in def_path.split('::')[-1]
    ])
