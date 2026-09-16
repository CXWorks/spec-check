pub open spec fn rmi_rtt_set_s2ap_spec(rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, rtt_tree: UInt64, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state != GRAN_REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecAt(old_s, rec_ptr).owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (base != RecAt(old_s, rec_ptr).s2ap_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) > (RecAt(old_s, rec_ptr).s2ap_top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!(RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RMM_TRUE) && !AddrRangeIsWithin(old_s, base, top,AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int),AlignUpToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int)) && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).index == RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.rtte.s2ap_indirect.overlay_index!= RecAt(old_s, rec_ptr).s2ap_overlay_index) ==> (ResultEqual(result, RMI_ERROR_RTT) && ResultEqual(result, RttWalkAnyNotAligned(new_s, RealmAt(new_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int))))
  && ((RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RMM_TRUE) && !AddrRangeIsWithin(old_s, base, top,AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int),AlignUpToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int)) && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).index != RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.rtte.s2ap_indirect.overlay_index!= RecAt(old_s, rec_ptr).s2ap_overlay_index) ==> (ResultEqual(result, RMI_ERROR_RTT_AUX) && ResultEqual(result, RttWalkAnyNotAligned(new_s, RealmAt(new_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int))))
  && (result == RMI_ERROR_DPT ==> ResultEqual(result, RMI_ERROR_DPT))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).s2ap_addr == out_top)
  && ((AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       AddrIsRmiGranuleAligned(old_s, rec_ptr) &&
       PaIsTracked(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != GRAN_REC) &&
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING) &&
       !(RecAt(old_s, rec_ptr).owner != rd) &&
       !((top) <= (base)) &&
       !(base != RecAt(old_s, rec_ptr).s2ap_addr) &&
       !((top) > (RecAt(old_s, rec_ptr).s2ap_top)) &&
       AddrIsRmiGranuleAligned(old_s, top) &&
       !((!(RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RMM_TRUE) && !AddrRangeIsWithin(old_s, base, top,AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int),AlignUpToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int)) && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).index == RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.rtte.s2ap_indirect.overlay_index!= RecAt(old_s, rec_ptr).s2ap_overlay_index)) &&
       !((RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RMM_TRUE) && !AddrRangeIsWithin(old_s, base, top,AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int),AlignUpToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int)) && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).index != RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.rtte.s2ap_indirect.overlay_index!= RecAt(old_s, rec_ptr).s2ap_overlay_index)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).s2ap_addr == RecAt(old_s, rec_ptr).s2ap_addr)
}