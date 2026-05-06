pub open spec fn rsi_plane_enter_spec(
    result: RsiCommandReturnCode,
    plane_idx: u64,
    run_ptr: Address,
    old_s: S,
    new_s: S,
) -> bool {
    let realm = CurrentRealm(old_s);
    let run = RsiPlaneRunAt(old_s, realm, run_ptr);
    let walk = RttWalk(old_s, realm, run_ptr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);

    // Failure condition: idx_bound
    (plane_idx == 0 || plane_idx > realm.num_aux_planes ==> result
        == RSI_ERROR_INPUT)
    // Failure condition: run_align
     && (!AddrIsGranuleAligned(run_ptr) ==> result
        == RSI_ERROR_INPUT)
    // Failure condition: run_bound
     && (!AddrIsProtected(run_ptr, realm) ==> result
        == RSI_ERROR_INPUT)
    // Failure condition: run_empty
     && (walk.rtte.ripas == EMPTY ==> result
        == RSI_ERROR_INPUT)
    // Failure condition: el
     && (run.enter.spsr_el2[3] == 1u64 ==> result
        == RSI_ERROR_INPUT)
    // Success condition: plane_exit
     && ((plane_idx != 0 && plane_idx <= realm.num_aux_planes && AddrIsGranuleAligned(run_ptr)
        && AddrIsProtected(run_ptr, realm) && walk.rtte.ripas != EMPTY && run.enter.spsr_el2[3]
        == 0u64) ==> (result == RSI_SUCCESS && RunContainsPlaneExitSyndrome(new_s, realm, run_ptr)))
}