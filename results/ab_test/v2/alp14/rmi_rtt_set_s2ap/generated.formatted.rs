pub open spec fn rmi_rtt_set_s2ap_spec(
    result: RmiCommandReturnCode,
    out_top: Address,
    rtt_tree: u64,
    old_s: S,
    new_s: S,
    rd: Address,
    rec_ptr: Address,
    base: Address,
    top: Address,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let rec = RecAt(old_s, rec_ptr);
    let not_aligned = RttWalkAnyNotAligned(old_s, realm, base, top, RMM_RTT_PAGE_LEVEL);

    // rd_align
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // rd_bound
     && (!PaIsDelegable(old_s, rd) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // rd_state
     && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // rec_align
     && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // rec_bound
     && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // rec_gran_state
     && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // rec_state
     && (rec.state == REC_RUNNING ==> ResultEqual(
        result,
        RMI_ERROR_REC,
    ))
    // rec_owner
     && (rec.owner != rd ==> ResultEqual(
        result,
        RMI_ERROR_REC,
    ))
    // size_valid
     && (UInt(top) <= UInt(base) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // base_bound
     && (base != rec.s2ap_addr ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // top_bound
     && (UInt(top) > UInt(rec.s2ap_top) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // top_gran_align
     && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // base_align_pri
     && ((not_aligned.valid == RMM_TRUE && !AddrRangeIsWithin(
        old_s,
        base,
        top,
        AlignDownToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level),
        AlignUpToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level),
    ) && not_aligned.index == RMM_RTT_TREE_PRIMARY
        && not_aligned.walk.rtte.s2ap_indirect.overlay_index != rec.s2ap_overlay_index) ==> (result
        == RMI_ERROR_RTT && rtt_tree
        == not_aligned.index))
    // base_align_aux
     && ((not_aligned.valid == RMM_TRUE && !AddrRangeIsWithin(
        old_s,
        base,
        top,
        AlignDownToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level),
        AlignUpToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level),
    ) && not_aligned.index != RMM_RTT_TREE_PRIMARY
        && not_aligned.walk.rtte.s2ap_indirect.overlay_index != rec.s2ap_overlay_index) ==> (result
        == RMI_ERROR_RTT_AUX && rtt_tree
        == not_aligned.index))
    // Success condition: s2ap_addr
     && ((AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state
        == RD && AddrIsGranuleAligned(old_s, rec_ptr) && PaIsDelegable(old_s, rec_ptr) && GranuleAt(
        old_s,
        rec_ptr,
    ).state == REC && rec.state != REC_RUNNING && rec.owner == rd && UInt(top) > UInt(base) && base
        == rec.s2ap_addr && UInt(top) <= UInt(rec.s2ap_top) && AddrIsGranuleAligned(old_s, top)
        && !(not_aligned.valid == RMM_TRUE && !AddrRangeIsWithin(
        old_s,
        base,
        top,
        AlignDownToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level),
        AlignUpToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level),
    ) && ((not_aligned.index == RMM_RTT_TREE_PRIMARY
        && not_aligned.walk.rtte.s2ap_indirect.overlay_index != rec.s2ap_overlay_index) || (
    not_aligned.index != RMM_RTT_TREE_PRIMARY && not_aligned.walk.rtte.s2ap_indirect.overlay_index
        != rec.s2ap_overlay_index)))) ==> (RecAt(new_s, rec_ptr).s2ap_addr == out_top))
}