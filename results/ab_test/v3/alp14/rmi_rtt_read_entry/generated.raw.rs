pub open spec fn rmi_rtt_read_entry_spec(result: RmiCommandReturnCode, rd: Address, ipa: Address, level: int, old_s: S, new_s: S, walk_level: int, state: RmiRttEntryState, rtte: RmmRttEntry, ripas: RmiRipas) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttLevelIsValid(old_s, realm, level) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(ipa, level) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((ipa as int) >= (1 << realm.ipa_width)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD && RttLevelIsValid(old_s, realm, level) && AddrIsRttLevelAligned(ipa, level) && ((ipa as int) < (1 << realm.ipa_width)))
        ==> (result.is_Ok()
            && walk_level == walk.level
            && state == RttEntryStateToRmi(old_s, walk.rtte.state)
            && ((walk.rtte.state == UNASSIGNED || walk.rtte.state == UNASSIGNED_NS)
                ==> (rtte.attr_unprot == Zeros(3)
                    && rtte.s2ap_indirect.base_index == S2AP_NO_ACCESS
                    && rtte.s2ap_indirect.overlay_index == 0
                    && rtte.s2ap_direct.read == RMM_FALSE
                    && rtte.s2ap_direct.write == RMM_FALSE
                    && rtte.addr == Zeros(ADDRESS_WIDTH)))
            && ((walk.rtte.state == ASSIGNED || walk.rtte.state == ASSIGNED_DEV || walk.rtte.state == ASSIGNED_VSMMU || walk.rtte.state == TABLE)
                ==> (rtte.attr_unprot == Zeros(3)
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