pub open spec fn rmi_rtt_read_entry_spec(
    result: RmiCommandReturnCode,
    walk_level: u64,
    state: u8,
    desc: u64,
    ripas: u8,
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    level: i64,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let rtte = RttDescriptorDecode(old_s, desc, realm.rtt_s2ap_encoding);

    // Failure conditions
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(rd)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, rd).state != RD
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!RttLevelIsValid(old_s, realm, level)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsRttLevelAligned(ipa, level)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (UInt(ipa) >= (1 << realm.ipa_width)
        ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // Success condition: all preconditions pass
     && ((AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD
        && RttLevelIsValid(old_s, realm, level) && AddrIsRttLevelAligned(ipa, level) && UInt(ipa)
        < (1 << realm.ipa_width)) ==> (result == RMI_SUCCESS && walk_level == walk.level && state
        == RttEntryStateToRmi(old_s, walk.rtte.state) && (
    // state_invalid: UNASSIGNED or UNASSIGNED_NS
    ((walk.rtte.state == UNASSIGNED || walk.rtte.state == UNASSIGNED_NS) ==> (rtte.attr_unprot
        == Zeros(3) && rtte.s2ap_indirect.base_index == S2AP_NO_ACCESS
        && rtte.s2ap_indirect.overlay_index == 0 && rtte.s2ap_direct.read == RMM_FALSE
        && rtte.s2ap_direct.write == RMM_FALSE && rtte.addr == Zeros(
        ADDRESS_WIDTH,
    )))
    // state_prot: ASSIGNED, ASSIGNED_DEV, ASSIGNED_VSMMU, or TABLE
     && ((walk.rtte.state == ASSIGNED || walk.rtte.state == ASSIGNED_DEV || walk.rtte.state
        == ASSIGNED_VSMMU || walk.rtte.state == TABLE) ==> (rtte.attr_unprot == Zeros(3)
        && rtte.s2ap_indirect.base_index == S2AP_NO_ACCESS && rtte.s2ap_indirect.overlay_index == 0
        && rtte.s2ap_direct.read == RMM_FALSE && rtte.s2ap_direct.write == RMM_FALSE && rtte.addr
        == walk.rtte.addr))
    // state_unprot: ASSIGNED_NS
     && (walk.rtte.state == ASSIGNED_NS ==> (rtte.attr_unprot == walk.rtte.attr_unprot
        && rtte.s2ap_indirect.base_index == walk.rtte.s2ap_indirect.base_index
        && rtte.s2ap_indirect.overlay_index == 0 && rtte.s2ap_direct.read
        == walk.rtte.s2ap_direct.read && rtte.s2ap_direct.write == walk.rtte.s2ap_direct.write
        && rtte.addr
        == walk.rtte.addr))
    // state_io: ASSIGNED_DEV (already covered by state_prot)
    // state_vsmmu: ASSIGNED_VSMMU (already covered by state_prot)
    // ripas_prot: UNASSIGNED or ASSIGNED
     && ((walk.rtte.state == UNASSIGNED || walk.rtte.state == ASSIGNED) ==> ripas == RipasToRmi(
        old_s,
        walk.rtte.ripas,
    ))
    // ripas_unprot: UNASSIGNED_NS or ASSIGNED_NS
     && ((walk.rtte.state == UNASSIGNED_NS || walk.rtte.state == ASSIGNED_NS) ==> ripas
        == RMI_EMPTY)) && old_s == new_s))
}