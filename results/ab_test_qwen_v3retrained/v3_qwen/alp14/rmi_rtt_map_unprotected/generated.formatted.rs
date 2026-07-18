pub open spec fn rmi_rtt_map_unprotected_spec(rd: Address, ipa: Address, level: Int64, desc: Bits64, result: Result<RmiCommandReturnCode, RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!RttDescriptorIsValidForUnprotected(old_s, desc) ==> result == RMI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, rd) ==> result == RMI_ERROR_INPUT)
  && (!PaIsDelegable(old_s, rd) ==> result == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != RD ==> result == RMI_ERROR_INPUT)
  && ((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || level < 1) ==> result == RMI_ERROR_INPUT)
  && (!AddrIsRttLevelAligned(old_s, RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).addr, level as int) ==> result == RMI_ERROR_INPUT)
  && (((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).addr) >= 2^48)) ==> result == RMI_ERROR_INPUT)
  && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> result == RMI_ERROR_INPUT)
  && (((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))) ==> result == RMI_ERROR_INPUT)
  && ((RealmAt(old_s, rd).rtt_s2ap_encoding == S2AP_INDIRECT && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index != S2AP_NO_ACCESS && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index != S2AP_RO && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index != S2AP_WO && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index != S2AP_RW) ==> result == RMI_ERROR_INPUT)
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).level < level ==> result == RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).level))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.state != UNASSIGNED_NS ==> result == RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).level))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.state == ASSIGNED_NS)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.attr_unprot == RttDescriptorDecode(new_s, desc, RealmAt(new_s, rd).rtt_s2ap_encoding).attr_unprot)
  && (result.is_Ok() && RealmAt(old_s, rd).rtt_s2ap_encoding == S2AP_DIRECT ==> (RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.s2ap_direct.read == RttDescriptorDecode(new_s, desc, RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_direct.read && RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.s2ap_direct.write == RttDescriptorDecode(new_s, desc, RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_direct.write))
  && (result.is_Ok() && RealmAt(old_s, rd).rtt_s2ap_encoding == S2AP_INDIRECT ==> (RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.s2ap_indirect.base_index == RttDescriptorDecode(new_s, desc, RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index && RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.s2ap_indirect.overlay_index == 15))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.addr == RttDescriptorDecode(new_s, desc, RealmAt(new_s, rd).rtt_s2ap_encoding).addr)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || level < 1)) &&
       AddrIsRttLevelAligned(old_s, RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).addr, level as int) &&
       !(((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).addr) >= 2^48))) &&
       AddrIsRttLevelAligned(old_s, ipa, level as int) &&
       !(((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)))) &&
       !((RealmAt(old_s, rd).rtt_s2ap_encoding == S2AP_INDIRECT && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index != S2AP_NO_ACCESS && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index != S2AP_RO && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index != S2AP_WO && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index != S2AP_RW)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).level < level) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.state != UNASSIGNED_NS))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.attr_unprot == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.attr_unprot)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.s2ap_direct.read == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.s2ap_direct.read)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.s2ap_direct.write == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.s2ap_direct.write)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.s2ap_indirect.base_index == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.s2ap_indirect.base_index)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY).rtte.s2ap_indirect.overlay_index == RttWalk(old_s, RealmAt