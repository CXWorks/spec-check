pub open spec fn rmi_rtt_map_unprotected_spec(rd: Address, ipa: Address, level: Int64, desc: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!(RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).attr_unprot == 0) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!(RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) && level > 0) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!(RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).addr % RttLevelSize(old_s, level as int) == 0) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).addr >= 2^48) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!(ipa % RttLevelSize(old_s, level as int) == 0) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!(ipa >= pow2(RealmAt(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, rd, RealmAt(old_s, rd))) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RealmAt(old_s, rd).rtt_s2ap_encoding == S2AP_INDIRECT && !(RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == S2AP_NO_ACCESS && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == S2AP_RO && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == S2AP_WO && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == S2AP_RW)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level < level ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level as int)))
  && (RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).rtte.state != UNASSIGNED_NS ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level as int)))
  && (result.is_Ok() ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).rtte.state == ASSIGNED_NS)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level as int)).attr_unprot == RttDescriptorDecode(new_s, desc, RealmAt(new_s, rd).rtt_s2ap_encoding).attr_unprot)
  && (result.is_Ok() && (RealmAt(old_s, rd).rtt_s2ap_encoding == S2AP_DIRECT) ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level as int)).s2ap_direct.read == RttDescriptorDecode(new_s, desc, RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_direct.read)
  && (result.is_Ok() && (RealmAt(old_s, rd).rtt_s2ap_encoding == S2AP_DIRECT) ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level as int)).s2ap_direct.write == RttDescriptorDecode(new_s, desc, RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_direct.write)
  && (result.is_Ok() && (RealmAt(old_s, rd).rtt_s2ap_encoding == S2AP_INDIRECT) ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level as int)).s2ap_indirect.base_index == RttDescriptorDecode(new_s, desc, RealmAt(new_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index)
  && (result.is_Ok() && (RealmAt(old_s, rd).rtt_s2ap_encoding == S2AP_INDIRECT) ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level as int)).s2ap_indirect.overlay_index == 15)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level as int)).addr == RttDescriptorDecode(new_s, desc, RealmAt(new_s, rd).rtt_s2ap_encoding).addr)
  && ((!(RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).attr_unprot == 0) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       (RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) && !(level > 0)) &&
       (RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).addr % RttLevelSize(old_s, level as int) == 0) &&
       !((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).addr >= 2^48)) &&
       (ipa % RttLevelSize(old_s, level as int) == 0) &&
       (ipa >= pow2(RealmAt(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, rd, RealmAt(old_s, rd))) &&
       !((RealmAt(old_s, rd).rtt_s2ap_encoding == S2AP_INDIRECT && !(RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == S2AP_NO_ACCESS && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == S2AP_RO && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == S2AP_WO && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == S2AP_RW))) &&
    !(RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level < level) &&
    !(RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).rtte.state != UNASSIGNED_NS))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).rtte.state == RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).rtte.state)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level as int)).attr_unprot == RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level as int)).attr_unprot)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level as int)).s2ap_direct.read == RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level as int)).s2ap_direct.read)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level as int)).s2ap_direct.write == RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level as int)).s2ap_direct.write)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level as int)).s2ap_indirect.base_index == RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level as int)).s2ap_indirect.base_index)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level as int)).s2ap_indirect.overlay_index == RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level as int)).s2ap_indirect.overlay_index)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk_(new_s, RealmAt(new_s, rd), ipa, level as int).level as int)).addr == RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa, level as int).level as int)).addr)
}