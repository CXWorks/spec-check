pub open spec fn rmi_rtt_create_spec(
    result: Result<(), RmiStatusCode>,
    rd: Address,
    rtt: Address,
    ipa: Address,
    level: int,
    old_s: S,
    new_s: S,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_pre = RttWalk(old_s, realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY);
    let rtte_pre = walk_pre.rtte;
    
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RmmGranuleState::RD ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && ((!RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(old_s, realm, level)) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, level - 1) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (((ipa as int) >= (1 << realm.ipa_width)) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rtt) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!PaIsDelegableDram(old_s, rtt) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rtt).state != RmmGranuleState::DELEGATED ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && ((realm.feat_lpa2 == RmmFeatureBool::FEATURE_FALSE && ((rtt as int) >= (1 << 48))) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (walk.level < level - 1 ==> result.is_Err() && result.get_Err_0() == RmiStatusCode::RMI_ERROR_RTT)
    && (walk.rtte.state == RmmRttEntryState::TABLE ==> result.is_Err() && result.get_Err_0() == RmiStatusCode::RMI_ERROR_RTT)
    && (
        (AddrIsGranuleAligned(old_s, rd)
         && PaIsDelegable(old_s, rd)
         && GranuleAt(old_s, rd).state == RmmGranuleState::RD
         && RttLevelIsValid(old_s, realm, level)
         && !RttLevelIsStarting(old_s, realm, level)
         && AddrIsRttLevelAligned(old_s, ipa, level - 1)
         && ((ipa as int) < (1 << realm.ipa_width))
         && AddrIsGranuleAligned(old_s, rtt)
         && PaIsDelegableDram(old_s, rtt)
         && GranuleAt(old_s, rtt).state == RmmGranuleState::DELEGATED
         && (realm.feat_lpa2 == RmmFeatureBool::FEATURE_TRUE || ((rtt as int) < (1 << 48)))
         && walk.level >= level - 1
         && walk.rtte.state != RmmRttEntryState::TABLE)
        ==> (result.is_Ok()
             && GranuleAt(new_s, rtt).state == RmmGranuleState::RTT
             && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == RmmRttEntryState::TABLE
             && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).addr == rtt
             && (AddrIsProtected(old_s, ipa, realm) ==> RttAllEntriesRipas(new_s, RttAt(new_s, rtt), rtte_pre.ripas))
             && RttAllEntriesState(new_s, RttAt(new_s, rtt), rtte_pre.state)
             && (rtte_pre.state != RmmRttEntryState::UNASSIGNED && rtte_pre.state != RmmRttEntryState::UNASSIGNED_NS ==> RttAllEntriesContiguous(new_s, RttAt(new_s, rtt), rtte_pre.addr, level)))
    )
}