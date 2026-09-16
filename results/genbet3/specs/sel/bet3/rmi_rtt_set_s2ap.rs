pub open spec fn rmi_rtt_set_s2ap_spec(rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<(Address, UInt64), RmiStatusCode>, out_top: Address, rtt_tree: UInt64, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, rec_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rec_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rec_ptr).state != GRAN_REC ==> result.status == RMI_ERROR_INPUT)
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> result.status == RMI_ERROR_REC)
  && (RecAt(old_s, rec_ptr).owner != rd ==> result.status == RMI_ERROR_REC)
  && ((top) <= (base) ==> result.status == RMI_ERROR_INPUT)
  && (base != RecAt(old_s, rec_ptr).s2ap_addr ==> result.status == RMI_ERROR_INPUT)
  && ((top) > (RecAt(old_s, rec_ptr).s2ap_top) ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, top) ==> result.status == RMI_ERROR_INPUT)
  && ((RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RMM_TRUE && !AddrRangeIsWithin(old_s, base, top, AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int), AlignUpToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int)) && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).index == RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.rtte.s2ap_indirect.overlay_index != RecAt(old_s, rec_ptr).s2ap_overlay_index) ==> (result.status == RMI_ERROR_RTT && result.data.level.level == RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level))
  && ((RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RMM_TRUE && !AddrRangeIsWithin(old_s, base, top, AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int), AlignUpToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int)) && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).index != RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.rtte.s2ap_indirect.overlay_index != RecAt(old_s, rec_ptr).s2ap_overlay_index) ==> (result.status == RMI_ERROR_RTT_AUX && result.data.level.level == RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level))
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
       !((RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RMM_TRUE && !AddrRangeIsWithin(old_s, base, top, AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int), AlignUpToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int)) && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).index == RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.rtte.s2ap_indirect.overlay_index != RecAt(old_s, rec_ptr).s2ap_overlay_index)) &&
       !((RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RMM_TRUE && !AddrRangeIsWithin(old_s, base, top, AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int), AlignUpToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int)) && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).index != RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.rtte.s2ap_indirect.overlay_index != RecAt(old_s, rec_ptr).s2ap_overlay_index)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).s2ap_addr == RecAt(old_s, rec_ptr).s2ap_addr)
}