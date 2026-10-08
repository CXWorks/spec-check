pub open spec fn rsi_attestation_token_continue_spec(addr: Address, offset: UInt64, size: UInt64, result: RsiCommandReturnCode, len: UInt64, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, CurrentRealm(old_s), addr) ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, CurrentRealm(old_s), addr) ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (RipasAt(old_s, CurrentRealm(old_s), addr).state == EMPTY ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (offset >= RMM_GRANULE_SIZE ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (offset + size < offset ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (offset + size > RMM_GRANULE_SIZE ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (CurrentRec(old_s).attest_state != ATTEST_IN_PROGRESS ==> ResultEqual(result, RSI_ERROR_STATE))
  && (result == RSI_SUCCESS ==> CurrentRec(new_s).attest_state == NO_ATTEST_IN_PROGRESS)
  && ((AddrIsGranuleAligned(old_s, CurrentRealm(old_s), addr) &&
       AddrIsProtected(old_s, CurrentRealm(old_s), addr) &&
       !(RipasAt(old_s, CurrentRealm(old_s), addr).state == EMPTY) &&
       !(offset >= RMM_GRANULE_SIZE) &&
       !(offset + size < offset) &&
       !(offset + size > RMM_GRANULE_SIZE) &&
       !(CurrentRec(old_s).attest_state != ATTEST_IN_PROGRESS))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> CurrentRec(new_s).attest_state == CurrentRec(old_s).attest_state)
}