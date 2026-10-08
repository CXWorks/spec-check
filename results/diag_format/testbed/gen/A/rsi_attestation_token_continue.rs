pub open spec fn rsi_attestation_token_continue_spec(addr: Address, offset: UInt64, size: UInt64, result: RsiCommandReturnCode, len: UInt64, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);
    let walk = RttWalk(old_s, realm, addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let input_fail = !AddrIsGranuleAligned(old_s, addr)
        || !AddrIsProtected(old_s, addr, realm)
        || walk.rtte.ripas == EMPTY
        || (offset as int) >= (RMM_GRANULE_SIZE as int)
        || (offset as int) + (size as int) > 0xFFFF_FFFF_FFFF_FFFF
        || (offset as int) + (size as int) > (RMM_GRANULE_SIZE as int);
    let state_fail = rec.attest_state != ATTEST_IN_PROGRESS;
    (input_fail ==> (result == RSI_ERROR_INPUT || (state_fail && result == RSI_ERROR_STATE)))
    && (state_fail ==> (result == RSI_ERROR_STATE || (input_fail && result == RSI_ERROR_INPUT)))
    && ((!input_fail && !state_fail) ==> (
        result == RSI_ERROR_UNKNOWN
        || (
            (result == RSI_SUCCESS || result == RSI_INCOMPLETE)
            && (len as int) == AttestationTokenWrite(old_s, addr, offset as int, size as int)
            && (result == RSI_INCOMPLETE ==> CurrentRec(new_s).attest_state == ATTEST_IN_PROGRESS)
            && (result == RSI_SUCCESS ==> CurrentRec(new_s).attest_state == NO_ATTEST_IN_PROGRESS)
        )
    ))
}
