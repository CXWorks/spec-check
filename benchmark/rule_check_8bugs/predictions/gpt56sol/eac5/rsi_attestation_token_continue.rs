pub open spec fn rsi_attestation_token_continue_spec(
    result: RsiCommandReturnCode,
    addr: Address,
    offset: UInt64,
    size: UInt64,
    len: UInt64,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT) && (!AddrIsProtected(
        old_s,
        addr,
        CurrentRealm(old_s),
    ) ==> result == RSI_ERROR_INPUT) && (offset >= RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT)
        && (offset + size < offset ==> result == RSI_ERROR_INPUT) && (offset + size
        > RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT) && (CurrentRec(old_s).attest_state
        != ATTEST_IN_PROGRESS ==> result == RSI_ERROR_STATE) && (AddrIsGranuleAligned(old_s, addr)
        && AddrIsProtected(old_s, addr, CurrentRealm(old_s)) && offset < RMM_GRANULE_SIZE && offset
        + size >= offset && offset + size <= RMM_GRANULE_SIZE && CurrentRec(old_s).attest_state
        == ATTEST_IN_PROGRESS ==> (result == RSI_INCOMPLETE || (result == RSI_SUCCESS && CurrentRec(
        new_s,
    ).attest_state == NO_ATTEST_IN_PROGRESS)))
}