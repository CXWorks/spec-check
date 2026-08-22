pub open spec fn rmi_rtt_map_unprotected_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S, rd: Address, ipa: Address, level: i64, desc: u64) -> bool {
    let realm = RealmAt(old_s, rd);
    let rtte = RttDescriptorDecode(old_s, desc, realm.rtt_s2ap_encoding);
    let walk = RttWalk(old_s, realm, ipa, level as int, RMM_RTT_TREE_PRIMARY);
    let new_walk = RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY);
    (!RttDescriptorIsValidForUnprotected(old_s, desc) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!RttLevelIsValid(old_s, realm, level as int) || level < 1) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, rtte.addr, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((realm.feat_lpa2 == FEATURE_FALSE && (rtte.addr as int) >= 0x1_0000_0000_0000) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((ipa as int) >= ((1u64 << (realm.ipa_width as u64)) as int) || AddrIsProtected(old_s, ipa, realm)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((realm.rtt_s2ap_encoding == S2AP_INDIRECT
            && rtte.s2ap_indirect.base_index != S2AP_NO_ACCESS
            && rtte.s2ap_indirect.base_index != S2AP_RO
            && rtte.s2ap_indirect.base_index != S2AP_WO
            && rtte.s2ap_indirect.base_index != S2AP_RW) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.level < level as int ==> ResultEqual(result, RMI_ERROR_RTT))
    && (walk.rtte.state != UNASSIGNED_NS ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((RttDescriptorIsValidForUnprotected(old_s, desc)
            && AddrIsGranuleAligned(rd)
            && PaIsDelegable(old_s, rd)
            && GranuleAt(old_s, rd).state == RD
            && RttLevelIsValid(old_s, realm, level as int)
            && level >= 1
            && AddrIsRttLevelAligned(old_s, rtte.addr, level as int)
            && !(realm.feat_lpa2 == FEATURE_FALSE && (rtte.addr as int) >= 0x1_0000_0000_0000)
            && AddrIsRttLevelAligned(old_s, ipa, level as int)
            && (ipa as int) < ((1u64 << (realm.ipa_width as u64)) as int)
            && !AddrIsProtected(old_s, ipa, realm)
            && !(realm.rtt_s2ap_encoding == S2AP_INDIRECT
                    && rtte.s2ap_indirect.base_index != S2AP_NO_ACCESS
                    && rtte.s2ap_indirect.base_index != S2AP_RO
                    && rtte.s2ap_indirect.base_index != S2AP_WO
                    && rtte.s2ap_indirect.base_index != S2AP_RW)
            && walk.level >= level as int
            && walk.rtte.state == UNASSIGNED_NS)
        ==> (result.is_Ok()
            && new_walk.rtte.state == ASSIGNED_NS
            && new_walk.rtte.attr_unprot == rtte.attr_unprot
            && (realm.rtt_s2ap_encoding == S2AP_DIRECT ==>
                    (new_walk.rtte.s2ap_direct.read == rtte.s2ap_direct.read
                        && new_walk.rtte.s2ap_direct.write == rtte.s2ap_direct.write))
            && (realm.rtt_s2ap_encoding == S2AP_INDIRECT ==>
                    (new_walk.rtte.s2ap_indirect.base_index == rtte.s2ap_indirect.base_index
                        && new_walk.rtte.s2ap_indirect.overlay_index == 15))
            && new_walk.rtte.addr == rtte.addr))
}