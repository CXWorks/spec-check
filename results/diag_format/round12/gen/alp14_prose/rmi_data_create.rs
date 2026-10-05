pub open spec fn rmi_data_create_spec(rd: Address, data: Address, ipa: Address, src: Address, flags: RmiDataFlags, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (src % GRANULE_SIZE == 0 ==> result.is_Ok())
  && (result.is_Ok() && !can_access_from_ns(old_s, src) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (data % GRANULE_SIZE == 0 ==> result.is_Ok())
  && (result.is_Ok() && !is_delegatable_dram(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() && GranuleAt(new_s, data).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((data) >= (1 << 48) && !(ImplFeatures(new_s).feat_lpa2 == FEATURE_TRUE) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (rd % GRANULE_SIZE == 0 ==> result.is_Ok())
  && (result.is_Ok() && !is_delegatable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() && GranuleAt(new_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (ipa % GRANULE_SIZE == 0 ==> result.is_Ok())
  && (result.is_Ok() && !is_in_protected_ipa_space(old_s, RealmAt(new_s, rd), ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() && RealmAt(new_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (result.is_Ok() && RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level != RMM_RTT_PAGE_LEVEL ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Ok() && RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)) != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Err() ==> GranuleAt(new_s, data).state == DATA)
  && (result.is_Ok() ==> GranuleAt(new_s, data).state == DATA)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).state == ASSIGNED)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).ripas == RAM)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).output_addr == data)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).mem_attr == MEMATTR_CACHEABLE)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).shareability == SHAREABILITY_INNER)
  && ((!(src % GRANULE_SIZE == 0) &&
       can_access_from_ns(old_s, src) &&
       !(data % GRANULE_SIZE == 0) &&
       is_delegatable_dram(old_s, data) &&
       GranuleAt(old_s, data).state == DELEGATED &&
       !((data) >= (1 << 48) && !(ImplFeatures(old_s).feat_lpa2 == FEATURE_TRUE)) &&
       !(rd % GRANULE_SIZE == 0) &&
       is_delegatable(old_s, rd) &&
       GranuleAt(old_s, rd).state == RD &&
       !(ipa % GRANULE_SIZE == 0) &&
       is_in_protected_ipa_space(old_s, RealmAt(old_s, rd), ipa) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level != RMM_RTT_PAGE_LEVEL) &&
       !(RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)) != UNASSIGNED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).state == RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).state)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).ripas == RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).ripas)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).output_addr == RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).output_addr)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).mem_attr == RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).mem_attr)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).shareability == RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).shareability)
  && (!(result.is_Ok() && (RealmAt(new_s, rd).state == REALM_NEW)) ==> RealmAt(new_s, rd).state == RealmAt(old_s, rd).state)
}