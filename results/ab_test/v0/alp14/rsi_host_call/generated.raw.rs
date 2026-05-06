```verus
pub open spec fn RSI_HOST_CALL_spec(s: S, addr: Address, realm: RmmRealm, rec: RmmRec) -> bool {
    let data = RsiHostCallAt(s, addr);
    let walk = RttWalk(s, realm, addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    
    if !AddrIsAligned(s, addr, 256) {
        return false;
    }
    if !AddrIsProtected(s, addr, realm) {
        return false;
    }
    if walk.rtte.ripas == EMPTY {
        return false;
    }
    
    true
}
```