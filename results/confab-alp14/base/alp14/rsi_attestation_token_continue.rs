pub open spec fn rsi_attestation_token_continue_spec(result: RsiCommandReturnCode, len: u64, addr: Address, offset: u64, size: u64, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(addr) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsProtected(addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
    && (RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
    && (offset >= RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT)
    && ((offset + size) < offset ==> result == RSI_ERROR_INPUT)
    && ((offset + size) > RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT)
    && (CurrentRec(old_s).attest_state != ATTEST_IN_PROGRESS ==> result == RSI_ERROR_STATE)
    && (AddrIsGranuleAligned(addr)
        && AddrIsProtected(addr, CurrentRealm(old_s))
        && RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas != EMPTY
        && offset < RMM_GRANULE_SIZE
        && (offset + size) >= offset
        && (offset + size) <= RMM_GRANULE_SIZE
        && CurrentRec(old_s).attest_state == ATTEST_IN_PROGRESS
        ==> (result == RSI_SUCCESS || result == RSI_INCOMPLETE || result == RSI_ERROR_UNKNOWN)
            && len == AttestationTokenWrite(old_s, addr, offset, size)
            && (result == RSI_SUCCESS ==> CurrentRec(new_s).attest_state == NO_ATTEST_IN_PROGRESS))
}