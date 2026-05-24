```verus
pub open spec fn rmi_rtt_set_s2ap_spec(
    result: Result<(), RmiStatusCode>,
    out_top: Address,
    rtt_tree: u64,
    old_s: S,
    new_s: S,
    rd: Address,
    rec_ptr: Address,
    base: Address,
    top: Address
) -> bool {
    let realm = RealmAt(old_s, rd);
    let rec = RecAt(old_s, rec_ptr);
    let not_aligned = RttWalkAnyNotAligned(old_s, realm, base, top, RMM_RTT_PAGE_LEVEL);
    
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!PaIsDelegable(rec_ptr) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (rec.state == REC_RUNNING ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_REC))
    && (rec.owner != rd ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_REC))
    && ((top as int) <= (base as int) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (base != rec.s2ap_addr ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && ((top as int) > (rec.s2ap_top as int) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(top) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && ((not_aligned.valid == RMM_TRUE
         && !AddrRangeIsWithin(
             base, top,
             AlignDownToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level),
             AlignUpToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level))
         && not_aligned.index == RMM_RTT_TREE_PRIMARY
         && not_aligned.walk.rtte.s2ap_indirect.overlay_index != rec.s2ap_overlay_index)
        ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_RTT))
    && ((not_aligned.valid == RMM_TRUE
         && !AddrRangeIsWithin(
             base, top,
             AlignDownToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level),
             AlignUpToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level))
         && not_aligned.index != RMM_RTT_TREE_PRIMARY
         && not_aligned.walk.rtte.s2ap_indirect.overlay_index != rec.s2ap_overlay_index)
        ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_RTT_AUX))
    && ((AddrIsGranuleAligned(rd)
         && PaIsDelegable(rd)
         && GranuleAt(old_s, rd).state == RD
         && AddrIsGranuleAligned(rec_ptr)
         && PaIsDelegable(rec_ptr)
         && GranuleAt(old_s, rec_ptr).state == REC
         && rec.state != REC_RUNNING
         && rec.owner == rd
         && (top as int) > (base as int)
         && base == rec.s2ap_addr
         && (top as int) <= (rec.s2ap_top as int)
         && AddrIsGranuleAligned(top)
         && !(not_aligned.valid == RMM_TRUE
              && !AddrRangeIsWithin(
                  base, top,
                  AlignDownToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level),
                  AlignUpToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level))
              && not_aligned.index == RMM_RTT_TREE_PRIMARY
              && not_aligned.walk.rtte.s2ap_indirect.overlay_index != rec.s2ap_overlay_index)
         && !(not_aligned.valid == RMM_TRUE
              && !AddrRangeIsWithin(
                  base, top,
                  AlignDownToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level),
                  AlignUpToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level))
              && not_aligned.index != RMM_RTT_TREE_PRIMARY
              && not_aligned.walk.rtte.s2ap_indirect.overlay_index != rec.s2ap_overlay_index))
        ==> (result.is_Ok() && RecAt(new_s, rec_ptr).s2ap_addr == out_top))
}
```