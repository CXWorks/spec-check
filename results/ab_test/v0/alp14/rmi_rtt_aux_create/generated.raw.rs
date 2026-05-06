```verus
pub open spec fn RMI_RTT_AUX_CREATE_spec(
    s: S,
    rd: Address,
    rtt: Address,
    ipa: Address,
    level: int,
    index: u64,
    result: RmiCommandReturnCode,
) -> bool {
    let realm = RmmRealm::RealmAt(s, rd);
    let walk = RttWalk(s, realm, ipa, level - 1, index as int);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let unfold = RttWalk(s, realm, ipa, level - 1, index as int).rtte;
    
    if !AddrIsGranuleAligned(rd) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !PaIsDelegable(rd) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if GranuleAt(s, rd).state != RmmGranuleState::RD {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !RttLevelIsValid(s, realm, level) || RttLevelIsStarting(s, realm, level) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !AddrIsRttLevelAligned(ipa, level - 1) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !AddrIsProtected(ipa, realm) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if realm.rtt_tree_per_plane == RmmFeature::FEATURE_FALSE || index == 0u64 || index > realm.num_aux_planes as u64 {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !AddrIsGranuleAligned(rtt) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if !PaIsDelegableDram(rtt) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if GranuleAt(s, rtt).state != RmmGranuleState::DELEGATED {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if realm.feat_lpa2 == RmmFeature::FEATURE_FALSE && rtt as u64 >= (1u64 << 48) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if walk.level < level - 1 {
        result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT_AUX && result.get_Err_1() == walk.level
    } else if walk.rtte.state == RmmRttEntryState::TABLE {
        result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT_AUX && result.get_Err_1() == walk.level
    } else {
        result.is_Ok()
        && GranuleAt(s, rtt).state == RmmGranuleState::RTT
        && walk.rtte.state == RmmRttEntryState::TABLE
        && walk.rtte.addr == rtt
        && RttAllEntriesRipas(s, RttAt(s, rtt), unfold.ripas)
        && RttAllEntriesState(s, RttAt(s, rtt), unfold.state)
        && (unfold.state != RmmRttEntryState::UNASSIGNED ==> RttAllEntriesContiguous(s, RttAt(s, rtt), unfold.addr, level))
    }
}
```