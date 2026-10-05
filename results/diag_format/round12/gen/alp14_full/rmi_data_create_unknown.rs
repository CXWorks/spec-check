pub open spec fn rmi_data_create_unknown_spec(rd: Rd, data: PhysAddr, ipa: Int, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((data) % GRANULE_SIZE == 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (is_delegable_physical_address_in_dram(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, data).state == DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RealmAt(old_s, rd).feat_lpa2 == FEATURE_TRUE && data >= 2^48 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((rd) % GRANULE_SIZE == 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (is_delegable_physical_address(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state == RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) % GRANULE_SIZE == 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (ipa within_protected_ipa_space(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> GranuleAt(new_s, data).state == DATA)
  && (result.is_Ok() ==> RTTEntryAt(new_s, ipa, RttWalk(new_s, ipa).level as int).state == ASSIGNED)
  && (result.is_Ok() ==> RTTEntryAt(new_s, ipa, RttWalk(new_s, ipa).level as int).output_address == data)
  && (result.is_Ok() ==> RTTEntryAt(new_s, ipa, RttWalk(new_s, ipa).level as int).memory_attribute == MEMATTR_CACHEABLE)
  && (result.is_Ok() ==> RTTEntryAt(new_s, ipa, RttWalk(new_s, ipa).level as int).shareability == SHAREABILITY_INNER)
  && ((!( (data) % GRANULE_SIZE == 0) &&
       is_delegable_physical_address_in_dram(old_s, data) &&
       !(GranuleAt(old_s, data).state == DELEGATED) &&
       (RealmAt(old_s, rd).feat_lpa2 == FEATURE_TRUE || !(data >= 2^48)) &&
       !((rd) % GRANULE_SIZE == 0) &&
       is_delegable_physical_address(old_s, rd) &&
       !(GranuleAt(old_s, rd).state == RD) &&
       !((ipa) % GRANULE_SIZE == 0) &&
       !(ipa within_protected_ipa_space(old_s, rd)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, data).state == GranuleAt(old_s, data).state)
  && (result.is_Err()
    ==> RTTEntryAt(new_s, ipa, RttWalk(new_s, ipa).level as int).state == RTTEntryAt(old_s, ipa, RttWalk(old_s, ipa).level as int).state)
  && (result.is_Err()
    ==> RTTEntryAt(new_s, ipa, RttWalk(new_s, ipa).level as int).output_address == RTTEntryAt(old_s, ipa, RttWalk(old_s, ipa).level as int).output_address)
  && (result.is_Err()
    ==> RTTEntryAt(new_s, ipa, RttWalk(new_s, ipa).level as int).memory_attribute == RTTEntryAt(old_s, ipa, RttWalk(old_s, ipa).level as int).memory_attribute)
  && (result.is_Err()
    ==> RTTEntryAt(new_s, ipa, RttWalk(new_s, ipa).level as int).shareability == RTTEntryAt(old_s, ipa, RttWalk(old_s, ipa).level as int).shareability)
}