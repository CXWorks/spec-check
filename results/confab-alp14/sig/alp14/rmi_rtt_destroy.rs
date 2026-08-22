pub open spec fn rmi_rtt_destroy_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, top: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int)
            || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, (level - 1) as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ipa as int) >= (1int << (RealmAt(old_s, rd).ipa_width as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).level < (level - 1) as int)
        ==> (ResultEqual(result, RMI_ERROR_RTT)
             && top == RttSkipNonLiveEntries(old_s,
                    RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).rtt_addr),
                    RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).level as int,
                    ipa)))
    && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).rtte.state != TABLE)
        ==> (ResultEqual(result, RMI_ERROR_RTT)
             && top == RttSkipNonLiveEntries(old_s,
                    RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).rtt_addr),
                    RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).level as int,
                    ipa)))
    && (RttIsLive(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).rtte.addr))
        ==> (ResultEqual(result, RMI_ERROR_RTT) && top == ipa))
    && (AddrIsAuxRef(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((AddrIsGranuleAligned(rd)
         && PaIsDelegable(old_s, rd)
         && GranuleAt(old_s, rd).state == RD
         && RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int)
         && !RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)
         && AddrIsRttLevelAligned(old_s, ipa, (level - 1) as int)
         && (ipa as int) < (1int << (RealmAt(old_s, rd).ipa_width as int))
         && RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).level >= (level - 1) as int
         && RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).rtte.state == TABLE
         && !RttIsLive(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).rtte.addr))
         && !AddrIsAuxRef(old_s, ipa, RealmAt(old_s, rd)))
        ==> (result.is_Ok()
             && (AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))
                 ==> (RttEntryAt(new_s,
                          RttAt(new_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).rtt_addr),
                          RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).level as int)).state == UNASSIGNED
                      && RttEntryAt(new_s,
                          RttAt(new_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).rtt_addr),
                          RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).level as int)).ripas == DESTROYED))
             && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))
                 ==> RttEntryAt(new_s,
                          RttAt(new_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).rtt_addr),
                          RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).level as int)).state == UNASSIGNED_NS)
             && GranuleAt(new_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).rtte.addr).state == DELEGATED
             && rtt == RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).rtte.addr
             && top == RttSkipNonLiveEntries(old_s,
                    RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).rtt_addr),
                    RttWalk(old_s, RealmAt(old_s, rd), ipa, (level - 1) as int, RMM_RTT_TREE_PRIMARY).level as int,
                    ipa)))
}