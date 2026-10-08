pub open spec fn rmi_data_destroy_spec(rd: Address, ipa: Address, result: Result<(), RmiStatusCode>, data: Address, top: Address, old_s: S, new_s: S) -> bool {
  (AddrIsGranuleAligned(old_s, rd) ==> result.is_Ok())
  && (AddrIsGranuleAligned(old_s, ipa) ==> result.is_Ok())
  && (GranuleAt(old_s, rd).state == RD ==> result.is_Ok())
  && (AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> result.is_Ok())
  && (result.is_Ok() && RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY) .level <= RMM_RTT_PAGE_LEVEL ==> result.is_Ok())
  && (result.is_Ok() && RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY) .rtte.state == ASSIGNED ==> result.is_Ok())
  && (result.is_Ok() && RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY) .rtte.ripas == RAM ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY).level as int)).ripas == DESTROYED)
  && (result.is_Ok() && RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY) .rtte.state != ASSIGNED ==> result.is_Err())
  && (result.is_Ok() && RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY) .rtte.state == ASSIGNED ==> result.is_Ok())
  && ((!(AddrIsGranuleAligned(old_s, rd)) ||
       !(AddrIsGranuleAligned(old_s, ipa)) ||
       !(GranuleAt(old_s, rd).state == RD) ||
       !(AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))))
    ==> result.is_Err())
  && (result.is_Err()
    ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY) .rtte.state == RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY) .rtte.state)
  && (result.is_Err()
    ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY) .rtte.ripas == RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY) .rtte.ripas)
  && (result.is_Err()
    ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY) .rtte.state == RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY) .rtte.state)
}