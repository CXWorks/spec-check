pub open spec fn rmi_rtt_create_spec(rd: Address, rtt: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk_pre = RttWalk(old_s, realm, ipa, (level as int) - 1, RMM_RTT_TREE_PRIMARY);
    let rtte_pre = walk_pre.rtte;
    let walk = RttWalk(new_s, RealmAt(new_s, rd), ipa, (level as int) - 1, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(new_s, ipa, walk.level);
    let rtte = RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx);
    let new_rtt = RttAt(new_s, rtt);
    let input_fail =
        !AddrIsGranuleAligned(old_s, rd)
        || !PaIsDelegable(old_s, rd)
        || GranuleAt(old_s, rd).state != RD
        || !RttLevelIsValid(old_s, realm, level as int)
        || RttLevelIsStarting(old_s, realm, level as int)
        || !AddrIsRttLevelAligned(old_s, ipa, (level as int) - 1)
        || (ipa as int) >= pow2(realm.ipa_width as nat)
        || !AddrIsGranuleAligned(old_s, rtt)
        || !PaIsDelegableDram(old_s, rtt)
        || GranuleAt(old_s, rtt).state != DELEGATED
        || (realm.feat_lpa2 == FEATURE_FALSE && (rtt as int) >= pow2(48));
    let rtt_fail =
        walk_pre.level < (level as int) - 1
        || rtte_pre.state == TABLE;
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!RttLevelIsValid(old_s, realm, level as int) || RttLevelIsStarting(old_s, realm, level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, (level as int) - 1) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ipa as int) >= pow2(realm.ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegableDram(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rtt).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((realm.feat_lpa2 == FEATURE_FALSE && (rtt as int) >= pow2(48)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!input_fail && walk_pre.level < (level as int) - 1) ==> ResultEqual(result, RMI_ERROR_RTT(walk_pre.level)))
    && ((!input_fail && rtte_pre.state == TABLE) ==> ResultEqual(result, RMI_ERROR_RTT(walk_pre.level)))
    && (result.is_Err() ==> GranuleAt(new_s, rtt).state == GranuleAt(old_s, rtt).state)
    && ((!input_fail && !rtt_fail) ==> (
        result.is_Ok()
        && GranuleAt(new_s, rtt).state == RTT
        && rtte.state == TABLE
        && rtte.addr == rtt
        && (AddrIsProtected(old_s, ipa, realm) ==> RttAllEntriesRipas(new_s, new_rtt, rtte_pre.ripas))
        && RttAllEntriesState(new_s, new_rtt, rtte_pre.state)
        && ((rtte_pre.state != UNASSIGNED && rtte_pre.state != UNASSIGNED_NS) ==> RttAllEntriesContiguous(new_s, new_rtt, rtte_pre.addr, level as int))
    ))
}
