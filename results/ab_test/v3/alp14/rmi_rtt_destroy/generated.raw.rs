pub open spec fn rmi_rtt_destroy_spec(result: RmiCommandReturnCode, rtt: Address, top: Address, rd: Address, ipa: Address, level: int, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(rd);
    let walk = RttWalk(realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(ipa, walk.level);
    let walk_rtt = RttAt(walk.rtt_addr);
    let walk_top = RttSkipNonLiveEntries(walk_rtt, walk.level, ipa);
    let rtte_at_walk = RttEntryAt(walk_rtt, entry_idx);

    (!AddrIsGranuleAligned(rd) ==> result == RMI_ERROR_INPUT)
    && (!PaIsDelegable(rd) ==> result == RMI_ERROR_INPUT)
    && (GranuleAt(old_s, rd).state != RD ==> result == RMI_ERROR_INPUT)
    && ((!RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(old_s, realm, level)) ==> result == RMI_ERROR_INPUT)
    && (!AddrIsRttLevelAligned(ipa, level - 1) ==> result == RMI_ERROR_INPUT)
    && (UInt(ipa) >= (1 << realm.ipa_width) ==> result == RMI_ERROR_INPUT)
    && (walk.level < level - 1 ==> (result == RMI_ERROR_RTT && top == walk_top))
    && (rtte_at_walk.state != TABLE ==> (result == RMI_ERROR_RTT && top == walk_top))
    && (RttIsLive(old_s, RttAt(old_s, rtte_at_walk.addr)) ==> (result == RMI_ERROR_RTT && top == ipa))
    && (AddrIsAuxRef(ipa, realm) ==> (result == RMI_ERROR_RTT && top == walk_top))
    && ((AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD
        && RttLevelIsValid(old_s, realm, level) && !RttLevelIsStarting(old_s, realm, level)
        && AddrIsRttLevelAligned(ipa, level - 1) && UInt(ipa) < (1 << realm.ipa_width)
        && walk.level == level - 1 && rtte_at_walk.state == TABLE
        && !RttIsLive(old_s, RttAt(old_s, rtte_at_walk.addr))
        && !AddrIsAuxRef(ipa, realm))
      ==> (result == RMI_SUCCESS
        && rtt == rtte_at_walk.addr
        && top == walk_top
        && (AddrIsProtected(ipa, realm) ==> RttEntryAt(RttAt(new_s, walk.rtt_addr), entry_idx).state == UNASSIGNED)
        && (AddrIsProtected(ipa, realm) ==> RttEntryAt(RttAt(new_s, walk.rtt_addr), entry_idx).ripas == DESTROYED)
        && (!AddrIsProtected(ipa, realm) ==> RttEntryAt(RttAt(new_s, walk.rtt_addr), entry_idx).state == UNASSIGNED_NS)
        && GranuleAt(new_s, rtte_at_walk.addr).state == DELEGATED))
}