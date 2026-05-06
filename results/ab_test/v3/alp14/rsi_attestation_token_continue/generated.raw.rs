```verus
pub open spec fn RSI_ATTESTATION_TOKEN_CONTINUE_spec(
    old_s: S,
    new_s: S,
    addr: Address,
    offset: u64,
    size: u64,
    result: RsiCommandReturnCode,
    len: u64,
) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);
    let walk = RttWalk(old_s, realm, addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    
    (
        // addr_align failure
        (!AddrIsGranuleAligned(addr) ==> result == RSI_ERROR_INPUT)
        &&
        // addr_bound failure
        (!AddrIsProtected(addr, realm) ==> result == RSI_ERROR_INPUT)
        &&
        // addr_empty failure
        (walk.rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
        &&
        // offset_bound failure
        (offset >= RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT)
        &&
        // size_overflow failure
        (offset + size < offset ==> result == RSI_ERROR_INPUT)
        &&
        // size_bound failure
        (offset + size > RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT)
        &&
        // state failure
        (rec.attest_state != ATTEST_IN_PROGRESS ==> result == RSI_ERROR_STATE)
        &&
        // Success conditions
        (
            (
                AddrIsGranuleAligned(addr)
                && AddrIsProtected(addr, realm)
                && walk.rtte.ripas != EMPTY
                && offset < RMM_GRANULE_SIZE
                && offset + size >= offset
                && offset + size <= RMM_GRANULE_SIZE
                && rec.attest_state == ATTEST_IN_PROGRESS
            ) ==> (
                (len == AttestationTokenWrite(addr, offset, size))
                && (
                    (result == RSI_INCOMPLETE)
                    || (
                        (result == RSI_OK)
                        && new_s.rec.attest_state == NO_ATTEST_IN_PROGRESS
                    )
                )
            )
        )
    )
}
```