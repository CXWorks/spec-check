pub open spec fn rmi_rtt_create_spec(rd: Address, rtt: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk_pre = RttWalk(old_s, realm, ipa, level - 1, 0);
    let entry_idx = RttEntryIndex(ipa, walk_pre.level);
    let rtte_pre = walk_pre.rtte;
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttLevelIsValid(old_s, realm, level) || level == RttLevelIsStarting(old_s, realm, level) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsAligned(old_s, ipa, pow2(RttLevelSize(level - 1) as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsWithin(old_s, ipa, 0, pow2(realm.ipa_width as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegableDram(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rtt).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (realm.feat_lpa2 == FEATURE_FALSE && rtt >= pow2(48) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk_pre.level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT))
    && (rtte_pre.state == TABLE ==> ResultEqual(result, RMI_ERROR_RTT))
    && (result.is_Ok() ==> (GranuleAt(new_s, rtt).state == RTT))
    && (result.is_Ok() ==> (RttEntryAt(RttAt(new_s, walk_pre.rtt_addr), entry_idx).state == TABLE))
    && (result.is_Ok() ==> (RttEntryAt(RttAt(new_s, walk_pre.rtt_addr), entry_idx).rtte == rtt))
    && (result.is_Ok() ==> (forall|entry: RmmRttEntry| entry.state == TABLE ==> entry.ripas == rtte_pre.ripas))
    && (result.is_Ok() ==> (forall|entry: RmmRttEntry| entry.state == TABLE ==> entry.state == rtte_pre.state))
}