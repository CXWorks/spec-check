pub open spec fn RSI_HOST_CALL_spec(s: S, addr: Address, result: RsiCommandReturnCode) -> bool {
    let realm = CurrentRealm(s);
    let rec = CurrentRec(s);
    let data = RsiHostCallAt(s, addr);
    let walk = RttWalk(s, realm, addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    
    (!AddrIsAligned(addr, 256) ==> result == RSI_ERROR_INPUT) &&
    (!AddrIsProtected(s, addr, realm) ==> result == RSI_ERROR_INPUT) &&
    (walk.rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT) &&
    (
        (AddrIsAligned(addr, 256) && AddrIsProtected(s, addr, realm) && walk.rtte.ripas != EMPTY) ==> result == RSI_SUCCESS
    )
}