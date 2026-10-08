pub open spec fn rmi_data_destroy_spec(rd: Address, ipa: Address, result: Result<(), RmiStatusCode>, data: Address, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level < RMM_RTT_PAGE_LEVEL ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)) && top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level, ipa))
  && (RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)) && top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level, ipa))
  && (AddrIsAuxLive(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_RTT_AUX(0)))
  && (result.is_Ok() ==> ResultEqual(result, RMI_SUCCESS))
  && (result.is_Ok() ==> GranuleAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == DELEGATED)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).state == UNASSIGNED)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).ripas == DESTROYED ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).ripas == DESTROYED)
  && (result.is_Ok() ==> data == RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level, ipa))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, ipa) &&
       AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) &&
       !(RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level < RMM_RTT_PAGE_LEVEL) &&
       !(RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED) &&
       !(AddrIsAuxLive(old_s, ipa, RealmAt(old_s, rd))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == GranuleAt(old_s, RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).state == RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).state)
  && (RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RAM && result.is_Ok()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).ripas == DESTROYED)
  && (!(result.is_Ok() && (RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RAM))
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).ripas == RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).ripas)
}