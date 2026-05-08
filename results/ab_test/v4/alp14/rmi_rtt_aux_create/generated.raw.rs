pub open spec fn RMI_RTT_AUX_CREATE_spec(old_s: S, new_s: S, rd: Address, rtt: Address, ipa: Address, level: int, index: u64, result: RmiCommandReturnCode) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, index as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let unfold = RttWalk(old_s, realm, ipa, level - 1, index as int).rtte;
    
    ((!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((!RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(old_s, realm, level)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!AddrIsRttLevelAligned(ipa, level - 1) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!AddrIsProtected(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((realm.rtt_tree_per_plane == FEATURE_FALSE || index as int == RMM_RTT_TREE_PRIMARY || index as int > realm.num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!AddrIsGranuleAligned(rtt) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegableDram(rtt) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(old_s, rtt).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((realm.feat_lpa2 == FEATURE_FALSE && (rtt as int) >= (1 << 48)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (walk.level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT_AUX)) &&
    (walk.rtte.state == TABLE ==> ResultEqual(result, RMI_ERROR_RTT_AUX)) &&
    (result.is_Ok() ==> (
        GranuleAt(new_s, rtt).state == RTT &&
        walk.rtte.state == TABLE &&
        walk.rtte.addr == rtt &&
        RttAllEntriesRipas(old_s, RttAt(new_s, rtt), unfold.ripas) &&
        RttAllEntriesState(old_s, RttAt(new_s, rtt), unfold.state) &&
        (unfold.state != UNASSIGNED ==> RttAllEntriesContiguous(old_s, RttAt(new_s, rtt), unfold.addr, level))
    )))
}