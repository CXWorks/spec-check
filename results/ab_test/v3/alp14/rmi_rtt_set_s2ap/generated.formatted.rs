pub open spec fn RMI_RTT_SET_S2AP_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    rec_ptr: Address,
    base: Address,
    top: Address,
    result: RmiCommandReturnCode,
    out_top: Address,
    rtt_tree: u64,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let rec = RecAt(old_s, rec_ptr);
    let not_aligned = RttWalkAnyNotAligned(old_s, realm, base, top, RMM_RTT_PAGE_LEVEL);

    // Failure condition: rd_align
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: rd_bound
    (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: rd_state
    (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: rec_align
    (!AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: rec_bound
    (!PaIsDelegable(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: rec_gran_state
    (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: rec_state
    (rec.state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
        &&
    // Failure condition: rec_owner
    (rec.owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
        &&
    // Failure condition: size_valid
    (UInt(top) <= UInt(base) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: base_bound
    (base != rec.s2ap_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: top_bound
    (UInt(top) > UInt(rec.s2ap_top) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: top_gran_align
    (!AddrIsGranuleAligned(top) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: base_align_pri
    ((not_aligned.valid == RMM_TRUE && !AddrRangeIsWithin(
        base,
        top,
        AlignDownToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level),
        AlignUpToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level),
    ) && not_aligned.index == RMM_RTT_TREE_PRIMARY
        && not_aligned.walk.rtte.s2ap_indirect.overlay_index != rec.s2ap_overlay_index) ==> (
    result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT))
        &&
    // Failure condition: base_align_aux
    ((not_aligned.valid == RMM_TRUE && !AddrRangeIsWithin(
        base,
        top,
        AlignDownToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level),
        AlignUpToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level),
    ) && not_aligned.index != RMM_RTT_TREE_PRIMARY
        && not_aligned.walk.rtte.s2ap_indirect.overlay_index != rec.s2ap_overlay_index) ==> (
    result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT_AUX))
        &&
    // Success condition: s2ap_addr
    (result.is_Ok() ==> RecAt(new_s, rec_ptr).s2ap_addr == out_top)
}