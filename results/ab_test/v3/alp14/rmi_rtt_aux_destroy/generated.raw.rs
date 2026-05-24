pub open spec fn rmi_rtt_aux_destroy_spec(
    result: Result<(), RmiStatusCode>,
    rtt: Address,
    top: Address,
    rd: Address,
    ipa: Address,
    level: int,
    index: u64,
    old_s: S,
    new_s: S,
) -> bool {
    let realm = RealmAt(rd);
    let walk = RttWalk(realm, ipa, level - 1, index);
    let entry_idx = RttEntryIndex(ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(RttAt(walk.rtt_addr), walk.level, ipa);
    
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && ((!RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(old_s, realm, level)) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(ipa, level - 1) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!AddrIsProtected(ipa, realm) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && ((realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (walk.level < level - 1 ==> (ResultEqual(result, RmiStatusCode::RMI_ERROR_RTT_AUX) && top == walk_top))
    && (walk.rtte.state != TABLE ==> (ResultEqual(result, RmiStatusCode::RMI_ERROR_RTT_AUX) && top == walk_top))
    && (RttIsLive(old_s, RttAt(old_s, walk.rtte.addr)) ==> (ResultEqual(result, RmiStatusCode::RMI_ERROR_RTT_AUX) && top == ipa))
    && (
        (AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD
         && RttLevelIsValid(old_s, realm, level) && !RttLevelIsStarting(old_s, realm, level)
         && AddrIsRttLevelAligned(ipa, level - 1) && AddrIsProtected(ipa, realm)
         && (realm.rtt_tree_per_plane != FEATURE_FALSE && index != RMM_RTT_TREE_PRIMARY && index <= realm.num_aux_planes)
         && walk.level >= level - 1 && walk.rtte.state == TABLE && !RttIsLive(old_s, RttAt(old_s, walk.rtte.addr)))
        ==> (result.is_Ok()
             && walk.rtte.state == AUX_DESTROYED
             && walk.rtte.ripas == DESTROYED
             && GranuleAt(new_s, walk.rtte.addr).state == DELEGATED
             && rtt == walk.rtte.addr
             && top == walk_top)
    )
}