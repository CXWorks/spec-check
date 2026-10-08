pub open spec fn rmi_rtt_destroy_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, top: Address, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, 0);
    let entry_idx = RttEntryIndex(ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(RttAt(walk.rtt_addr), walk.level, ipa);
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(old_s, realm, level) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsAligned(old_s, ipa, pow2(RttLevelSize(level - 1) as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa >= pow2(realm.ipa_width as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT))
    && (walk.rtte.state != TABLE ==> ResultEqual(result, RMI_ERROR_RTT))
    && (walk.rtte.state == TABLE && GranuleAt(old_s, walk.rtte.addr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_RTT))
    && (walk.rtte.state == TABLE && AuxAlias32(old_s, RttAt(walk.rtt_addr).rtt_addr, walk.rtte.addr, 32, 1) ==> ResultEqual(result, RMI_ERROR_RTT))
    && (result.is_Ok() ==> (walk.rtte.state == UNASSIGNED && walk.rtte.ripas == DESTROYED || walk.rtte.state == UNASSIGNED_NS))
    && (result.is_Ok() ==> GranuleAt(old_s, walk.rtte.addr).state == DELEGATED)
    && (result.is_Ok() ==> rtt == walk.rtte.addr)
    && (result.is_Ok() ==> top == walk_top)
    && (GranuleAt(old_s, rd).state == RD ==> GranuleAt(new_s, rd).state == DELEGATED)
    && (GranuleAt(old_s, walk.rtte.addr).state == UNASSIGNED || GranuleAt(old_s, walk.rtte.addr).state == UNASSIGNED_NS ==> GranuleAt(new_s, walk.rtte.addr).state == DELEGATED)
    && (GranuleAt(old_s, walk.rtte.addr).state == UNASSIGNED && walk.rtte.ripas == DESTROYED ==> GranuleAt(new_s, walk.rtte.addr).state == DELEGATED)
    && (GranuleAt(old_s, walk.rtte.addr).state == UNASSIGNED_NS ==> GranuleAt(new_s, walk.rtte.addr).state == DELEGATED)
}