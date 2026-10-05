pub open spec fn rmi_rtt_map_unprotected_spec(rd: Address, ipa: Address, level: Int64, desc: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding) != valid_rtte_for_unprotected_ipa(old_s, desc) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((rd) % granule_size(old_s) != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!is_delegable_physical_address(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttGranuleState(old_s, RttAt(old_s, RttWalk(old_s,  RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY)).rtt_addr), RttEntryIndex(old_s,  ipa, RttWalk(old_s,  RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).level as int)) != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!is_valid_rtt_level(old_s, RealmAt(old_s, rd), level) || level < 1 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding).addr % address_range_size_for_rtt_entry_at_level(old_s, RttWalk(old_s,  RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).level as int) != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!ImplFeatures(old_s).feat_lpa2 == FEATURE_FALSE) && (RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding).addr >= 2^48) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (ipa % address_range_size_for_rtt_entry_at_level(old_s, level as int) != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (ipa >= pow2(RealmAt(old_s, rd).ipa_width) || is_protected_ipa(old_s, RealmAt(old_s, rd), ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).rtt_s2ap_encoding == S2AP_INDIRECT && (RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding).indirect_base_index != S2AP_NO_ACCESS && RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding).indirect_base_index != S2AP_RO && RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding).indirect_base_index != S2AP_WO && RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding).indirect_base_index != S2AP_RW) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s,  RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).level < level ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s,  RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).level as int)))
  && (RttDescriptorDecode(new_s, RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding),RealmAt(new_s, rd).rtt_s2ap_encoding) != UNASSIGNED_NS ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s,  RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).level as int)))
  && (result.is_Ok() ==> RttDescriptorDecode(new_s, RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding),RealmAt(new_s, rd).rtt_s2ap_encoding) == ASSIGNED_NS)
  && (result.is_Ok() ==> Unprotected_attributes(RttDescriptorDecode(new_s, RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding),RealmAt(new_s, rd).rtt_s2ap_encoding))) == Unprotected_attributes(new_s, desc))
  && (result.is_Ok() && RealmAt(old_s, rd).rtt_s2ap_encoding == S2AP_DIRECT ==> read_write_permissions(RttDescriptorDecode(new_s, RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding),RealmAt(new_s, rd).rtt_s2ap_encoding))) == read_write_permissions(new_s, desc))
  && (result.is_Ok() && RealmAt(old_s, rd).rtt_s2ap_encoding == S2AP_INDIRECT ==> indirect_base_index(RttDescriptorDecode(new_s, RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding),RealmAt(new_s, rd).rtt_s2ap_encoding))) == indirect_base_index(new_s, desc))
  && (result.is_Ok() && RealmAt(old_s, rd).rtt_s2ap_encoding == S2AP_INDIRECT ==> overlay_index(RttDescriptorDecode(new_s, RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding),RealmAt(new_s, rd).rtt_s2ap_encoding))) == 15)
  && (result.is_Ok() ==> output_address(RttDescriptorDecode(new_s, RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding),RealmAt(new_s, rd).rtt_s2ap_encoding))) == output_address(new_s, desc))
  && ((!(RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding) != valid_rtte_for_unprotected_ipa(old_s, desc)) &&
       ((rd) % granule_size(old_s) == 0) &&
       (is_delegable_physical_address(old_s, rd)) &&
       (RttGranuleState(old_s, RttAt(old_s, RttWalk(old_s,  RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY)).rtt_addr), RttEntryIndex(old_s,  ipa, RttWalk(old_s,  RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).level as int)) == RD) &&
       (is_valid_rtt_level(old_s, RealmAt(old_s, rd), level) && !(level < 1)) &&
       (RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding).addr % address_range_size_for_rtt_entry_at_level(old_s, RttWalk(old_s,  RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).level as int) == 0) &&
       (!((!ImplFeatures(old_s).feat_lpa2 == FEATURE_FALSE) && (RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding).addr >= 2^48))) &&
       (ipa % address_range_size_for_rtt_entry_at_level(old_s, level as int) == 0) &&
       !((ipa >= pow2(RealmAt(old_s, rd).ipa_width)) || is_protected_ipa(old_s, RealmAt(old_s, rd), ipa)) &&
       !((RealmAt(old_s, rd).rtt_s2ap_encoding == S2AP_INDIRECT) && (RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding).indirect_base_index != S2AP_NO_ACCESS && RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding).indirect_base_index != S2AP_RO && RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding).indirect_base_index != S2AP_WO && RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding).indirect_base_index != S2AP_RW)) &&
       !(RttWalk(old_s,  RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).level < level) &&
       !(RttDescriptorDecode(old_s, RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding),RealmAt(old_s, rd).rtt_s2ap_encoding) != UNASSIGNED_NS))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttDescriptorDecode(new_s, RttDescriptorDecode(new_s, desc,RealmAt(new_s, rd).rtt_s2ap_encoding),RealmAt(new_s, rd).rtt_s2ap_encoding) == RttDescriptorDecode(old_s, RttDescriptorDecode(old_s, desc,RealmAt(old_s, rd).rtt_s2ap_encoding),RealmAt(old_s, rd).rtt_s2ap_encoding))
  )
}