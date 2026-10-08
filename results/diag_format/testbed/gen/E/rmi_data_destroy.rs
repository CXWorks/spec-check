pub open spec fn rmi_data_destroy_spec(rd: Address, ipa: Address, result: Result<(), RmiStatusCode>, data: Address, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level > RMM_RTT_PAGE_LEVEL ==> ResultEqual(result, RMI_ERROR_RTT(0)))
  && (RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).state != ASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(0)))
  && (AddrIsAuxLive(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_RTT_AUX(0)))
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).state == UNASSIGNED)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).ripas == DESTROYED)
  && (result.is_Ok() ==> GranuleAt(new_s, RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).addr).state == DELEGATED)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level, ipa))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, ipa) &&
       AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) &&
       !(RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level > RMM_RTT_PAGE_LEVEL) &&
       !(RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).state != ASSIGNED) &&
       !(AddrIsAuxLive(old_s, ipa, RealmAt(old_s, rd))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).state == RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).state)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).ripas == RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).ripas)
  && (result.is_Err()
    ==> GranuleAt(new_s, RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).addr).state == GranuleAt(old_s, RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).addr).state)
  && (result.is_Err()
    ==> RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level, ipa) == RttSkipNonLiveEntries(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level, ipa))
}