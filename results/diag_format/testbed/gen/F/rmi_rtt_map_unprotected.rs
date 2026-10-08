pub open spec fn rmi_rtt_map_unprotected_spec(rd: Address, ipa: Address, level: Int64, desc: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!((rd as int) % RmmGranuleSize(old_s) != 0) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleState(old_s, rd) == RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttLevelIsValid(old_s, RealmAt(old_s, rd), level) || level < 1 ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsAligned(old_s, (desc as int) & (RttLevelSize(level as int) - 1), RttLevelSize(level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!((RealmAt(old_s, rd).feat_lpa2 == RmmFeature::FEATURE_FALSE) && ((ipa as int) >= (1u64 << 48)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsAligned(old_s, ipa, RttLevelSize(level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!((ipa as int) >= (1u64 << (RealmAt(old_s, rd).ipa_width as int)) || GranuleAccessPermitted(old_s, ipa, RealmAt(old_s, rd).state) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RealmAt(old_s, rd).rtt_s2ap_encoding == RmmRttS2APEncoding::S2AP_INDIRECT && !((RttDescriptorDecode(desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == RmmRttS2APBase::S2AP_NO_ACCESS) || (RttDescriptorDecode(desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == RmmRttS2APBase::S2AP_RO) || (RttDescriptorDecode(desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == RmmRttS2APBase::S2AP_WO) || (RttDescriptorDecode(desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == RmmRttS2APBase::S2AP_RW)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level, RMM_RTT_TREE_PRIMARY).level < level ==> ResultEqual(result, RMI_ERROR_RTT))
    && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level, RMM_RTT_TREE_PRIMARY).rtte.state != RmmRttEntryState::UNASSIGNED_NS ==> ResultEqual(result, RMI_ERROR_RTT))
    && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level, RMM_RTT_TREE_PRIMARY).rtte.state == RmmRttEntryState::ASSIGNED_NS && !((RttWalk(old_s, RealmAt(old_s, rd), ipa, level, RMM_RTT_TREE_PRIMARY).rtte.attr_unprot == (desc as int) & 0x7)) ==> true)
    && (RealmAt(old_s, rd).rtt_s2ap_encoding == RmmRttS2APEncoding::S2AP_DIRECT && !((RttWalk(old_s, RealmAt(old_s, rd), ipa, level, RMM_RTT_TREE_PRIMARY).rtte.s2ap_direct.read == ((desc as int) >> 56) & 1) && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level, RMM_RTT_TREE_PRIMARY).rtte.s2ap_direct.write == ((desc as int) >> 55) & 1)) ==> true)
    && (RealmAt(old_s, rd).rtt_s2ap_encoding == RmmRttS2APEncoding::S2AP_INDIRECT && !((RttWalk(old_s, RealmAt(old_s, rd), ipa, level, RMM_RTT_TREE_PRIMARY).rtte.s2ap_indirect.base_index == (desc as int) >> 56) && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level, RMM_RTT_TREE_PRIMARY).rtte.s2ap_indirect.overlay_index == 15)) ==> true)
    && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level, RMM_RTT_TREE_PRIMARY).rtte.addr != (desc as int) & 0xFFFFFFFFFFFFFFFF) ==> true
}