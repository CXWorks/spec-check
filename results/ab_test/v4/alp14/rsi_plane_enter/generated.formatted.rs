pub open spec fn RSI_PLANE_ENTER_spec(
    old_s: S,
    new_s: S,
    plane_idx: u64,
    run_ptr: Address,
    result: RsiCommandReturnCode,
) -> bool {
    let realm = CurrentRealm(old_s);
    let run = RsiPlaneRunAt(old_s, realm, run_ptr);
    let walk = RttWalk(old_s, realm, run_ptr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);

    ((plane_idx == 0 || plane_idx > realm.num_aux_planes) ==> result == RSI_ERROR_INPUT) && (
    !AddrIsGranuleAligned(run_ptr) ==> result == RSI_ERROR_INPUT) && (!AddrIsProtected(
        old_s,
        run_ptr,
        realm,
    ) ==> result == RSI_ERROR_INPUT) && (walk.rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
        && (run.enter.spsr_el2[3] == 1 ==> result == RSI_ERROR_INPUT) && ((plane_idx > 0
        && plane_idx <= realm.num_aux_planes && AddrIsGranuleAligned(run_ptr) && AddrIsProtected(
        old_s,
        run_ptr,
        realm,
    ) && walk.rtte.ripas != EMPTY && run.enter.spsr_el2[3] == 0) ==> (result == RSI_SUCCESS
        && new_s.realms[realm].planes[plane_idx as int].run.exit == run.exit))
}