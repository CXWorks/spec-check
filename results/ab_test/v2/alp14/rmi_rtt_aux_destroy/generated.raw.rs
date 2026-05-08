```verus
pub open spec fn rmi_rtt_aux_destroy_spec(
    result: RmiCommandReturnCode,
    rtt: Address,
    top: Address,
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    level: int,
    index: u64
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, index);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
    
    // Failure conditions
    ((!AddrIsGranuleAligned(old_s, rd)) ==> result == RMI_ERROR_INPUT)
    && ((!PaIsDelegable(old_s, rd)) ==> result == RMI_ERROR_INPUT)
    && ((GranuleAt(old_s, rd).state != RD) ==> result == RMI_ERROR_INPUT)
    && (((!RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(old_s, realm, level))) ==> result == RMI_ERROR_INPUT)
    && ((!AddrIsRttLevelAligned(old_s, ipa, level - 1)) ==> result == RMI_ERROR_INPUT)
    && ((!AddrIsProtected(old_s, ipa, realm)) ==> result == RMI_ERROR_INPUT)
    && (((realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes)) ==> result == RMI_ERROR_INPUT)
    && ((walk.level < level - 1) ==> (result == RMI_ERROR_RTT_AUX && top == walk_top))
    && ((RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx as int).state != TABLE) ==> (result == RMI_ERROR_RTT_AUX && top == walk_top))
    && ((RttIsLive(old_s, RttAt(old_s, RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx as int).addr))) ==> (result == RMI_ERROR_RTT_AUX && top == ipa))
    
    // Success conditions
    && ((AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && RttLevelIsValid(old_s, realm, level)
        && !RttLevelIsStarting(old_s, realm, level)
        && AddrIsRttLevelAligned(old_s, ipa, level - 1)
        && AddrIsProtected(old_s, ipa, realm)
        && !(realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes)
        && walk.level >= level - 1
        && RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx as int).state == TABLE
        && !RttIsLive(old_s, RttAt(old_s, RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx as int).addr)))
    ==> (result == RMI_SUCCESS
        && RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx as int).state == AUX_DESTROYED
        && RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx as int).ripas == DESTROYED
        && GranuleAt(new_s, RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx as int).addr).state == DELEGATED
        && rtt == RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx as int).addr
        && top == walk_top))
}
```