pub open spec fn rmi_rtt_destroy_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, top: Address, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk_(old_s, rd, ipa, (level - 1) as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level as int);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level as int, ipa);
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(old_s, realm, level) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, (level - 1) as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa >= pow2(realm.ipa_width as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.level < level - 1 ==> (ResultEqual(result, RMI_ERROR_RTT(walk.level as int)) && top == walk_top))
    && (walk.rtte.state != RmmRttEntryState::TABLE ==> (ResultEqual(result, RMI_ERROR_RTT(walk.level as int)) && top == walk_top))
    && (RttIsLive(old_s, RttAt(old_s, walk.rtt_addr)) ==> (ResultEqual(result, RMI_ERROR_RTT(level as int)) && top == ipa))
    && (AddrIsAuxRef(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && (result.is_Ok() ==> (result.get_Ok_0() == () && rtt == walk.rtte.addr && top == walk_top))
    && (result.is_Ok() ==> GranuleAt(old_s, walk.rtte.addr).state == RmmGranuleState::DELEGATED)
    && (result.is_Ok() ==> AddrIsProtected(old_s, ipa, realm) ==> (walk.rtte.state == RmmRttEntryState::UNASSIGNED && walk.rtte.ripas == RmmRipas::DESTROYED))
    && (result.is_Ok() ==> !AddrIsProtected(old_s, ipa, realm) ==> walk.rtte.state == RmmRttEntryState::UNASSIGNED_NS)
    && (result.is_Ok() ==> GranuleAt(old_s, walk.rtte.addr).state == RmmGranuleState::DELEGATED)
    && (result.is_Ok() ==> GranuleAt(new_s, walk.rtte.addr).state == RmmGranuleState::DELEGATED)
}