```verus
pub open spec fn rmi_rtt_unmap_unprotected_spec(
    result: RmiCommandReturnCode,
    top: Address,
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    level: int
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
    let rtte = RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx);
    
    // Failure: rd_align
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!RttLevelIsValid(old_s, realm, level) || level < 1) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(ipa, level) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((UInt(ipa) >= (ToBits64(1) << realm.ipa_width) || AddrIsProtected(ipa, realm)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.level < level ==> (ResultEqual(result, RMI_ERROR_RTT(walk.level)) && (top == walk_top)))
    && (rtte.state != ASSIGNED_NS ==> (ResultEqual(result, RMI_ERROR_RTT(walk.level)) && (top == walk_top)))
    
    // Success conditions
    && (
        AddrIsGranuleAligned(rd)
        && PaIsDelegable(rd)
        && GranuleAt(old_s, rd).state == RD
        && RttLevelIsValid(old_s, realm, level)
        && level >= 1
        && AddrIsRttLevelAligned(ipa, level)
        && UInt(ipa) < (ToBits64(1) << realm.ipa_width)
        && !AddrIsProtected(ipa, realm)
        && walk.level >= level
        && rtte.state == ASSIGNED_NS
        ==> (
            result == RMI_SUCCESS
            && top == walk_top
            && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == UNASSIGNED_NS
        )
    )
}
```