pub open spec fn rmi_rtt_map_unprotected_spec(
    result: RmiCommandReturnCode,
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    level: int,
    desc: u64,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let rtte = RttDescriptorDecode(old_s, desc, realm.rtt_s2ap_encoding);

    // Failure conditions with ordering constraints
    (!RttDescriptorIsValidForUnprotected(old_s, desc) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        old_s,
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, rd).state != RD
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!RttLevelIsValid(old_s, realm, level)
        || level < 1) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsRttLevelAligned(
        old_s,
        rtte.addr,
        level,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((realm.feat_lpa2 == FEATURE_FALSE && UInt(
        rtte.addr,
    ) >= pow(2, 48)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsRttLevelAligned(
        old_s,
        ipa,
        level,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((UInt(ipa) >= pow(2, realm.ipa_width)
        || AddrIsProtected(old_s, ipa, realm)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    realm.rtt_s2ap_encoding == S2AP_INDIRECT && rtte.s2ap_indirect.base_index != S2AP_NO_ACCESS
        && rtte.s2ap_indirect.base_index != S2AP_RO && rtte.s2ap_indirect.base_index != S2AP_WO
        && rtte.s2ap_indirect.base_index != S2AP_RW) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    walk.level < level ==> ResultEqual(result, RMI_ERROR_RTT(walk.level))) && (walk.rtte.state
        != UNASSIGNED_NS ==> ResultEqual(
        result,
        RMI_ERROR_RTT(walk.level),
    ))
    // Success conditions
     && ((result.is_Ok() && walk.rtte.state == ASSIGNED_NS && walk.rtte.attr_unprot
        == rtte.attr_unprot && walk.rtte.addr == rtte.addr && (realm.rtt_s2ap_encoding
        == S2AP_DIRECT ==> (walk.rtte.s2ap_direct.read == rtte.s2ap_direct.read
        && walk.rtte.s2ap_direct.write == rtte.s2ap_direct.write)) && (realm.rtt_s2ap_encoding
        == S2AP_INDIRECT ==> (walk.rtte.s2ap_indirect.base_index == rtte.s2ap_indirect.base_index
        && walk.rtte.s2ap_indirect.overlay_index == 15))) || result.is_Err())
}