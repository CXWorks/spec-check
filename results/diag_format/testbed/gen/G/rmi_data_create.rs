pub open spec fn rmi_data_create_spec(rd: Address, data: Address, ipa: Address, src: Address, flags: RmiDataFlags, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((!(AddrIsGranuleAligned(old_s, src)) ||
       (GranuleAt(old_s, src).state != DATA) ||
       (!AddrIsGranuleAligned(old_s, data)) ||
       (!PaIsDelegable(old_s, data)) ||
       (GranuleAt(old_s, data).state != DELEGATED) ||
       ((data) >= pow2(48) &&
        RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE) ||
       (!AddrIsGranuleAligned(old_s, rd)) ||
       (!PaIsDelegable(old_s, rd)) ||
       (GranuleAt(old_s, rd).state != RD) ||
       (!AddrIsGranuleAligned(old_s, ipa)) ||
       (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))))
    ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_NEW
    ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level < RMM_RTT_PAGE_LEVEL
    ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED
    ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Ok()
    ==> GranuleAt(new_s, data).state == DATA)
  && (result.is_Ok()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).state == ASSIGNED)
  && (result.is_Ok()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).ripas == RAM)
  && (result.is_Ok()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).output_addr == data)
  && (result.is_Ok()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).attr_prot == MEMATTR_CACHEABLE)
  && (result.is_Ok()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).sh == SHAREABILITY_INNER)
  && ((!(AddrIsGranuleAligned(old_s, src)) &&
       !(GranuleAt(old_s, src).state != DATA)) &&
       (AddrIsGranuleAligned(old_s, src)) &&
       (GranuleAt(old_s, src).state == DATA) &&
       AddrIsGranuleAligned(old_s, data) &&
       PaIsDelegable(old_s, data) &&
       GranuleAt(old_s, data).state == DELEGATED &&
       !((data) >= pow2(48) &&
        RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       GranuleAt(old_s, rd).state == RD &&
       AddrIsGranuleAligned(old_s, ipa) &&
       AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, data).state == GranuleAt(old_s, data).state)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).state == RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).state)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).ripas == RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).ripas)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).output_addr == RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).output_addr)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).attr_prot == RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).attr_prot)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).sh == RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, rd, ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).sh)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).measurements[0] == RealmAt(old_s, rd).measurements[0])
}