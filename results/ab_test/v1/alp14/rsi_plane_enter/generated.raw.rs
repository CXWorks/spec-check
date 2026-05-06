```rust
pub open spec fn RSI_PLANE_ENTER_spec(s: S, plane_idx: u64, run_ptr: Address) -> (result: RsiCommandReturnCode, s_new: S) {
    let realm = CurrentRealm(s);
    let run = RsiPlaneRunAt(s, realm, run_ptr);
    let walk = RttWalk(s, realm, run_ptr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    
    // Failure condition: idx_bound
    if plane_idx == 0 || plane_idx > realm.num_aux_planes {
        (RSI_ERROR_INPUT, s)
    }
    // Failure condition: run_align
    else if !AddrIsGranuleAligned(run_ptr) {
        (RSI_ERROR_INPUT, s)
    }
    // Failure condition: run_bound
    else if !AddrIsProtected(run_ptr, realm) {
        (RSI_ERROR_INPUT, s)
    }
    // Failure condition: run_empty
    else if walk.rtte.ripas == EMPTY {
        (RSI_ERROR_INPUT, s)
    }
    // Failure condition: el
    else if run.enter.spsr_el2[3] == 1u64 {
        (RSI_ERROR_INPUT, s)
    }
    // Success condition: plane_exit
    else {
        // run.exit contains Plane exit syndrome information
        (RSI_SUCCESS, s)
    }
}
```