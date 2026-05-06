```verus
pub open spec fn RMI_RTT_INIT_RIPAS_spec(
    s: S,
    rd: Address,
    base: Address,
    top: Address,
    result: Result<Address, RmiStatusCode>
) -> bool {
    let realm_pre = RealmAt(s, rd);
    let walk = RttWalk(s, realm_pre, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let walk_rtt = RttAt(s, walk.rtt_addr);
    let walk_top = RttSkipEntriesIfNotState(s, walk_rtt, walk.level, base, top, UNASSIGNED);
    
    (
        // Failure conditions
        (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (GranuleAt(s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (UInt(top) <= UInt(base) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!AddrIsProtected(s, ToAddress(UInt(top) - RMM_GRANULE_SIZE), realm_pre) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (realm_pre.state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM)) &&
        (!AddrIsRttLevelAligned(base, walk.level) ==> ResultEqual(result, RMI_ERROR_RTT)) &&
        (walk.rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT)) &&
        (!AddrIsGranuleAligned(top) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (UInt(base) == UInt(walk_top) ==> ResultEqual(result, RMI_ERROR_RTT)) &&
        
        // Success conditions
        (result.is_Ok() ==> (
            let out_top = result.get_Ok_0();
            let realm = RealmAt(s, rd);
            RttEntriesInRangeRipas(s, walk_rtt, walk.level, base, walk_top, RAM) &&
            realm.measurements[0] == RimExtendRipas(s, realm_pre, base, walk_top, walk.level) &&
            out_top == walk_top
        ))
    )
}
```