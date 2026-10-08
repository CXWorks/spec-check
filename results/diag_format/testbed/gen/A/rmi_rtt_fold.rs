pub open spec fn rmi_rtt_fold_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, (level as int) - 1, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level as int);
    let fold_pre = RttFold(old_s, RttAt(old_s, walk.rtte.addr));
    let new_rtte = RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx);
    let rd_ok = AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD;
    let level_ok = RttLevelIsValid(old_s, realm, level as int)
        && !RttLevelIsStarting(old_s, realm, level as int);
    let ipa_ok = AddrIsRttLevelAligned(old_s, ipa, (level as int) - 1)
        && (ipa as int) < pow2(realm.ipa_width as nat);
    let walk_ok = (walk.level as int) == (level as int) - 1
        && walk.rtte.state == TABLE;
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((rd_ok && (!RttLevelIsValid(old_s, realm, level as int) || RttLevelIsStarting(old_s, realm, level as int))) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((rd_ok && !AddrIsRttLevelAligned(old_s, ipa, (level as int) - 1)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((rd_ok && (ipa as int) >= pow2(realm.ipa_width as nat)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((rd_ok && level_ok && ipa_ok && (walk.level as int) < (level as int) - 1) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && ((rd_ok && level_ok && ipa_ok && walk.rtte.state != TABLE) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && ((rd_ok && level_ok && ipa_ok && walk_ok && !RttIsHomogeneous(old_s, RttAt(old_s, walk.rtte.addr))) ==> ResultEqual(result, RMI_ERROR_RTT(level as int)))
    && ((rd_ok && level_ok && ipa_ok && AddrIsAuxRef(old_s, ipa, realm)) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && ((rd_ok && level_ok && ipa_ok && walk_ok
            && RttIsHomogeneous(old_s, RttAt(old_s, walk.rtte.addr))
            && !AddrIsAuxRef(old_s, ipa, realm)) ==> (
        result.is_Ok()
        && new_rtte.state == fold_pre.state
        && ((fold_pre.state != UNASSIGNED && fold_pre.state != UNASSIGNED_NS) ==> new_rtte.addr == fold_pre.addr)
        && (fold_pre.state == ASSIGNED ==> (RttMemAttrEqual(new_rtte, fold_pre, RTT_PROTECTED) && RttS2APEqual(new_rtte, fold_pre, S2AP_INDIRECT)))
        && (fold_pre.state == ASSIGNED_NS ==> (RttMemAttrEqual(new_rtte, fold_pre, RTT_UNPROTECTED) && RttS2APEqual(new_rtte, fold_pre, realm.rtt_s2ap_encoding)))
        && (AddrIsProtected(old_s, ipa, realm) ==> new_rtte.ripas == fold_pre.ripas)
        && GranuleAt(new_s, walk.rtte.addr).state == DELEGATED
        && rtt == walk.rtte.addr
    ))
}
