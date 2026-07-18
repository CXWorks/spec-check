pub open spec fn rmi_rtt_set_s2ap_spec(rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<RmiCommandReturnCode, _>, out_top: Address, rtt_tree: UInt64, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecAt(old_s, rec_ptr).owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (base != RecAt(old_s, rec_ptr).s2ap_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) > (RecAt(old_s, rec_ptr).s2ap_top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RMM_TRUE && !AddrRangeIsWithin(old_s, base, top,AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level),AlignUpToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level)) && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).index == RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.rtte.s2ap_indirect.overlay_index != RecAt(old_s, rec_ptr).s2ap_overlay_index) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalkAnyNotAligned(new_s, RealmAt(new_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int)))
  && ((RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RMM_TRUE && !AddrRangeIsWithin(old_s, base, top,AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level),AlignUpToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr,RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level)) && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).index != RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.rtte.s2ap_indirect.overlay_index != RecAt(old_s, rec_ptr).s2ap_overlay_index) ==> ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalkAnyNotAligned(new_s, RealmAt(new_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int)))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).s2ap_addr == out_top)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, rec_ptr) &&
       PaIsDelegable(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != REC) &&
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING) &&
       !(RecAt(old_s, rec_ptr).owner != rd) &&
       !((top) <= (base)) &&
       !(base != RecAt(old_s, rec_ptr).s2ap_addr) &&
       !((top) > (RecAt(old_s, rec_ptr).s2ap_top)) &&
       AddrIsGranuleAligned(old_s, top) &&
       !((RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RMM_TRUE && !AddrRangeIsWithin(old_s, base, top,AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s, rd), RttWalkAnyNotAligned(old_s, rd).walk.level),AlignUpToRttLevel(old_s, RttWalkAnyNotAligned(old_s, rd).addr,RttWalkAnyNotAligned(old_s, rd).walk.level)) && RttWalkAnyNotAligned(old_s, rd).index == RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, rd).walk.rtte.s2ap_indirect.overlay_index != RecAt(old_s, rec_ptr).s2ap_overlay_index)) &&
       (RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RMM_TRUE && !AddrRangeIsWithin(old_s, base, top,AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s, rd).addr,RttWalkAnyNotAligned(old_s, rd).walk.level),AlignUpToRttLevel(old_s, RttWalkAnyNotAligned(old_s, rd).addr,RttWalkAnyNotAligned(old_s, rd).walk.level)) && RttWalkAnyNotAligned(old_s, rd).index != RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, rd).walk.rtte.s2ap_indirect.overlay_index != RecAt(old_s, rec_ptr).s2ap_overlay_index))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).s2ap_addr == RecAt(old_s, rec_ptr).s2ap_addr)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).s2ap_top == RecAt(old_s, rec_ptr).s2ap_top)
  && (AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, rec_ptr) &&
       PaIsDelegable(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != REC) &&
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING) &&
       !(RecAt(old_s, rec_ptr).owner != rd) &&
       !((top) <= (base)) &&
       !(base != RecAt(old_s, rec_ptr).s2ap_addr) &&
       !((top) > (RecAt(old_s, rec_ptr).s2ap_top)) &&
       AddrIsGranuleAligned(old_s, top) &&
       !((RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RMM_TRUE && !AddrRangeIsWithin(old_s, base, top,AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s, rd), RttWalkAnyNotAligned(old_s, rd).walk.level),AlignUpToRttLevel(old_s, RttWalkAnyNotAligned(old_s, rd).addr,RttWalkAnyNotAligned(old_s, rd).walk.level)) && RttWalkAnyNotAligned(old_s, rd).index == RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, rd).walk.rtte.s2ap_indirect.overlay_index != RecAt(old_s, rec_ptr).s2ap_overlay_index)) &&
       (RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RMM_TRUE && !AddrRangeIsWithin(old_s, base, top,AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s