pub open spec fn rmi_rtt_read_entry_spec(result: Result<(), RmiStatusCode>, rd: Address, ipa: Address, level: i64, walk_level: u64, state: RmiRttEntryState, desc: u64, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && GranuleAt(old_s, rd).state == RD
            && RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int)
            && AddrIsRttLevelAligned(old_s, ipa, level as int))
        ==> (result.is_Ok()
            && walk_level == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).level
            && state == RttEntryStateToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).rtte.state)
            && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == UNASSIGNED
                    || RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == UNASSIGNED_NS)
                ==> (RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).attr_unprot == 0
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == S2AP_NO_ACCESS
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.overlay_index == 0
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_direct.read == RMM_FALSE
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_direct.write == RMM_FALSE
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).addr == 0))
            && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED
                    || RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_DEV
                    || RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_VSMMU
                    || RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == TABLE)
                ==> (RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).attr_unprot == 0
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == S2AP_NO_ACCESS
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.overlay_index == 0
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_direct.read == RMM_FALSE
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_direct.write == RMM_FALSE
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).addr == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr))
            && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_NS
                ==> (RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).attr_unprot == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).rtte.attr_unprot
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_indirect.base_index
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.overlay_index == 0
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_direct.read == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_direct.read
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_direct.write == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_direct.write
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).addr == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr))
            && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == UNASSIGNED
                    || RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED)
                ==> ripas == RipasToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas))
            && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == UNASSIGNED_NS
                    || RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_NS)
                ==> ripas == RMI_EMPTY)))
    && new_s == old_s
}