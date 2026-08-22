pub open spec fn rsi_attestation_token_continue_spec(addr: Address, offset: UInt64, size: UInt64, result: RsiCommandReturnCode, len: UInt64, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(addr) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
    && (RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
    && ((offset as int) >= RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT)
    && ((offset as int) + (size as int) < (offset as int) ==> result == RSI_ERROR_INPUT)
    && ((offset as int) + (size as int) > RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT)
    && (CurrentRec(old_s).attest_state != ATTEST_IN_PROGRESS ==> result == RSI_ERROR_STATE)
    && ((AddrIsGranuleAligned(addr)
         && AddrIsProtected(old_s, addr, CurrentRealm(old_s))
         && RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas != EMPTY
         && (offset as int) < RMM_GRANULE_SIZE
         && (offset as int) + (size as int) >= (offset as int)
         && (offset as int) + (size as int) <= RMM_GRANULE_SIZE
         && CurrentRec(old_s).attest_state == ATTEST_IN_PROGRESS)
        ==> (result == RSI_SUCCESS || result == RSI_INCOMPLETE || result == RSI_ERROR_UNKNOWN))
    && (result == RSI_SUCCESS ==> CurrentRec(new_s).attest_state == NO_ATTEST_IN_PROGRESS)
    && (result == RSI_INCOMPLETE ==> CurrentRec(new_s).attest_state == ATTEST_IN_PROGRESS)
    && (result == RSI_ERROR_INPUT || result == RSI_ERROR_STATE
        ==> CurrentRec(new_s).attest_state == CurrentRec(old_s).attest_state)
}