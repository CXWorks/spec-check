pub open spec fn rmi_rtt_set_s2ap_spec(rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, rtt_tree: UInt64, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
    && (!RecAt(old_s, rec_ptr).owner == rd ==> ResultEqual(result, RMI_ERROR_REC))
    && (top <= base ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (base != RecAt(old_s, rec_ptr).s2ap_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (top > RecAt(old_s, rec_ptr).s2ap_top ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top, RMM_RTT_PAGE_LEVEL).valid ==> (
        (!AddrRangeIsWithin(old_s, RealmAt(old_s, rd), base, top, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top, RMM_RTT_PAGE_LEVEL).addr_aligned_down, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top, RMM_RTT_PAGE_LEVEL).addr_aligned_up)
        && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top, RMM_RTT_PAGE_LEVEL).tree == RMM_RTT_TREE_PRIMARY
        && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top, RMM_RTT_PAGE_LEVEL).overlay_index != RecAt(old_s, rec_ptr).s2ap_overlay_index
        ==> ResultEqual(result, RMI_ERROR_RTT)
    ))
    && (RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top, RMM_RTT_PAGE_LEVEL).valid ==> (
        (!AddrRangeIsWithin(old_s, RealmAt(old_s, rd), base, top, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top, RMM_RTT_PAGE_LEVEL).addr_aligned_down, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top, RMM_RTT_PAGE_LEVEL).addr_aligned_up)
        && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top, RMM_RTT_PAGE_LEVEL).tree != RMM_RTT_TREE_PRIMARY
        && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top, RMM_RTT_PAGE_LEVEL).overlay_index != RecAt(old_s, rec_ptr).s2ap_overlay_index
        ==> ResultEqual(result, RMI_ERROR_RTT_AUX)
    ))
    && (result.is_Ok() ==> (
        RecAt(new_s, rec_ptr).s2ap_addr == out_top
    ))
}