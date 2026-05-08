pub open spec fn RMI_RTT_INIT_RIPAS_spec(
    s: S,
    rd: Address,
    base: Address,
    top: Address,
) -> (result: RmiCommandReturnCode, out_top: Address)
    // Input validation - rd parameter
    requires AddrIsGranuleAligned(rd),
    requires PaIsDelegable(rd),
    requires GranuleAt(s, rd).state == RD,
    // Input validation - address range
    requires UInt(top) > UInt(base),
    requires AddrIsProtected(s, ToAddress(UInt(top) - RMM_GRANULE_SIZE)),
    requires AddrIsGranuleAligned(top),
    // Setup context
    let realm_pre = RealmAt(s, rd),
    requires realm_pre.state == REALM_NEW,
    let walk = RttWalk(s, realm_pre, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY),
    requires AddrIsRttLevelAligned(base, walk.level),
    requires walk.rtte.state == UNASSIGNED,
    let walk_top = RttSkipEntriesIfNotState(
        s, RttAt(s, walk.rtt_addr), walk.level, base, top, UNASSIGNED),
    requires UInt(base) != UInt(walk_top),
    
    ensures result == RMI_SUCCESS,
    ensures out_top == walk_top,
    ensures let realm = RealmAt(s, rd);
        RttEntriesInRangeRipas(s, RttAt(s, walk.rtt_addr), walk.level, base, walk_top, RAM),
    ensures let realm = RealmAt(s, rd);
        realm.measurements[0] == RimExtendRipas(s, realm_pre, base, walk_top, walk.level),
{
    (RMI_SUCCESS, walk_top)
}