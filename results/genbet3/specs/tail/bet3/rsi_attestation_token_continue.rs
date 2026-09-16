pub open spec fn rsi_attestation_token_continue_spec(addr: Address, offset: UInt64, size: UInt64, result: RsiCommandReturnCode, len: UInt64, old_s: S, new_s: S) -> bool {
  (!AddrIsRsiGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RIPAS_EMPTY ==> result == RSI_ERROR_INPUT)
  && (offset >= RSI_GRANULE_SIZE ==> result == RSI_ERROR_INPUT)
  && (offset + size < offset ==> result == RSI_ERROR_INPUT)
  && (offset + size > RSI_GRANULE_SIZE ==> result == RSI_ERROR_INPUT)
  && (CurrentRec(old_s).attest_state != ATTEST_IN_PROGRESS ==> result == RSI_ERROR_STATE)
  && (result == RSI_SUCCESS ==> len == AttestationTokenWrite(new_s, addr, offset, size as int))
  && (result == RSI_SUCCESS && CurrentRec(old_s).attest_state == ATTEST_IN_PROGRESS && TokenGenerationIsComplete(old_s) ==> CurrentRec(new_s).attest_state == NO_ATTEST_IN_PROGRESS)
  && ((AddrIsRsiGranuleAligned(old_s, addr) &&
       AddrIsProtected(old_s, addr, CurrentRealm(old_s)) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RIPAS_EMPTY) &&
       !(offset >= RSI_GRANULE_SIZE) &&
       !(offset + size < offset) &&
       !(offset + size > RSI_GRANULE_SIZE) &&
       !(CurrentRec(old_s).attest_state != ATTEST_IN_PROGRESS))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> len == 0)
}