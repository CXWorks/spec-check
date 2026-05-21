```verus
pub open spec fn rmi_rtt_aux_destroy_spec(
    result: RmiCommandReturnCode,
    rtt: Address,
    top: Address,
    rd: Address,
    ipa: Address,
    level: int,
    index: u64,
    old_s: S,
    new_s: S
) -> bool {
    let realm = RealmAt(rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, index as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);

    (
        (!AddrIsGranuleAligned(rd) ==> result == RMI_ERROR_INPUT)
        && (!PaIsDelegable(rd) ==> result == RMI_ERROR_INPUT)
        && (GranuleAt(old_s, rd).state != RD ==> result == RMI_ERROR_INPUT)
        && ((!RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(old_s, realm, level)) ==> result == RMI_ERROR_INPUT)
        && (!AddrIsRttLevelAligned(ipa, level - 1) ==> result == RMI_ERROR_INPUT)
        && (!AddrIsProtected(ipa, realm) ==> result == RMI_ERROR_INPUT)
        && ((realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes) ==> result == RMI_ERROR_INPUT)
        && (walk.level < level - 1 ==> (result == RMI_ERROR_RTT_AUX && top == walk_top))
        && (walk.rtte.state != TABLE ==> (result == RMI_ERROR_RTT_AUX && top == walk_top))
        && (RttIsLive(old_s, RttAt(old_s, walk.rtte.addr)) ==> (result == RMI_ERROR_RTT_AUX && top == ipa))
        && (
            (!AddrIsGranuleAligned(rd) || !PaIsDelegable(rd) || GranuleAt(old_s, rd).state != RD
                || !RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(old_s, realm, level)
                || !AddrIsRttLevelAligned(ipa, level - 1) || !AddrIsProtected(ipa, realm)
                || realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes
                || walk.level < level - 1 || walk.rtte.state != TABLE || RttIsLive(old_s, RttAt(old_s, walk.rtte.addr)))
            || (walk.rtte.state == AUX_DESTROYED && walk.rtte.ripas == DESTROYED
                && GranuleAt(new_s, walk.rtte.addr).state == DELEGATED
                && rtt == walk.rtte.addr && top == walk_top && result == RMI_SUCCESS)
        )
    )
}
```