pub open spec fn rsi_ipa_state_get_spec(base: Address, top: Address, result: RsiCommandReturnCode, out_top: Address, ripas: RsiRipas, old_s: S, new_s: S) -> bool {
  (result == RSI_ERROR_INPUT && !(base % GRANULE_SIZE == 0))
  && (result == RSI_ERROR_INPUT && !(top % GRANULE_SIZE == 0))
  && (result == RSI_ERROR_INPUT && !(top <= base))
  && (result == RSI_ERROR_INPUT && !(range_from(base, top) is_within_protected_address_range(CurrentRealm(old_s))))
  && (result == RSI_SUCCESS ==> out_top > base)
  && (result == RSI_SUCCESS ==> out_top <= top)
  && ((!(result == RSI_ERROR_INPUT) &&
       !(result == RSI_ERROR_INPUT) &&
       !(result == RSI_ERROR_INPUT) &&
       !(result == RSI_ERROR_INPUT))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> out_top == 0)
  && (result != RSI_SUCCESS
    ==> ripas == 0)
}