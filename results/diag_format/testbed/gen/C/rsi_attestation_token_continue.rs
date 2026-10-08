pub open spec fn rsi_attestation_token_continue_spec(addr: Address, offset: UInt64, size: UInt64, result: RsiCommandReturnCode, len: UInt64, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (RttWalk_(old_s, CurrentRec(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
  && (offset >= RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT)
  && (offset + size < offset ==> result == RSI_ERROR_INPUT)
  && (offset + size > RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT)
  && (CurrentRec(old_s).attest_state != ATTEST_IN_PROGRESS ==> result == RSI_ERROR_STATE)
  && (AttestationTokenGenerationFailed(old_s, CurrentRec(old_s)) ==> result == RSI_ERROR_UNKNOWN)
  && (result == RSI_SUCCESS ==> len == AttestationTokenWrite(new_s, addr, offset as int, size as int))
  && (result == RSI_SUCCESS ==> !AttestationTokenGenerationComplete(new_s, CurrentRec(new_s)) ==> result == RSI_INCOMPLETE)
  && (result == RSI_SUCCESS ==> AttestationTokenGenerationComplete(new_s, CurrentRec(new_s)) ==> CurrentRec(new_s).attest_state == NO_ATTEST_IN_PROGRESS)
  && ((AddrIsGranuleAligned(old_s, addr) &&
       AddrIsProtected(old_s, addr, CurrentRealm(old_s)) &&
       !(RttWalk_(old_s, CurrentRec(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == EMPTY) &&
       !(offset >= RMM_GRANULE_SIZE) &&
       !(offset + size < offset) &&
       !(offset + size > RMM_GRANULE_SIZE) &&
       !(CurrentRec(old_s).attest_state != ATTEST_IN_PROGRESS) &&
       !(AttestationTokenGenerationFailed(old_s, CurrentRec(old_s))))
    ==> result == RSI_SUCCESS)
}