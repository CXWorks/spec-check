pub open spec fn rmi_rtt_create_spec(rd: Address, rtt: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk_pre = RttWalk_(old_s, rd, ipa, (level - 1) as int);
    let rtte_pre = RttEntryAt(RttAt(old_s, walk_pre.rtt_addr), RttEntryIndex(old_s, ipa, (walk_pre.level) as int));
    let walk = RttWalk_(old_s, rd, ipa, (level - 1) as int);
    let rtte = RttEntryAt(RttAt(old_s, walk.rtt_addr), RttEntryIndex(old_s, ipa, (walk.level) as int));

    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttLevelIsValid(old_s, realm, level) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RttLevelIsStarting(old_s, realm, level) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, (level - 1) as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa >= pow2(realm.ipa_width) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegableDram(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rtt).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (realm.feat_lpa2 == FEATURE_FALSE && rtt >= pow2(48) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    && (walk.rtte.state == TABLE ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    && (result.is_Ok() ==> (GranuleAt(new_s, rtt).state == RTT))
    && (result.is_Ok() ==> (walk.rtte.state == TABLE))
    && (result.is_Ok() ==> (walk.rtte.addr == rtt))
    && (result.is_Ok() ==> (AddrIsProtected(old_s, ipa, realm) ==> RttAllEntriesRipas(RttAt(new_s, rtt), rtte_pre.ripas)))
    && (result.is_Ok() ==> RttAllEntriesState(RttAt(new_s, rtt), rtte_pre.state))
    && (result.is_Ok() ==> ((rtte_pre.state != UNASSIGNED && rtte_pre.state != UNASSIGNED_NS) ==> RttAllEntriesContiguous(RttAt(new_s, rtt), rtte_pre.addr, level)))
}