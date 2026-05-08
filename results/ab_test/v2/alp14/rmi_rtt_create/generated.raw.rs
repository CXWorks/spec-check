```verus
pub open spec fn rmi_rtt_create_spec(result: RmiCommandReturnCode, old_s: S, new_s: S, rd: Address, rtt: Address, ipa: Address, level: int) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_pre = RttWalk(old_s, realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY);
    let rtte_pre = walk_pre.rtte;
    
    // Failure conditions
    ((!AddrIsGranuleAligned(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!PaIsDelegable(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((GranuleAt(old_s, rd).state != RD) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((!RttLevelIsValid(old_s, realm, level)) || RttLevelIsStarting(old_s, realm, level)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!AddrIsRttLevelAligned(old_s, ipa, level - 1)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((ipa as int) >= (1 << realm.ipa_width)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!AddrIsGranuleAligned(old_s, rtt)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!PaIsDelegableDram(old_s, rtt)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((GranuleAt(old_s, rtt).state != DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((realm.feat_lpa2 == FEATURE_FALSE) && ((rtt as int) >= (1 << 48))) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((walk.level < level - 1) ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((walk.rtte.state == TABLE) ==> ResultEqual(result, RMI_ERROR_RTT))
    
    // Success conditions
    && ((AddrIsGranuleAligned(old_s, rd) 
        && PaIsDelegable(old_s, rd) 
        && GranuleAt(old_s, rd).state == RD
        && RttLevelIsValid(old_s, realm, level) 
        && !RttLevelIsStarting(old_s, realm, level)
        && AddrIsRttLevelAligned(old_s, ipa, level - 1) 
        && (ipa as int) < (1 << realm.ipa_width)
        && AddrIsGranuleAligned(old_s, rtt) 
        && PaIsDelegableDram(old_s, rtt) 
        && GranuleAt(old_s, rtt).state == DELEGATED
        && !((realm.feat_lpa2 == FEATURE_FALSE) && ((rtt as int) >= (1 << 48)))
        && walk.level >= level - 1
        && walk.rtte.state != TABLE)
        ==> (
            GranuleAt(new_s, rtt).state == RTT
            && walk.rtte.state == TABLE
            && walk.rtte.addr == rtt
            && (AddrIsProtected(old_s, ipa, realm) ==> RttAllEntriesRipas(old_s, RttAt(new_s, rtt), rtte_pre.ripas))
            && RttAllEntriesState(old_s, RttAt(new_s, rtt), rtte_pre.state)
            && ((rtte_pre.state != UNASSIGNED && rtte_pre.state != UNASSIGNED_NS) ==> RttAllEntriesContiguous(old_s, RttAt(new_s, rtt), rtte_pre.addr, level))
        ))
}
```