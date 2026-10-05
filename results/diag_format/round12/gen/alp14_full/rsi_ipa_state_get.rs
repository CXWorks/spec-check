pub open spec fn rsi_ipa_state_get_spec(base: UInt64, top: UInt64, result: RsiCommandReturnCode, out_top: UInt64, ripas: UInt8, old_s: S, new_s: S) -> bool {
  ((base % GRANULE_SIZE) != 0 ==> result == RSI_ERROR_INPUT)
  && ((top % GRANULE_SIZE) != 0 ==> result == RSI_ERROR_INPUT)
  && (top <= base ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> out_top > base)
  && (result == RSI_SUCCESS ==> out_top <= top)
  && ((!( (base % GRANULE_SIZE) != 0) &&
       !( (top % GRANULE_SIZE) != 0) &&
       !(top <= base))
    ==> result == RSI_SUCCESS)
}