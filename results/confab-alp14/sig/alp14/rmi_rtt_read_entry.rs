pub open spec fn rmi_rtt_read_entry_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, walk_level: UInt64, state: RmiRttEntryState, desc: Bits64, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level as int, RMM_RTT_TREE_PRIMARY as int);
    let rtte = RttDescriptorDecode(old_s, desc, realm.rtt_s2ap_encoding);
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttLevelIsValid(old_s, realm, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ipa as int) >= (1int << (realm.ipa_width as u64)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && GranuleAt(old_s, rd).state == RD
            && RttLevelIsValid(old_s, realm, level as int)
            && AddrIsRttLevelAligned(old_s, ipa, level as int)
            && (ipa as int) < (1int << (realm.ipa_width as u64)))
        ==> (result.is_Ok()
            && walk_level as int == walk.level as int
            && state == RttEntryStateToRmi(old_s, walk.rtte.state)
            && ((walk.rtte.state == UNASSIGNED || walk.rtte.state == UNASSIGNED_NS)
                ==> (rtte.attr_unprot == 0
                    && rtte.s2ap_indirect.base_index == S2AP_NO_ACCESS
                    && rtte.s2ap_indirect.overlay_index == 0
                    && rtte.s2ap_direct.read == RMM_FALSE
                    && rtte.s2ap_direct.write == RMM_FALSE
                    && rtte.addr == 0))
            && ((walk.rtte.state == ASSIGNED
                    || walk.rtte.state == ASSIGNED_DEV
                    || walk.rtte.state == ASSIGNED_VSMMU
                    || walk.rtte.state == TABLE)
                ==> (rtte.attr_unprot == 0
                    && rtte.s2ap_indirect.base_index == S2AP_NO_ACCESS
                    && rtte.s2ap_indirect.overlay_index == 0
                    && rtte.s2ap_direct.read == RMM_FALSE
                    && rtte.s2ap_direct.write == RMM_FALSE
                    && rtte.addr == walk.rtte.addr))
            && (walk.rtte.state == ASSIGNED_NS
                ==> (rtte.attr_unprot == walk.rtte.attr_unprot
                    && rtte.s2ap_indirect.base_index == walk.rtte.s2ap_indirect.base_index
                    && rtte.s2ap_indirect.overlay_index == 0
                    && rtte.s2ap_direct.read == walk.rtte.s2ap_direct.read
                    && rtte.s2ap_direct.write == walk.rtte.s2ap_direct.write
                    && rtte.addr == walk.rtte.addr))
            && ((walk.rtte.state == UNASSIGNED || walk.rtte.state == ASSIGNED)
                ==> ripas == RipasToRmi(old_s, walk.rtte.ripas))
            && ((walk.rtte.state == UNASSIGNED_NS || walk.rtte.state == ASSIGNED_NS)
                ==> ripas == RMI_EMPTY)
            && new_s == old_s))
}