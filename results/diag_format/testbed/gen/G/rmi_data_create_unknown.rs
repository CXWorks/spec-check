pub open spec fn rmi_data_create_unknown_spec(rd: Address, data: Address, ipa: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((data) % RMM_GRANULE_SIZE != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, data).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE && data >= 2^48 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((rd) % RMM_GRANULE_SIZE != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) % RMM_GRANULE_SIZE != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsWithin(old_s, ipa, RealmAt(old_s, rd).rtt_base[0], RealmAt(old_s, rd).rtt_base[0]) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_RTT(RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)) ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level == RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_RTT(RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)) ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).state == RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).state)
  && (result.is_Ok() ==> GranuleAt(new_s, data).state == DATA)
  && (result.is_Ok() ==> GranuleAt(new_s, data).state == DATA)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).state == ASSIGNED)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).rtte_addr == data)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).attr_prot == MEMATTR_CACHEABLE)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).sh == SHAREABILITY_INNER)
  && ((!( (data) % RMM_GRANULE_SIZE != 0) &&
       PaIsDelegable(old_s, data) &&
       !(GranuleAt(old_s, data).state != DELEGATED) &&
       !((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE) && (data >= 2^48)) &&
       ((rd) % RMM_GRANULE_SIZE == 0) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       ((ipa) % RMM_GRANULE_SIZE == 0) &&
       AddrIsWithin(old_s, ipa, RealmAt(old_s, rd).rtt_base[0], RealmAt(old_s, rd).rtt_base[0]))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, data).state == GranuleAt(old_s, data).state)
  && (result.is_Err()
    ==> GranuleAt(new_s, data).state == GranuleAt(old_s, data).state)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).state == RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).state)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).rtte_addr == RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).rtte_addr)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).attr_prot == RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).attr_prot)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).sh == RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)).sh)
}