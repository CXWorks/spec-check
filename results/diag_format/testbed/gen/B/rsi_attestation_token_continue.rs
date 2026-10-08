pub open spec fn rsi_attestation_token_continue_spec(addr: Address, offset: UInt64, size: UInt64, result: RsiCommandReturnCode, len: UInt64, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);
    let walk = RttWalk_(old_s, realm, addr, RMM_RTT_PAGE_LEVEL as int);
    let addr_is_aligned = AddrIsGranuleAligned(old_s, addr);
    let addr_is_protected = AddrIsProtected(old_s, addr, realm);
    let ripas_is_empty = walk.rtte.ripas == EMPTY;
    let offset_ge_granule_size = offset >= RMM_GRANULE_SIZE;
    let offset_size_overflow = offset + size < offset;
    let offset_size_exceeds_granule = offset + size > RMM_GRANULE_SIZE;
    let rec_attest_in_progress = rec.attest_state == ATTEST_IN_PROGRESS;
    let token_generation_failed = AttestationTokenGenerationFailed(old_s, rec);
    let rec_attest_complete = AttestationTokenGenerationComplete(old_s, rec);
    let rec_attest_state_no_progress = new_s.recs[new_s.recs.len() - 1].attest_state == NO_ATTEST_IN_PROGRESS;
    let len_eq_write = len == AttestationTokenWrite(old_s, addr, offset as int, size as int);
    let incomplete_condition = !AttestationTokenGenerationComplete(old_s, rec);
    let complete_condition = AttestationTokenGenerationComplete(old_s, rec);
    (!addr_is_aligned ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (!addr_is_protected ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (ripas_is_empty ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (offset_ge_granule_size ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (offset_size_overflow ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (offset_size_exceeds_granule ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (rec_attest_in_progress ==> ResultEqual(result, RSI_ERROR_STATE))
    && (token_generation_failed ==> ResultEqual(result, RSI_ERROR_UNKNOWN))
    && (len_eq_write)
    && (incomplete_condition ==> ResultEqual(result, RSI_INCOMPLETE))
    && (complete_condition ==> rec_attest_state_no_progress)
}