```verus
pub open spec fn RMI_RTT_SET_S2AP_spec(s: S, rd: Address, rec_ptr: Address, base: Address, top: Address, result: RmiCommandReturnCode, out_top: Address, rtt_tree: u64) -> bool {
    let realm = RealmAt(s, rd);
    let rec = RecAt(s, rec_ptr);
    let not_aligned = RttWalkAnyNotAligned(s, realm, base, top, RMM_RTT_PAGE_LEVEL);
    
    let rd_align_fail = !AddrIsGranuleAligned(s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rd_bound_fail = !PaIsDelegable(s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rd_state_fail = GranuleAt(s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rec_align_fail = !AddrIsGranuleAligned(s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rec_bound_fail = !PaIsDelegable(s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rec_gran_state_fail = GranuleAt(s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rec_state_fail = rec.state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC);
    let rec_owner_fail = rec.owner != rd ==> ResultEqual(result, RMI_ERROR_REC);
    let size_valid_fail = UInt(top) <= UInt(base) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let base_bound_fail = base != rec.s2ap_addr ==> ResultEqual(result, RMI_ERROR_INPUT);
    let top_bound_fail = UInt(top) > UInt(rec.s2ap_top) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let top_gran_align_fail = !AddrIsGranuleAligned(s, top) ==> ResultEqual(result, RMI_ERROR_INPUT);
    
    let base_align_pri_fail = (not_aligned.valid == RMM_TRUE 
        && !AddrRangeIsWithin(s, base, top, 
            AlignDownToRttLevel(s, not_aligned.addr, not_aligned.walk.level),
            AlignUpToRttLevel(s, not_aligned.addr, not_aligned.walk.level))
        && not_aligned.index == RMM_RTT_TREE_PRIMARY
        && not_aligned.walk.rtte.s2ap_indirect.overlay_index != rec.s2ap_overlay_index)
        ==> ResultEqual(result, RMI_ERROR_RTT);
    
    let base_align_aux_fail = (not_aligned.valid == RMM_TRUE 
        && !AddrRangeIsWithin(s, base, top,
            AlignDownToRttLevel(s, not_aligned.addr, not_aligned.walk.level),
            AlignUpToRttLevel(s, not_aligned.addr, not_aligned.walk.level))
        && not_aligned.index != RMM_RTT_TREE_PRIMARY
        && not_aligned.walk.rtte.s2ap_indirect.overlay_index != rec.s2ap_overlay_index)
        ==> ResultEqual(result, RMI_ERROR_RTT_AUX);
    
    let success_s2ap_addr = (result.is_Ok() && result.get_Ok_0() == RMI_SUCCESS) ==> rec.s2ap_addr == out_top;
    
    rd_align_fail && rd_bound_fail && rd_state_fail && rec_align_fail && rec_bound_fail 
        && rec_gran_state_fail && rec_state_fail && rec_owner_fail && size_valid_fail 
        && base_bound_fail && top_bound_fail && top_gran_align_fail && base_align_pri_fail 
        && base_align_aux_fail && success_s2ap_addr
}
```