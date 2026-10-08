pub open spec fn rmi_vsmmu_map_spec(rd: Address, vsmmu_ptr: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!ImplFeatures(old_s).feat_da == RMM_FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state == RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) && level >= 2 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state == REALM_NEW ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (AddrIsGranuleAligned(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PaIsDelegable(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vsmmu_ptr).state == VSMMU ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (ipa < VsmmuAt(old_s, vsmmu_ptr).reg_base ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Err() ==> GranuleAt(new_s, rd).state == GranuleAt(old_s, rd).state)
  && (result.is_Err() ==> GranuleAt(new_s, vsmmu_ptr).state == GranuleAt(old_s, vsmmu_ptr).state)
  && (result.is_Ok() ==> RttWalk_(new_s, rd, ipa, level as int).rtte.state == ASSIGNED_VSMMU)
  && (result.is_Ok() ==> RttWalk_(new_s, rd, ipa, level as int).rtte.addr == vsmmu_ptr)
  && ((!(ImplFeatures(old_s).feat_da == RMM_FEATURE_FALSE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state == RD) &&
       !(RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) && level >= 2) &&
       !(RealmAt(old_s, rd).state == REALM_NEW) &&
       !(AddrIsGranuleAligned(old_s, vsmmu_ptr)) &&
       !(PaIsDelegable(old_s, vsmmu_ptr)) &&
       !(GranuleAt(old_s, vsmmu_ptr).state == VSMMU) &&
       AddrIsRttLevelAligned(old_s, ipa, level as int) &&
       !(ipa < VsmmuAt(old_s, vsmmu_ptr).reg_base))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk_(new_s, rd, ipa, level as int).rtte.state == RttWalk_(old_s, rd, ipa, level as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk_(new_s, rd, ipa, level as int).rtte.addr == RttWalk_(old_s, rd, ipa, level as int).rtte.addr)
}