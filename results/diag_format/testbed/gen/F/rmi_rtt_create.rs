pub open spec fn rmi_rtt_create_spec(rd: Address, rtt: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    // Failure: rd not aligned to granule
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: rd not delegable
    (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: Granule at rd not in RD state
    (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: level not valid or is starting level
    (!RttLevelIsValid(old_s, RealmAt(old_s, rd), level) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: ipa not aligned to parent entry size
    (!AddrIsAligned(old_s, ipa, RttLevelSize(old_s, level - 1 as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: ipa outside IPA space
    (!AddrIsWithin(old_s, ipa, 0, pow2(old_s, RealmAt(old_s, rd).ipa_width as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: rtt not aligned to granule
    (!AddrIsGranuleAligned(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: rtt not delegable in DRAM
    (!PaIsDelegable(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: Granule at rtt not in DELEGATED state
    (GranuleAt(old_s, rtt).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: LPA2 not supported and rtt >= 2^48
    (!Equal(old_s, RealmAt(old_s, rd).feat_lpa2, FEATURE_TRUE) && (rtt as int) >= (1u64 << 48) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: RTT walk ends below level - 1
    (RttWalk(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY).level < (level - 1) as int ==> ResultEqual(result, RMI_ERROR_RTT))
    // Failure: RTT entry found is already in TABLE state
    (RttWalk(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY).rtte.state == TABLE ==> ResultEqual(result, RMI_ERROR_RTT))
    // Success: Granule at rtt is in RTT state
    (GranuleAt(new_s, rtt).state == RTT)
    // Success: RTT entry found by walk is in TABLE state
    (RttWalk(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY).rtte.state == TABLE)
    // Success: That RTT entry points to rtt
    (RttWalk(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY).rtte.addr == rtt)
    // Success: If ipa is Protected IPA, every entry in new RTT has RIPAS value that parent had
    (AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> (forall rtte: RmmRttEntry |
        let rtte_idx: int = RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY).level as int);
        let rtte: RmmRttEntry = RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY).rtte.addr), rtte_idx);
        let parent_rtte: RmmRttEntry = RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY).rtte.addr));
        rtte.ripas == parent_rtte.ripas))
    // Success: Every entry in new RTT has state that parent had
    (forall rtte: RmmRttEntry |
        let rtte_idx: int = RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY).level as int);
        let rtte: RmmRttEntry = RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY).rtte.addr), rtte_idx);
        let parent_rtte: RmmRttEntry = RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY).rtte.addr));
        rtte.state == parent_rtte.state)
    // Success: If parent was not UNASSIGNED/UNASSIGNED_NS, entries map contiguous output range starting at parent's address
    (forall rtte: RmmRttEntry |
        let rtte_idx: int = RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY).level as int);
        let rtte: RmmRttEntry = RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY).rtte.addr), rtte_idx);
        let parent_rtte: RmmRttEntry = RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY).rtte.addr));
        (parent_rtte.state != UNASSIGNED && parent_rtte.state != UNASSIGNED_NS) ==> (
            let parent_addr: Address = parent_rtte.addr;
            let parent_level: int = RttWalk(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY).level as int;
            let entry_level: int = RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa, level - 1 as int, RMM_RTT_TREE_PRIMARY).level as int);
            let entry_size: int = RttLevelSize(old_s, entry_level as int);
            let start_addr: Address = parent_addr;
            let end_addr: Address = start_addr + (entry_size as int) * (1u64 << (level - entry_level - 1) as int);
            AddrRangeIsWithin(old_s, start_addr, end_addr, start_addr, end_addr)))
}