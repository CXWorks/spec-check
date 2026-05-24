pub open spec fn rmi_rtt_aux_create_spec(
    result: Result<(), RmiStatusCode>,
    rd: Address,
    rtt: Address,
    ipa: Address,
    level: int,
    index: u64,
    old_s: S,
    new_s: S,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, index);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let unfold = RttWalk(old_s, realm, ipa, level - 1, index).rtte;
    
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(old_s, realm, level)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(ipa, level - 1) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegableDram(rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rtt).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((realm.feat_lpa2 == FEATURE_FALSE && (rtt as int) >= (1 << 48)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT_AUX))
    && (walk.rtte.state == TABLE ==> ResultEqual(result, RMI_ERROR_RTT_AUX))
    && (
        (AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD
         && (RttLevelIsValid(old_s, realm, level) && !RttLevelIsStarting(old_s, realm, level))
         && AddrIsRttLevelAligned(ipa, level - 1) && AddrIsProtected(ipa, realm)
         && (realm.rtt_tree_per_plane != FEATURE_FALSE && index != RMM_RTT_TREE_PRIMARY && index <= realm.num_aux_planes)
         && AddrIsGranuleAligned(rtt) && PaIsDelegableDram(rtt) && GranuleAt(old_s, rtt).state == DELEGATED
         && (realm.feat_lpa2 != FEATURE_FALSE || (rtt as int) < (1 << 48))
         && walk.level >= level - 1 && walk.rtte.state != TABLE)
        ==> (result.is_Ok()
             && GranuleAt(new_s, rtt).state == RTT
             && RttAllEntriesRipas(old_s, RttAt(new_s, rtt), unfold.ripas)
             && RttAllEntriesState(old_s, RttAt(new_s, rtt), unfold.state)
             && (unfold.state != UNASSIGNED ==> RttAllEntriesContiguous(old_s, RttAt(new_s, rtt), unfold.addr, level)))
    )
}