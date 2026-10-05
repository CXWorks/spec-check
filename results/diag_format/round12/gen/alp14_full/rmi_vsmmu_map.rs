pub open spec fn rmi_vsmmu_map_spec(rd: RmmRealmDescriptor, vsmmu_ptr: Address, ipa: Address, level: int, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (RmmGranuleState(old_s, rd) != RD ==> RMI_ERROR_INPUT(result))
  && (!IsDelegableAddress(old_s, rd) ==> RMI_ERROR_INPUT(result))
  && (RmmGranuleState(old_s, rd) != IN_RD ==> RMI_ERROR_INPUT(result))
  && (!IsValidRttLevel(old_s, rd, level) || level < 2 ==> RMI_ERROR_INPUT(result))
  && (RealmAt(old_s, rd).state != REALM_NEW ==> RMI_ERROR_INPUT(result))
  && (RmmGranuleState(old_s, vsmmu_ptr) != VSMMU ==> RMI_ERROR_INPUT(result))
  && (!IsDelegableAddress(old_s, vsmmu_ptr) ==> RMI_ERROR_INPUT(result))
  && (RmmGranuleState(old_s, vsmmu_ptr) != IN_VSMMU ==> RMI_ERROR_INPUT(result))
  && (!IsAligned(old_s, ipa, RttEntryMapSize(old_s, RealmAt(old_s, rd).rtt_level_start as int, level as int)) ==> RMI_ERROR_INPUT(result))
  && (ipa < VSMMURegisterRegionBase(old_s, vsmmu_ptr) ==> RMI_ERROR_INPUT(result))
  && (RttWalk(old_s, ipa, level as int).level < level ==> RMI_ERROR_RTT(result, RttWalk(new_s, ipa, level as int).level as int))
  && (RttWalk(old_s, ipa, level as int).entry.state != UNASSIGNED ==> RMI_ERROR_RTT(result, RttWalk(new_s, ipa, level as int).level as int))
  && (RttWalk(old_s, ipa, level as int).entry.ripas != EMPTY ==> RMI_ERROR_RTT(result, RttWalk(new_s, ipa, level as int).level as int))
  && (RttWalk(old_s, ipa, level as int).end >= VSMMURegisterRegionTop(old_s, vsmmu_ptr) ==> RMI_ERROR_RTT(result, RttWalk(new_s, ipa, level as int).level as int))
  && (result.is_Ok() ==> RttWalk(new_s, ipa, level as int).entry.state == ASSIGNED_VSMMU)
  && (result.is_Ok() ==> RttWalk(new_s, ipa, level as int).entry.addr == vsmmu_ptr)
  && ((!(RmmGranuleState(old_s, rd) != RD) &&
       IsDelegableAddress(old_s, rd) &&
       !(RmmGranuleState(old_s, rd) != IN_RD) &&
       (IsValidRttLevel(old_s, rd, level) && !(level < 2)) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       !(RmmGranuleState(old_s, vsmmu_ptr) != VSMMU) &&
       IsDelegableAddress(old_s, vsmmu_ptr) &&
       !(RmmGranuleState(old_s, vsmmu_ptr) != IN_VSMMU) &&
       IsAligned(old_s, ipa, RttEntryMapSize(old_s, RealmAt(old_s, rd).rtt_level_start as int, level as int)) &&
       !(ipa < VSMMURegisterRegionBase(old_s, vsmmu_ptr)) &&
       !(RttWalk(old_s, ipa, level as int).level < level) &&
       !(RttWalk(old_s, ipa, level as int).entry.state != UNASSIGNED) &&
       !(RttWalk(old_s, ipa, level as int).entry.ripas != EMPTY) &&
       !(RttWalk(old_s, ipa, level as int).end >= VSMMURegisterRegionTop(old_s, vsmmu_ptr)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, ipa, level as int).entry.state == RttWalk(old_s, ipa, level as int).entry.state)
  && (result.is_Err()
    ==> RttWalk(new_s, ipa, level as int).entry.addr == RttWalk(old_s, ipa, level as int).entry.addr)
  && (RttWalk(new_s, ipa, level as int).entry.ripas == RttWalk(old_s, ipa, level as int).entry.ripas)
}