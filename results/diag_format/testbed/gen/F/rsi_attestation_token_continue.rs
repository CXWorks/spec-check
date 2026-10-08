pub open spec fn rsi_attestation_token_continue_spec(addr: Address, offset: UInt64, size: UInt64, result: RsiCommandReturnCode, len: UInt64, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);
    let walk = RttWalk_(old_s, realm as int, addr as int, RMM_RTT_PAGE_LEVEL as int);
    let granule_size = RMM_GRANULE_SIZE as int;
    let is_aligned = AddrIsAligned(old_s, addr, granule_size);
    let is_protected = AddrIsProtected(old_s, addr, realm);
    let ripas_empty = (walk.level as int) == 0 && (walk.rtte.ripas == RmmRipas::EMPTY);
    let offset_overflow = offset >= granule_size;
    let sum_overflow = (offset as int) + (size as int) < (offset as int);
    let sum_exceeds = (offset as int) + (size as int) > granule_size;
    let attest_in_progress = (old_s.rec.attest_state == RmmRecAttestState::ATTEST_IN_PROGRESS);
    let attest_complete = (new_s.rec.attest_state == RmmRecAttestState::NO_ATTEST_IN_PROGRESS);
    let token_complete = (result == RSI_SUCCESS);
    let token_incomplete = (result == RSI_INCOMPLETE);
    let token_failed_unknown = (result == RSI_ERROR_UNKNOWN);
    (is_aligned ==> (result == RSI_ERROR_INPUT))
    && (is_protected ==> (result == RSI_ERROR_INPUT))
    && (ripas_empty ==> (result == RSI_ERROR_INPUT))
    && (offset_overflow ==> (result == RSI_ERROR_INPUT))
    && (sum_overflow ==> (result == RSI_ERROR_INPUT))
    && (sum_exceeds ==> (result == RSI_ERROR_INPUT))
    && (!attest_in_progress ==> (result == RSI_ERROR_STATE))
    && (token_complete ==> (attest_complete))
    && (token_incomplete ==> (result == RSI_INCOMPLETE))
    && (token_failed_unknown ==> (result == RSI_ERROR_UNKNOWN))
    && (result == RSI_SUCCESS ==> (len <= size))
}