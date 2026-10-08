pub open spec fn rsi_attestation_token_continue_spec(addr: Address, offset: UInt64, size: UInt64, result: RsiCommandReturnCode, len: UInt64, old_s: S, new_s: S) -> bool {
  (AddrIsAligned(old_s, addr, 4096 as int) ==> result == RSI_SUCCESS)
  && (AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_SUCCESS)
  && (RmmRecAt(old_s, CurrentRec(old_s)).ripas_value == EMPTY ==> result == RSI_SUCCESS)
  && (offset >= RMM_GRANULE_SIZE ==> result == RSI_SUCCESS)
  && ((offset + size) < RMM_GRANULE_SIZE ==> result == RSI_SUCCESS)
  && (CurrentRec(old_s).attest_state == ATTEST_IN_PROGRESS ==> result == RSI_SUCCESS)
  && (result == RSI_INCOMPLETE ==> CurrentRec(new_s).attest_state == ATTEST_IN_PROGRESS)
  && (result == RSI_SUCCESS && CurrentRec(old_s).attest_state == ATTEST_IN_PROGRESS ==> CurrentRec(new_s).attest_state == NO_ATTEST_IN_PROGRESS)
  && ((!(AddrIsAligned(old_s, addr, 4096 as int)) ||
       !(AddrIsProtected(old_s, addr, CurrentRealm(old_s))) ||
       !(RmmRecAt(old_s, CurrentRec(old_s)).ripas_value == EMPTY) ||
       !(offset >= RMM_GRANULE_SIZE) ||
       !((offset + size) < RMM_GRANULE_SIZE) ||
       !(CurrentRec(old_s).attest_state == ATTEST_IN_PROGRESS))
    ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS && !(CurrentRec(old_s).attest_state == ATTEST_IN_PROGRESS)
    ==> result == RSI_ERROR_STATE)
  && (result != RSI_INCOMPLETE && result != RSI_SUCCESS
    ==> CurrentRec(new_s).attest_state == CurrentRec(old_s).attest_state)
  && (result == RSI_SUCCESS
    ==> CurrentRec(new_s).attest_state == NO_ATTEST_IN_PROGRESS)
}