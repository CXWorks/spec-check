pub open spec fn rmi_rtt_set_s2ap_spec(rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, rtt_tree: UInt64, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let rec = RecAt(old_s, rec_ptr);
    let not_aligned = RttWalkAnyNotAligned(old_s, realm, base, top, RMM_RTT_PAGE_LEVEL);
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rec.state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
    && (rec.owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
    && (top <= base ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (base != rec.s2ap_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (top > rec.s2ap_top ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((not_aligned.valid == RMM_TRUE
         && !AddrRangeIsWithin(old_s, base, top,
                AlignDownToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level as int),
                AlignUpToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level as int))
         && not_aligned.index == RMM_RTT_TREE_PRIMARY
         && not_aligned.walk.rtte.s2ap_indirect.overlay_index != rec.s2ap_overlay_index)
        ==> ResultEqual(result, RMI_ERROR_RTT(not_aligned.walk.level as int)))
    && ((not_aligned.valid == RMM_TRUE
         && !AddrRangeIsWithin(old_s, base, top,
                AlignDownToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level as int),
                AlignUpToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level as int))
         && not_aligned.index != RMM_RTT_TREE_PRIMARY
         && not_aligned.walk.rtte.s2ap_indirect.overlay_index != rec.s2ap_overlay_index)
        ==> ResultEqual(result, RMI_ERROR_RTT_AUX(not_aligned.walk.level as int)))
    && ((AddrIsGranuleAligned(old_s, rd)
         && PaIsDelegable(old_s, rd)
         && GranuleAt(old_s, rd).state == RD
         && AddrIsGranuleAligned(old_s, rec_ptr)
         && PaIsDelegable(old_s, rec_ptr)
         && GranuleAt(old_s, rec_ptr).state == REC
         && rec.state != REC_RUNNING
         && rec.owner == rd
         && top > base
         && base == rec.s2ap_addr
         && top <= rec.s2ap_top
         && AddrIsGranuleAligned(old_s, top)
         && !(not_aligned.valid == RMM_TRUE
              && !AddrRangeIsWithin(old_s, base, top,
                     AlignDownToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level as int),
                     AlignUpToRttLevel(old_s, not_aligned.addr, not_aligned.walk.level as int))
              && not_aligned.walk.rtte.s2ap_indirect.overlay_index != rec.s2ap_overlay_index))
        ==> result.is_Ok()
            && RecAt(new_s, rec_ptr).s2ap_addr == out_top)
}