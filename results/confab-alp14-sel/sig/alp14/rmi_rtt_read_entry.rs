pub open spec fn rmi_rtt_read_entry_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, walk_level: UInt64, state: RmiRttEntryState, desc: Bits64, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!(ipa < (1u64 << RealmAt(old_s, rd).ipa_width)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && GranuleAt(old_s, rd).state == RD
            && RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int)
            && AddrIsRttLevelAligned(old_s, ipa, level as int)
            && ipa < (1u64 << RealmAt(old_s, rd).ipa_width))
        ==> (result.is_Ok()
            && walk_level as int == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).level
            && state == RttEntryStateToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.state)
            && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.state == UNASSIGNED
                    || RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.state == UNASSIGNED_NS)
                ==> (RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).attr_unprot == 0
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == S2AP_NO_ACCESS
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.overlay_index == 0
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_direct.read == RMM_FALSE
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_direct.write == RMM_FALSE
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).addr == 0))
            && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.state == ASSIGNED
                    || RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.state == ASSIGNED_DEV
                    || RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.state == ASSIGNED_VSMMU
                    || RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.state == TABLE)
                ==> (RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).attr_unprot == 0
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == S2AP_NO_ACCESS
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.overlay_index == 0
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_direct.read == RMM_FALSE
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_direct.write == RMM_FALSE
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).addr == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.addr))
            && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.state == ASSIGNED_NS
                ==> (RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).attr_unprot == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.attr_unprot
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.base_index == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.s2ap_indirect.base_index
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_indirect.overlay_index == 0
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_direct.read == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.s2ap_direct.read
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).s2ap_direct.write == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.s2ap_direct.write
                    && RttDescriptorDecode(old_s, desc, RealmAt(old_s, rd).rtt_s2ap_encoding).addr == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.addr))
            && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.state == UNASSIGNED
                    || RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.state == ASSIGNED)
                ==> ripas == RipasToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.ripas))
            && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.state == UNASSIGNED_NS
                    || RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.state == ASSIGNED_NS)
                ==> ripas == RMI_EMPTY)))
    && new_s == old_s
}