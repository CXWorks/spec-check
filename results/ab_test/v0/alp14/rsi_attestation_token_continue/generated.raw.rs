```verus
pub open spec fn RSI_ATTESTATION_TOKEN_CONTINUE_spec(
    s: S,
    addr: Address,
    offset: u64,
    size: u64,
    result: RsiCommandReturnCode,
    len: u64,
) -> bool {
    let realm = CurrentRealm(s);
    let rec = CurrentRec(s);
    let walk = RttWalk(s, realm, addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);

    (
        // Failure conditions
        (!AddrIsGranuleAligned(addr) ==> result == RSI_ERROR_INPUT) &&
        (!AddrIsProtected(addr, realm) ==> result == RSI_ERROR_INPUT) &&
        (walk.rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT) &&
        (offset >= RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT) &&
        (offset + size < offset ==> result == RSI_ERROR_INPUT) &&
        (offset + size > RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT) &&
        (rec.attest_state != ATTEST_IN_PROGRESS ==> result == RSI_ERROR_STATE) &&

        // Success conditions
        (
            (AddrIsGranuleAligned(addr) && AddrIsProtected(addr, realm) && 
             walk.rtte.ripas != EMPTY && offset < RMM_GRANULE_SIZE && 
             offset + size >= offset && offset + size <= RMM_GRANULE_SIZE && 
             rec.attest_state == ATTEST_IN_PROGRESS) ==>
            (
                (result == RSI_INCOMPLETE || 
                 (result == RSI_SUCCESS && rec.attest_state == NO_ATTEST_IN_PROGRESS)) &&
                len == AttestationTokenWrite(s, addr, offset, size)
            )
        )
    )
}
```