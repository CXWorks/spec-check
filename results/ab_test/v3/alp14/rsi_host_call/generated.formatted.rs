pub open spec fn RSI_HOST_CALL_spec(s: S, addr: Address, result: RsiCommandReturnCode) -> bool {
    let realm = CurrentRealm(s);
    let walk = RttWalk(s, realm, addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let data = RsiHostCallAt(s, addr);

    (!AddrIsAligned(s, addr, 256) ==> result == RSI_ERROR_INPUT) && (!AddrIsProtected(
        s,
        addr,
        realm,
    ) ==> result == RSI_ERROR_INPUT) && (walk.rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
}