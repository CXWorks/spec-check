pub open spec fn rmi_rtt_map_unprotected_spec(
    result: RmiCommandReturnCode,
    rd: Address,
    ipa: Address,
    level: int,
    desc: u64,
    old_s: S,
    new_s: S,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, level);
    let rtte = RttDescriptorDecode(old_s, desc, realm.rtt_s2ap_encoding);

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
    ) >= 281474976710656) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsRttLevelAligned(
        old_s,
        ipa,
        level,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((UInt(ipa) >= (1 << realm.ipa_width)
        || AddrIsProtected(old_s, ipa, realm)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    realm.rtt_s2ap_encoding == S2AP_INDIRECT && rtte.s2ap_indirect.base_index != S2AP_NO_ACCESS
        && rtte.s2ap_indirect.base_index != S2AP_RO && rtte.s2ap_indirect.base_index != S2AP_WO
        && rtte.s2ap_indirect.base_index != S2AP_RW) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    walk.level < level ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT) && (
    walk.rtte.state != UNASSIGNED_NS ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT)
        && ((RttDescriptorIsValidForUnprotected(old_s, desc) && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state == RD && RttLevelIsValid(
        old_s,
        realm,
        level,
    ) && level >= 1 && AddrIsRttLevelAligned(old_s, rtte.addr, level) && (realm.feat_lpa2
        != FEATURE_FALSE || UInt(rtte.addr) < 281474976710656) && AddrIsRttLevelAligned(
        old_s,
        ipa,
        level,
    ) && UInt(ipa) < (1 << realm.ipa_width) && !AddrIsProtected(old_s, ipa, realm) && (
    realm.rtt_s2ap_encoding != S2AP_INDIRECT || rtte.s2ap_indirect.base_index == S2AP_NO_ACCESS
        || rtte.s2ap_indirect.base_index == S2AP_RO || rtte.s2ap_indirect.base_index == S2AP_WO
        || rtte.s2ap_indirect.base_index == S2AP_RW) && walk.level >= level && walk.rtte.state
        == UNASSIGNED_NS) ==> (result.is_Ok() && RttEntryAt(
        old_s,
        RttAt(old_s, walk.rtt_addr),
        entry_idx,
    ).state != ASSIGNED_NS ==> RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state
        == ASSIGNED_NS && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).attr_unprot
        == rtte.attr_unprot && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).addr
        == rtte.addr && (realm.rtt_s2ap_encoding == S2AP_DIRECT ==> (RttEntryAt(
        new_s,
        RttAt(new_s, walk.rtt_addr),
        entry_idx,
    ).s2ap_direct.read == rtte.s2ap_direct.read && RttEntryAt(
        new_s,
        RttAt(new_s, walk.rtt_addr),
        entry_idx,
    ).s2ap_direct.write == rtte.s2ap_direct.write)) && (realm.rtt_s2ap_encoding == S2AP_INDIRECT
        ==> (RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).s2ap_indirect.base_index
        == rtte.s2ap_indirect.base_index && RttEntryAt(
        new_s,
        RttAt(new_s, walk.rtt_addr),
        entry_idx,
    ).s2ap_indirect.overlay_index == 15))))
}