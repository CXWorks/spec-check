pub open spec fn rsi_host_call_spec(
    addr: Address,
    result: RsiCommandReturnCode,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsAligned(old_s, addr, 256) ==> result == RSI_ERROR_INPUT) && (!AddrIsProtected(
        old_s,
        addr,
        CurrentRealm(old_s),
    ) ==> result == RSI_ERROR_INPUT) && (RttWalk(
        old_s,
        CurrentRealm(old_s),
        addr,
        RMM_RTT_PAGE_LEVEL as int,
        RMM_RTT_TREE_PRIMARY as int,
    ).rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT) && ((AddrIsAligned(old_s, addr, 256)
        && AddrIsProtected(old_s, addr, CurrentRealm(old_s)) && !(RttWalk(
        old_s,
        CurrentRealm(old_s),
        addr,
        RMM_RTT_PAGE_LEVEL as int,
        RMM_RTT_TREE_PRIMARY as int,
    ).rtte.ripas == EMPTY)) ==> result == RSI_SUCCESS)
}