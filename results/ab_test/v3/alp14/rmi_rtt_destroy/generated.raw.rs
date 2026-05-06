```verus
pub open spec fn RMI_RTT_DESTROY_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    level: int,
    result: RmiCommandReturnCode,
    rtt: Address,
    top: Address,
) -> bool
{
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_rtt = RttAt(old_s, walk.rtt_addr);
    let walk_top = RttSkipNonLiveEntries(old_s, walk_rtt, walk.level, ipa);
    let walk_rtte = RttEntryAt(old_s, walk_rtt, entry_idx);

    // Failure conditions (checked in order)
    (
        // rd_align
        (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // rd_bound
        (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // rd_state
        (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // level_bound
        ((!RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(old_s, realm, level)) 
            ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // ipa_align
        (!AddrIsRttLevelAligned(ipa, level - 1) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // ipa_bound
        (UInt(ipa) >= (1 << realm.ipa_width) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // rtt_walk (checked before rtte_state per ordering)
        (walk.level < level - 1 ==> 
            (ResultEqual(result, RMI_ERROR_RTT) && result.get_Err_0().1 == walk.level && top == walk_top))
        &&
        // rtte_state (ordered after rd checks, before rtt_live)
        (walk_rtte.state != TABLE ==> 
            (ResultEqual(result, RMI_ERROR_RTT) && result.get_Err_0().1 == walk.level && top == walk_top))
        &&
        // rtt_live (ordered after rtte_state)
        (RttIsLive(old_s, RttAt(old_s, walk_rtte.addr)) ==> 
            (ResultEqual(result, RMI_ERROR_RTT) && result.get_Err_0().1 == level && top == ipa))
        &&
        // aux_ref (ordered after rtte_state)
        (AddrIsAuxRef(ipa, realm) ==> 
            (ResultEqual(result, RMI_ERROR_RTT) && result.get_Err_0().1 == walk.level))
    )
    ||
    // Success case (all preconditions satisfied)
    (
        AddrIsGranuleAligned(rd)
        && PaIsDelegable(rd)
        && GranuleAt(old_s, rd).state == RD
        && RttLevelIsValid(old_s, realm, level)
        && !RttLevelIsStarting(old_s, realm, level)
        && AddrIsRttLevelAligned(ipa, level - 1)
        && UInt(ipa) < (1 << realm.ipa_width)
        && walk.level >= level - 1
        && walk_rtte.state == TABLE
        && !RttIsLive(old_s, RttAt(old_s, walk_rtte.addr))
        && !AddrIsAuxRef(ipa, realm)
        && result.is_Ok()
        && rtt == walk_rtte.addr
        && top == walk_top
        && (AddrIsProtected(ipa, realm) ==>
            (walk_rtte.state == UNASSIGNED && walk_rtte.ripas == DESTROYED))
        && (!AddrIsProtected(ipa, realm) ==>
            (walk_rtte.state == UNASSIGNED_NS))
        && GranuleAt(new_s, walk_rtte.addr).state == DELEGATED
    )
}
```