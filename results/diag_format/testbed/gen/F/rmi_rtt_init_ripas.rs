pub open spec fn rmi_rtt_init_ripas_spec(rd: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let walk = RttWalk_(old_s, rd, base, RMM_RTT_PAGE_LEVEL as int);
    let walk_level = walk.level as int;
    let rtte = RttAt(old_s, walk.rtt_addr);
    let walk_top = RttSkipEntriesIfNotState(old_s, RttAt(old_s, walk.rtt_addr), walk_level, base, top, RmmRttEntryState::UNASSIGNED);
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAt(old_s, rd).state == RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!realm_pre.state == REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM))
    && (top <= base ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, (top - RmmGranuleSize) as Address, realm_pre) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, base, walk_level) ==> ResultEqual(result, RMI_ERROR_RTT(walk_level)))
    && (!rtte.state == UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(walk_level)))
    && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (base == walk_top ==> ResultEqual(result, RMI_ERROR_RTT(walk_level)))
    && (result.is_Ok() ==> out_top == walk_top)
    && (result.is_Ok() ==> new_s.realm.measurements[0] == old_s.realm.measurements[0])
}