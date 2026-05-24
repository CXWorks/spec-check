pub open spec fn rsi_plane_enter_spec(result: RsiCommandReturnCode, plane_idx: u64, run_ptr: Address, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    let run = RsiPlaneRunAt(old_s, realm, run_ptr);
    let walk = RttWalk(old_s, realm, run_ptr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    
    ((plane_idx == 0 || plane_idx > realm.num_aux_planes) ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(run_ptr) ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (!AddrIsProtected(run_ptr, realm) ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (walk.rtte.ripas == EMPTY ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (run.enter.spsr_el2[3] == 1u64 ==> ResultEqual(result, RSI_ERROR_INPUT))
    && ((plane_idx > 0 && plane_idx <= realm.num_aux_planes
         && AddrIsGranuleAligned(run_ptr)
         && AddrIsProtected(run_ptr, realm)
         && walk.rtte.ripas != EMPTY
         && run.enter.spsr_el2[3] != 1u64)
        ==> (result.is_Ok() && run.exit.is_some()))
}