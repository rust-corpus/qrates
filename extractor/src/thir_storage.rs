//! This module stores THIR bodies in thread local storage.
//! This is necessary because the query results will have been consumed by the time we need to access the THIR bodies.

extern crate rustc_hash;

use rustc_hash::FxHashMap;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::{
    thir::{ExprId, Thir},
    ty::TyCtxt,
};

thread_local! {
    static THIR_BODIES: std::cell::RefCell<FxHashMap<LocalDefId, (Thir<'static>, ExprId)>> = Default::default();
}

pub unsafe fn store_thir_body<'tcx>(
    _tcx: TyCtxt<'tcx>,
    def_id: LocalDefId,
    thir_body: Thir<'tcx>,
    expr_id: ExprId,
) {
    let thir_body = std::mem::transmute(thir_body);
    // eprintln!("Storing THIR body for {:?}", def_id);
    THIR_BODIES.with(|state| {
        let mut map = state.borrow_mut();
        assert!(map.insert(def_id, (thir_body, expr_id)).is_none());
    });
}

pub unsafe fn retrieve_thir_body<'tcx>(
    _tcx: TyCtxt<'tcx>,
    def_id: LocalDefId,
) -> Option<(Thir<'tcx>, ExprId)> {
    THIR_BODIES.with(|state| {
        let map = state.borrow();
        map.get(&def_id)
            .map(|(thir_body, expr_id)| (std::mem::transmute(thir_body.clone()), *expr_id))
    })
}
