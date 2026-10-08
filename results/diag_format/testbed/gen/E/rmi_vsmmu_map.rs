pub open spec fn rmi_vsmmu_map_spec(rd: Address, vsmmu_ptr: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!ImplFeatures(old_s).feat_da ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!(RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) && level >= 2) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vsmmu_ptr).state != VSMMU ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsAligned(old_s, ipa, pow2(level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (ipa < VsmmuAt(old_s, vsmmu_ptr).reg_base ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level < level ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level as int)))
  && (RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level)).state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level as int)))
  && (RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level)).ripas != EMPTY ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level as int)))
  && (RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level)).addr + pow2(RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level) >= VsmmuAt(old_s, vsmmu_ptr).reg_top ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level as int)))
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level)).state == ASSIGNED_VSMMU)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level)).addr == vsmmu_ptr)
  && ((!(ImplFeatures(old_s).feat_da) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       (RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) && !(level >= 2)) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       AddrIsGranuleAligned(old_s, vsmmu_ptr) &&
       PaIsDelegable(old_s, vsmmu_ptr) &&
       !(GranuleAt(old_s, vsmmu_ptr).state != VSMMU) &&
       AddrIsAligned(old_s, ipa, pow2(level as int)) &&
       !(ipa < VsmmuAt(old_s, vsmmu_ptr).reg_base) &&
       !(RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level < level) &&
       !(RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level)).state != UNASSIGNED) &&
       !(RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level)).ripas != EMPTY) &&
       !(RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level)).addr + pow2(RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level) >= VsmmuAt(old_s, vsmmu_ptr).reg_top))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level)).state == RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level)).state)
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level)).addr == RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level)).addr)
  )
}