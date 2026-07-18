pub open spec fn rmi_vsmmu_map_spec(rd: Address, vsmmu_ptr: Address, ipa: Address, level: Int64, result: Result<RmiCommandReturnCode, RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> result == RMI_ERROR_NOT_SUPPORTED)
  && (!AddrIsGranuleAligned(old_s, rd) ==> result == RMI_ERROR_INPUT)
  && (!PaIsDelegable(old_s, rd) ==> result == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != RD ==> result == RMI_ERROR_INPUT)
  && ((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || level < 2) ==> result == RMI_ERROR_INPUT)
  && (RealmAt(old_s, rd).state != REALM_NEW ==> result == RMI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, vsmmu_ptr) ==> result == RMI_ERROR_INPUT)
  && (!PaIsDelegable(old_s, vsmmu_ptr) ==> result == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, vsmmu_ptr).state != VSMMU ==> result == RMI_ERROR_INPUT)
  && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> result == RMI_ERROR_INPUT)
  && ((ipa) < (VsmmuAt(old_s, vsmmu_ptr).reg_base) ==> result == RMI_ERROR_INPUT)
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level < level ==> result == RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level as int))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != EMPTY ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (((ipa) + (RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level) - 1)>= (VsmmuAt(old_s, vsmmu_ptr).reg_top)) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || level < 2)) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       AddrIsGranuleAligned(old_s, vsmmu_ptr) &&
       PaIsDelegable(old_s, vsmmu_ptr) &&
       !(GranuleAt(old_s, vsmmu_ptr).state != VSMMU) &&
       AddrIsRttLevelAligned(old_s, ipa, level as int) &&
       !((ipa) < (VsmmuAt(old_s, vsmmu_ptr).reg_base)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level < level) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != EMPTY) &&
       !(((ipa) + (RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level) - 1)>= (VsmmuAt(old_s, vsmmu_ptr).reg_top))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RealmAt(old_s, rd).state == RealmAt(new_s, rd).state)
  && (result.is_Err()
    ==> RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
}