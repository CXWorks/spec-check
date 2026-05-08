```verus
pub open spec fn RMI_RTT_FOLD_spec(old_s: S, new_s: S, rd: Address, ipa: Address, level: int, result: RmiCommandReturnCode, rtt: Address) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let fold_pre = RttFold(old_s, RttAt(old_s, walk.rtte.addr));
    
    // Failure conditions
    ((!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((!RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(old_s, realm, level)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!AddrIsRttLevelAligned(ipa, level - 1) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (UInt(ipa) >= (1 << realm.ipa_width) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (walk.level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT)) &&
    (walk.rtte.state != TABLE ==> ResultEqual(result, RMI_ERROR_RTT)) &&
    (!RttIsHomogeneous(old_s, RttAt(old_s, walk.rtte.addr)) ==> ResultEqual(result, RMI_ERROR_RTT)) &&
    (AddrIsAuxRef(ipa, realm) ==> ResultEqual(result, RMI_ERROR_RTT))) &&
    
    // Success condition: all failure conditions false implies success
    ((AddrIsGranuleAligned(rd) &&
    PaIsDelegable(rd) &&
    GranuleAt(old_s, rd).state == RD &&
    RttLevelIsValid(old_s, realm, level) &&
    !RttLevelIsStarting(old_s, realm, level) &&
    AddrIsRttLevelAligned(ipa, level - 1) &&
    UInt(ipa) < (1 << realm.ipa_width) &&
    walk.level >= level - 1 &&
    walk.rtte.state == TABLE &&
    RttIsHomogeneous(old_s, RttAt(old_s, walk.rtte.addr)) &&
    !AddrIsAuxRef(ipa, realm)) ==>
    (result.is_Ok() &&
    walk.rtte.state == fold_pre.state &&
    ((fold_pre.state != UNASSIGNED && fold_pre.state != UNASSIGNED_NS) ==> walk.rtte.addr == fold_pre.addr) &&
    ((fold_pre.state == ASSIGNED) ==> (RttMemAttrEqual(walk.rtte, fold_pre, RTT_PROTECTED) && RttS2APEqual(walk.rtte, fold_pre, S2AP_INDIRECT))) &&
    ((fold_pre.state == ASSIGNED_NS) ==> (RttMemAttrEqual(walk.rtte, fold_pre, RTT_UNPROTECTED) && RttS2APEqual(walk.rtte, fold_pre, realm.rtt_s2ap_encoding))) &&
    ((AddrIsProtected(ipa, realm)) ==> walk.rtte.ripas == fold_pre.ripas) &&
    GranuleAt(new_s, walk.rtte.addr).state == DELEGATED &&
    rtt == walk.rtte.addr))
}
```