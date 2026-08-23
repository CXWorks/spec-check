pub open spec fn rmi_rtt_map_unprotected_spec(rd: Address, ipa: Address, level: Int64, desc: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level as int, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let rtte = RttDescriptorDecode(old_s, desc, realm.rtt_s2ap_encoding);
    let new_rtte = RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx);
    (!RttDescriptorIsValidForUnprotected(old_s, desc) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!RttLevelIsValid(old_s, realm, level as int) || (level as int) < 1) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, rtte.addr, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((realm.feat_lpa2 == FEATURE_FALSE && (rtte.addr as int) >= 0x1_0000_0000_0000) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((ipa as int) >= ((1u64 << realm.ipa_width) as int) || AddrIsProtected(old_s, ipa, realm)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((realm.rtt_s2ap_encoding == S2AP_INDIRECT
            && rtte.s2ap_indirect.base_index != S2AP_NO_ACCESS
            && rtte.s2ap_indirect.base_index != S2AP_RO
            && rtte.s2ap_indirect.base_index != S2AP_WO
            && rtte.s2ap_indirect.base_index != S2AP_RW) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((walk.level < level as int) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    && ((walk.rtte.state != UNASSIGNED_NS) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    && ((RttDescriptorIsValidForUnprotected(old_s, desc)
            && AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && GranuleAt(old_s, rd).state == RD
            && RttLevelIsValid(old_s, realm, level as int)
            && (level as int) >= 1
            && AddrIsRttLevelAligned(old_s, rtte.addr, level as int)
            && !(realm.feat_lpa2 == FEATURE_FALSE && (rtte.addr as int) >= 0x1_0000_0000_0000)
            && AddrIsRttLevelAligned(old_s, ipa, level as int)
            && (ipa as int) < ((1u64 << realm.ipa_width) as int)
            && !AddrIsProtected(old_s, ipa, realm)
            && !(realm.rtt_s2ap_encoding == S2AP_INDIRECT
                    && rtte.s2ap_indirect.base_index != S2AP_NO_ACCESS
                    && rtte.s2ap_indirect.base_index != S2AP_RO
                    && rtte.s2ap_indirect.base_index != S2AP_WO
                    && rtte.s2ap_indirect.base_index != S2AP_RW)
            && walk.level == level as int
            && walk.rtte.state == UNASSIGNED_NS)
        ==> (result.is_Ok()
            && new_rtte.state == ASSIGNED_NS
            && new_rtte.attr_unprot == rtte.attr_unprot
            && new_rtte.addr == rtte.addr
            && (realm.rtt_s2ap_encoding == S2AP_DIRECT
                ==> (new_rtte.s2ap_direct.read == rtte.s2ap_direct.read
                    && new_rtte.s2ap_direct.write == rtte.s2ap_direct.write))
            && (realm.rtt_s2ap_encoding == S2AP_INDIRECT
                ==> (new_rtte.s2ap_indirect.base_index == rtte.s2ap_indirect.base_index
                    && new_rtte.s2ap_indirect.overlay_index == 15))))
}