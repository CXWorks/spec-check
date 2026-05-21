pub open spec fn rsi_host_call_spec(
    result: RsiCommandReturnCode,
    addr: Address,
    old_s: S,
    new_s: S,
) -> bool {
    let realm = CurrentRealm(old_s);
    let walk = RttWalk(old_s, realm, addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    (!AddrIsAligned(addr, 256) ==> result == RSI_ERROR_INPUT) && (!AddrIsProtected(addr, realm)
        ==> result == RSI_ERROR_INPUT) && (walk.rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
        && ((AddrIsAligned(addr, 256) && AddrIsProtected(addr, realm) && walk.rtte.ripas != EMPTY)
        ==> result == RSI_SUCCESS)
}