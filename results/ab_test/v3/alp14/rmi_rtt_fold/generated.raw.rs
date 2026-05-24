pub open spec fn rmi_rtt_fold_spec(
    result: RmiCommandReturnCode,
    rd: Address,
    ipa: Address,
    level: int,
    rtt: Address,
    old_s: S,
    new_s: S,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let fold_pre = RttFold(old_s, RttAt(old_s, walk.rtte.addr));
    
    (
        (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
        && ((!RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(old_s, realm, level)) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (!AddrIsRttLevelAligned(ipa, level - 1) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && ((ipa as int) >= (1 << realm.ipa_width) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (walk.level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT) && walk.level == (ResultGetErr1(result) as int))
        && (walk.rtte.state != TABLE ==> ResultEqual(result, RMI_ERROR_RTT) && walk.level == (ResultGetErr1(result) as int))
        && (!RttIsHomogeneous(old_s, RttAt(old_s, walk.rtte.addr)) ==> ResultEqual(result, RMI_ERROR_RTT) && level == (ResultGetErr1(result) as int))
        && (AddrIsAuxRef(ipa, realm) ==> ResultEqual(result, RMI_ERROR_RTT) && walk.level == (ResultGetErr1(result) as int))
        && (
            (AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD
             && RttLevelIsValid(old_s, realm, level) && !RttLevelIsStarting(old_s, realm, level)
             && AddrIsRttLevelAligned(ipa, level - 1) && ((ipa as int) < (1 << realm.ipa_width))
             && walk.level >= level - 1 && walk.rtte.state == TABLE
             && RttIsHomogeneous(old_s, RttAt(old_s, walk.rtte.addr)) && !AddrIsAuxRef(ipa, realm))
            ==> (
                result.is_Ok()
                && rtt == walk.rtte.addr
                && RttEntryAt(old_s, RttAt(old_s, walk.rtte.addr), entry_idx).state == fold_pre.state
                && (fold_pre.state == UNASSIGNED || fold_pre.state == UNASSIGNED_NS || RttEntryAt(old_s, RttAt(old_s, walk.rtte.addr), entry_idx).addr == fold_pre.addr)
                && (fold_pre.state != ASSIGNED || (RttMemAttrEqual(RttEntryAt(old_s, RttAt(old_s, walk.rtte.addr), entry_idx), fold_pre, RTT_PROTECTED) && RttS2APEqual(RttEntryAt(old_s, RttAt(old_s, walk.rtte.addr), entry_idx), fold_pre, S2AP_INDIRECT)))
                && (fold_pre.state != ASSIGNED_NS || (RttMemAttrEqual(RttEntryAt(old_s, RttAt(old_s, walk.rtte.addr), entry_idx), fold_pre, RTT_UNPROTECTED) && RttS2APEqual(RttEntryAt(old_s, RttAt(old_s, walk.rtte.addr), entry_idx), fold_pre, realm.rtt_s2ap_encoding)))
                && (!AddrIsProtected(ipa, realm) || RttEntryAt(old_s, RttAt(old_s, walk.rtte.addr), entry_idx).ripas == fold_pre.ripas)
                && GranuleAt(new_s, walk.rtte.addr).state == DELEGATED
            )
        )
    )
}