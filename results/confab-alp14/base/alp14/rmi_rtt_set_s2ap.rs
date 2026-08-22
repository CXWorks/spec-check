pub open spec fn rmi_rtt_set_s2ap_spec(result: Result<(), RmiStatusCode>, out_top: Address, rtt_tree: u64, rd: Address, rec_ptr: Address, base: Address, top: Address, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let rec = RecAt(old_s, rec_ptr);
    let not_aligned = RttWalkAnyNotAligned(old_s, realm, base, top, RMM_RTT_PAGE_LEVEL as int);
    let base_not_aligned = not_aligned.valid == RMM_TRUE
        && !AddrRangeIsWithin(
            base,
            top,
            AlignDownToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level as int),
            AlignUpToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level as int))
        && not_aligned.walk.rtte.s2ap_indirect.overlay_index != rec.s2ap_overlay_index;
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rec.state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
    && (rec.owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
    && ((top as int) <= (base as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (base != rec.s2ap_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((top as int) > (rec.s2ap_top as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((base_not_aligned && not_aligned.index == RMM_RTT_TREE_PRIMARY)
        ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((base_not_aligned && not_aligned.index != RMM_RTT_TREE_PRIMARY)
        ==> ResultEqual(result, RMI_ERROR_RTT_AUX))
    && ((AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, rec_ptr)
        && PaIsDelegable(old_s, rec_ptr)
        && GranuleAt(old_s, rec_ptr).state == REC
        && rec.state != REC_RUNNING
        && rec.owner == rd
        && (top as int) > (base as int)
        && base == rec.s2ap_addr
        && (top as int) <= (rec.s2ap_top as int)
        && AddrIsGranuleAligned(old_s, top)
        && !base_not_aligned)
        ==> result.is_Ok() && RecAt(new_s, rec_ptr).s2ap_addr == out_top)
}