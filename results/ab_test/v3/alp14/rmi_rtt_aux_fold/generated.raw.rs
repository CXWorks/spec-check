```verus
pub open spec fn rmi_rtt_aux_fold_spec(
    result: RmiCommandReturnCode,
    rtt: Address,
    rd: Address,
    ipa: Address,
    level: int,
    index: u64,
    old_s: S,
    new_s: S
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, index);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let fold_pre = RttFold(old_s, RttAt(old_s, walk.rtte.addr));

    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(old_s, realm, level)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(ipa, level - 1) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.level < level - 1 ==> result == RMI_ERROR_RTT_AUX)
    && (walk.rtte.state != TABLE ==> result == RMI_ERROR_RTT_AUX)
    && (!RttIsHomogeneous(old_s, RttAt(old_s, walk.rtte.addr)) ==> result == RMI_ERROR_RTT_AUX)
    && (
        (AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD
         && RttLevelIsValid(old_s, realm, level) && !RttLevelIsStarting(old_s, realm, level)
         && AddrIsRttLevelAligned(ipa, level - 1) && AddrIsProtected(ipa, realm)
         && !(realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes)
         && walk.level >= level - 1 && walk.rtte.state == TABLE && RttIsHomogeneous(old_s, RttAt(old_s, walk.rtte.addr)))
        ==> (
            result == RMI_SUCCESS
            && walk.rtte.state == fold_pre.state
            && (fold_pre.state != UNASSIGNED && fold_pre.state != UNASSIGNED_NS ==> walk.rtte.addr == fold_pre.addr)
            && (fold_pre.state == ASSIGNED ==> (RttMemAttrEqual(walk.rtte, fold_pre, RTT_PROTECTED) && RttS2APEqual(walk.rtte, fold_pre, S2AP_INDIRECT)))
            && (fold_pre.state == ASSIGNED_NS ==> (RttMemAttrEqual(walk.rtte, fold_pre, RTT_UNPROTECTED) && RttS2APEqual(walk.rtte, fold_pre, realm.rtt_s2ap_encoding)))
            && (AddrIsProtected(ipa, realm) ==> walk.rtte.ripas == fold_pre.ripas)
            && GranuleAt(new_s, walk.rtte.addr).state == DELEGATED
            && rtt == walk.rtte.addr
        )
    )
}
```