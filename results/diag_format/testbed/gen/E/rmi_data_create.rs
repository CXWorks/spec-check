pub open spec fn rmi_data_create_spec(rd: Address, data: Address, ipa: Address, src: Address, flags: RmiDataFlags, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, src) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, src, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, data).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (data >= 2^48 && RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level < RMM_RTT_PAGE_LEVEL ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Ok() ==> GranuleAt(new_s, data).state == DATA)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).state == ASSIGNED)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).ripas == RAM)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).out == data)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).attr == MEMATTR_CACHEABLE)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).sh == SHAREABILITY_INNER)
  && (result.is_Ok() ==> RealmAt(new_s, rd).measurements[0] == RimExtendData(new_s, RealmAt(new_s, rd), data, flags))
  && ((AddrIsGranuleAligned(old_s, src) &&
       GranuleAccessPermitted(old_s, src, PAS_NS) &&
       AddrIsGranuleAligned(old_s, data) &&
       PaIsDelegable(old_s, data) &&
       !(GranuleAt(old_s, data).state != DELEGATED) &&
       !(data >= 2^48 && RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, ipa) &&
       AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level < RMM_RTT_PAGE_LEVEL) &&
       !(RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).state != UNASSIGNED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, data).state == GranuleAt(old_s, data).state)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).state == RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).state)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).ripas == RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).ripas)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).out == RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).out)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).attr == RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).attr)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).sh == RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)).sh)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).measurements[0] == RealmAt(old_s, rd).measurements[0])
  && (!(result.is_Ok() && (GranuleAt(new_s, data).state != DATA))
    ==> GranuleAt(new_s, data).state == DATA)
}