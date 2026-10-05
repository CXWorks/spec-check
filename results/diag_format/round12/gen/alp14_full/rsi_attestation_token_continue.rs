pub open spec fn rsi_attestation_token_continue_spec(addr: Address, offset: UInt64, size: UInt64, result: RsiCommandReturnCode, len: UInt64, old_s: S, new_s: S) -> bool {
  ((addr % RMM_GRANULE_SIZE) != 0 ==> result == RSI_ERROR_INPUT)
  && (RealmAt(old_s, current_rec(old_s)).ripas[addr as int] == EMPTY ==> result == RSI_ERROR_INPUT)
  && (offset >= RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT)
  && ((offset + size) < offset ==> result == RSI_ERROR_INPUT)
  && (offset + size > RMM_GRANULE_SIZE ==> result == RSI_ERROR_INPUT)
  && (RECAt(old_s, current_rec(old_s)).attestation_state != ATTEST_IN_PROGRESS ==> result == RSI_ERROR_STATE)
  && ((result == RSI_SUCCESS || result == RSI_INCOMPLETE) ==> RECAt(new_s, current_rec(new_s)).attestation_state == NO_ATTEST_IN_PROGRESS)
  && ((!( (addr % RMM_GRANULE_SIZE) != 0) &&
       !(RealmAt(old_s, current_rec(old_s)).ripas[addr as int] == EMPTY) &&
       !(offset >= RMM_GRANULE_SIZE) &&
       !((offset + size) < offset) &&
       !(offset + size > RMM_GRANULE_SIZE) &&
       !(RECAt(old_s, current_rec(old_s)).attestation_state != ATTEST_IN_PROGRESS))
    ==> result == RSI_SUCCESS)
  && (result == RSI_ERROR_UNKNOWN ==> RECAt(new_s, current_rec(new_s)).attestation_state == ATTEST_IN_PROGRESS)
  && (result != RSI_SUCCESS && result != RSI_INCOMPLETE ==> RECAt(new_s, current_rec(new_s)).attestation_state == RECAt(old_s, current_rec(old_s)).attestation_state)
}