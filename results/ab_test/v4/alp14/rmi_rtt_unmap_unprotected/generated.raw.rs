pub open spec fn RMI_RTT_UNMAP_UNPROTECTED_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    level: int,
    result: RmiCommandReturnCode,
    top: Address
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level, 0);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
    let rtte_before = RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx);
    let rtte_after = RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx);
    
    ((!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
     (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
     (GranuleAt(old_s, rd).state != RD as int ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
     ((!RttLevelIsValid(old_s, realm, level) || level < 1) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
     (!AddrIsRttLevelAligned(old_s, ipa, level) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
     ((UInt(ipa) >= (1 << realm.ipa_width) || AddrIsProtected(old_s, ipa, realm)) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
     (walk.level < level ==> (ResultEqual(result, (RMI_ERROR_RTT as int) + walk.level) && (top == walk_top))) &&
     (rtte_before.state != ASSIGNED_NS as int ==> (ResultEqual(result, (RMI_ERROR_RTT as int) + walk.level) && (top == walk_top))) &&
     ((walk.level >= level && rtte_before.state == ASSIGNED_NS as int) ==> (rtte_after.state == UNASSIGNED_NS as int && top == walk_top && ResultEqual(result, RMI_SUCCESS as int))))
}