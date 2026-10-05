pub open spec fn rmi_rtt_map_unprotected_spec(rd: Rd, ipa: UInt64, level: int, desc: RtteDescriptor, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((!(rd as int % GRANULE_SIZE == 0) ==> ResultEqual(result, RMI_ERROR_INPUT))
   ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RdIsDelegablePhysicalAddress(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != GRANULE_STATE_RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!((!(RttLevelValid(old_s, rd, level)) && !(level < 1)) ==> ResultEqual(result, RMI_ERROR_INPUT)))
  && (RttEntryOutputAddressAligned(old_s, rd, ipa, level, desc.addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (ImplFeatures(old_s).feat_lpa2 == FEATURE_FALSE && desc.addr >= 2^48 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttUsesS2AP_INDIRECT(old_s, rd) && !(desc.indirect_base_index == S2AP_NO_ACCESS && desc.indirect_base_index == S2AP_RO && desc.indirect_base_index == S2AP_WO && desc.indirect_base_index == S2AP_RW) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!RttWalk(old_s, rd, ipa, level as int).reached ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, ipa, level as int).level as int)))
  && (RttWalk(old_s, rd, ipa, level as int).entry.state != UNASSIGNED_NS ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, ipa, level as int).level as int)))
  && (result.is_Ok() ==> RttWalk(new_s, rd, ipa, level as int).entry.state == ASSIGNED_NS)
  && (result.is_Ok() ==> RttWalk(new_s, rd, ipa, level as int).entry.unprotected_attributes == desc.unprotected_attributes)
  && (result.is_Ok() && RttUsesS2AP_DIRECT(old_s, rd) ==> RttWalk(new_s, rd, ipa, level as int).entry.read_permission == desc.read_permission)
  && (result.is_Ok() && RttUsesS2AP_DIRECT(old_s, rd) ==> RttWalk(new_s, rd, ipa, level as int).entry.write_permission == desc.write_permission)
  && (result.is_Ok() && RttUsesS2AP_INDIRECT(old_s, rd) ==> RttWalk(new_s, rd, ipa, level as int).entry.indirect_base_index == desc.indirect_base_index)
  && (result.is_Ok() && RttUsesS2AP_INDIRECT(old_s, rd) ==> RttWalk(new_s, rd, ipa, level as int).entry.overlay_index == 15)
  && (result.is_Ok() ==> RttWalk(new_s, rd, ipa, level as int).entry.output_address == desc.addr)
  && ((RttLevelValid(old_s, rd, level) &&
       level >= 1 &&
       RttEntryOutputAddressAligned(old_s, rd, ipa, level, desc.addr) &&
       !(ImplFeatures(old_s).feat_lpa2 == FEATURE_FALSE && desc.addr >= 2^48) &&
       !(RttUsesS2AP_INDIRECT(old_s, rd) && !(desc.indirect_base_index == S2AP_NO_ACCESS && desc.indirect_base_index == S2AP_RO && desc.indirect_base_index == S2AP_WO && desc.indirect_base_index == S2AP_RW)) &&
       RttWalk(old_s, rd, ipa, level as int).reached &&
       !(RttWalk(old_s, rd, ipa, level as int).entry.state != UNASSIGNED_NS))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa, level as int).entry.state == RttWalk(old_s, rd, ipa, level as int).entry.state)
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa, level as int).entry.unprotected_attributes == RttWalk(old_s, rd, ipa, level as int).entry.unprotected_attributes)
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa, level as int).entry.indirect_base_index == RttWalk(old_s, rd, ipa, level as int).entry.indirect_base_index)
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa, level as int).entry.overlay_index == RttWalk(old_s, rd, ipa, level as int).entry.overlay_index)
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa, level as int).entry.output_address == RttWalk(old_s, rd, ipa, level as int).entry.output_address)
}