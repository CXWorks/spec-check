pub open spec fn rmi_rtt_set_s2ap_spec(rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, rtt_tree: UInt64, old_s: S, new_s: S) -> bool {
  (AddrIsGranuleAligned(old_s, rd) ==> result.is_Ok())
  && (PaIsDelegable(old_s, rd) ==> result.is_Ok())
  && (GranuleAt(old_s, rd).state == RD ==> result.is_Ok())
  && (AddrIsGranuleAligned(old_s, rec_ptr) ==> result.is_Ok())
  && (PaIsDelegable(old_s, rec_ptr) ==> result.is_Ok())
  && (GranuleAt(old_s, rec_ptr).state == REC ==> result.is_Ok())
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecAt(old_s, rec_ptr).owner == rd ==> result.is_Ok())
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (base != RecAt(old_s, rec_ptr).s2ap_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) > RecAt(old_s, rec_ptr).s2ap_top ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (AddrIsGranuleAligned(old_s, top) ==> result.is_Ok())
  && (RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RMM_TRUE && !AddrRangeIsWithin( base, top, AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level ), AlignUpToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level )) && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).index == RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.rtte.s2ap_indirect.overlay_index != RecAt(old_s, rec_ptr).s2ap_overlay_index ==> ResultEqual(result, RMI_ERROR_RTT(RttWalkAnyNotAligned(new_s, RealmAt(new_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int)))
  && (RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).valid == RMM_TRUE && !AddrRangeIsWithin( base, top, AlignDownToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level ), AlignUpToRttLevel(old_s, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).addr, RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level )) && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).index != RMM_RTT_TREE_PRIMARY && RttWalkAnyNotAligned(old_s, RealmAt(old_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.rtte.s2ap_indirect.overlay_index != RecAt(old_s, rec_ptr).s2ap_overlay_index ==> ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalkAnyNotAligned(new_s, RealmAt(new_s, rd), base, top,RMM_RTT_PAGE_LEVEL as int).walk.level as int)))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).s2ap_addr == out_top)
  && ((!(AddrIsGranuleAligned(old_s, rd)) &&
       !(PaIsDelegable(old_s, rd)) &&
       !(GranuleAt(old_s, rd).state == RD) &&
       !(AddrIsGranuleAligned(old_s, rec_ptr)) &&
       !(PaIsDelegable(old_s, rec_ptr)) &&
       !(GranuleAt(old_s, rec_ptr).state == REC) &&
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING) &&
       !(RecAt(old_s, rec_ptr).owner == rd) &&
       !((top) <= (base)) &&
       !(base != RecAt(old_s, rec_ptr).s2ap_addr) &&
       !((top) > RecAt(old_s, rec_ptr).s2ap_top) &&
       AddrIsGranuleAligned(old_s, top))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).s2ap_addr == RecAt(old_s, rec_ptr).s2ap_addr)
}