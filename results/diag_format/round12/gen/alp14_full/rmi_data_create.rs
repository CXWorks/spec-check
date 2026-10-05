pub open spec fn rmi_data_create_spec(rd: Rd, data: PhysAddr, ipa: Ipa, src: PhysAddr, flags: UInt64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((src % GRANULE_SIZE) != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, src).state != GRANULE_ACCESSIBLE ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((data % GRANULE_SIZE) != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (data >= (1 << 48) && !(RealmAt(old_s, rd).impl_features.feat_lpa2 == FEATURE_TRUE) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, data).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((rd % GRANULE_SIZE) != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!CanBeDelegated(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa % GRANULE_SIZE) != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!IsInProtectedIpaSpace(old_s, rd, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (RttWalk(old_s, ipa, RMM_RTT_PAGE_LEVEL as int).state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, ipa, RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (result.is_Ok() ==> GranuleAt(new_s, data).state == DATA)
  && (result.is_Ok() ==> RttEntry(new_s, rd, RttWalk(new_s, ipa, RMM_RTT_PAGE_LEVEL as int).index).state == ASSIGNED)
  && (result.is_Ok() ==> RttEntry(new_s, rd, RttWalk(new_s, ipa, RMM_RTT_PAGE_LEVEL as int).index).ripas == RAM)
  && (result.is_Ok() ==> RttEntry(new_s, rd, RttWalk(new_s, ipa, RMM_RTT_PAGE_LEVEL as int).index).output_addr == data)
  && (result.is_Ok() ==> RttEntry(new_s, rd, RttWalk(new_s, ipa, RMM_RTT_PAGE_LEVEL as int).index).mem_attr == MEMATTR_CACHEABLE)
  && (result.is_Ok() ==> RttEntry(new_s, rd, RttWalk(new_s, ipa, RMM_RTT_PAGE_LEVEL as int).index).shareability == SHAREABILITY_INNER)
  && ((!( (src % GRANULE_SIZE) != 0) &&
       GranuleAt(old_s, src).state == GRANULE_ACCESSIBLE &&
       ((data % GRANULE_SIZE) == 0) &&
       !((data >= (1 << 48)) && !(RealmAt(old_s, rd).impl_features.feat_lpa2 == FEATURE_TRUE)) &&
       GranuleAt(old_s, data).state == DELEGATED &&
       ((rd % GRANULE_SIZE) == 0) &&
       CanBeDelegated(old_s, rd) &&
       GranuleAt(old_s, rd).state == RD &&
       ((ipa % GRANULE_SIZE) == 0) &&
       IsInProtectedIpaSpace(old_s, rd, ipa) &&
       RealmAt(old_s, rd).state == REALM_NEW &&
       !(RttWalk(old_s, ipa, RMM_RTT_PAGE_LEVEL as int).state != UNASSIGNED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, data).state == GranuleAt(old_s, data).state)
  && (result.is_Err()
    ==> RttEntry(new_s, rd, RttWalk(new_s, ipa, RMM_RTT_PAGE_LEVEL as int).index).state == RttEntry(old_s, rd, RttWalk(old_s, ipa, RMM_RTT_PAGE_LEVEL as int).index).state)
  && (result.is_Err()
    ==> RttEntry(new_s, rd, RttWalk(new_s, ipa, RMM_RTT_PAGE_LEVEL as int).index).ripas == RttEntry(old_s, rd, RttWalk(old_s, ipa, RMM_RTT_PAGE_LEVEL as int).index).ripas)
  && (result.is_Err()
    ==> RttEntry(new_s, rd, RttWalk(new_s, ipa, RMM_RTT_PAGE_LEVEL as int).index).output_addr == RttEntry(old_s, rd, RttWalk(old_s, ipa, RMM_RTT_PAGE_LEVEL as int).index).output_addr)
  && (result.is_Err()
    ==> RttEntry(new_s, rd, RttWalk(new_s, ipa, RMM_RTT_PAGE_LEVEL as int).index).mem_attr == RttEntry(old_s, rd, RttWalk(old_s, ipa, RMM_RTT_PAGE_LEVEL as int).index).mem_attr)
  && (result.is_Err()
    ==> RttEntry(new_s, rd, RttWalk(new_s, ipa, RMM_RTT_PAGE_LEVEL as int).index).shareability == RttEntry(old_s, rd, RttWalk(old_s, ipa, RMM_RTT_PAGE_LEVEL as int).index).shareability)
  && (RttWalk(new_s, ipa, RMM_RTT_PAGE_LEVEL as int).state == RttWalk(old_s, ipa, RMM_RTT_PAGE_LEVEL as int).state)
}