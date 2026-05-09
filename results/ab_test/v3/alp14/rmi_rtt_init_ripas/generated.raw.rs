pub open spec fn rmi_rtt_init_ripas_spec(result: RmiCommandReturnCode, rd: Address, base: Address, top: Address, out_top: Address, old_s: S, new_s: S) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm_pre, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let walk_top = RttSkipEntriesIfNotState(old_s, RttAt(old_s, walk.rtt_addr), walk.level, base, top, UNASSIGNED);
    let realm = RealmAt(new_s, rd);
    
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (UInt(top) <= UInt(base) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ToAddress(UInt(top) - RMM_GRANULE_SIZE), realm_pre) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (realm_pre.state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM))
    && (!AddrIsRttLevelAligned(base, walk.level) ==> result == RMI_ERROR_RTT || (result.is_RmiErrorRtt() && result.get_RmiErrorRtt_level() == walk.level))
    && (walk.rtte.state != UNASSIGNED ==> result == RMI_ERROR_RTT || (result.is_RmiErrorRtt() && result.get_RmiErrorRtt_level() == walk.level))
    && (!AddrIsGranuleAligned(top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (UInt(base) == UInt(walk_top) ==> result == RMI_ERROR_RTT || (result.is_RmiErrorRtt() && result.get_RmiErrorRtt_level() == walk.level))
    && (
        (AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD && UInt(top) > UInt(base) && AddrIsProtected(old_s, ToAddress(UInt(top) - RMM_GRANULE_SIZE), realm_pre) && realm_pre.state == REALM_NEW && AddrIsRttLevelAligned(base, walk.level) && walk.rtte.state == UNASSIGNED && AddrIsGranuleAligned(top) && UInt(base) != UInt(walk_top))
        ==> (result == RMI_SUCCESS && out_top == walk_top && RttEntriesInRangeRipas(old_s, RttAt(old_s, walk.rtt_addr), walk.level, base, walk_top, RAM) && realm.measurements[0] == RimExtendRipas(old_s, realm_pre, base, walk_top, walk.level))
    )
}