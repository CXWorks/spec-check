pub open spec fn rsi_mem_set_perm_index_spec(base: Address, top: Address, perm_index: UInt64, cookie: Bits64, result: RsiCommandReturnCode, new_base: Address, response: RsiResponse, new_cookie: Bits64, old_s: S, new_s: S) -> bool {
  (result == RSI_ERROR_INPUT && is_aligned_to_granule(old_s, base) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_ERROR_INPUT && is_aligned_to_granule(old_s, top) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_ERROR_INPUT && top <= base ==> result == RSI_ERROR_INPUT)
  && (result == RSI_ERROR_INPUT && is_within_protected_ipa_space(old_s, CurrentRealm(old_s), base, top) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_ERROR_INPUT && perm_index >= RMM_NUM_PERM_OVERLAY_INDICES ==> result == RSI_ERROR_INPUT)
  && (result == RSI_ERROR_INPUT && CookieIsValid(old_s, cookie) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> CurrentRealm(new_s).overlay_locked[perm_index as int] == MEM_PERM_LOCKED)
  && (result == RSI_SUCCESS ==> new_base == CurrentRec(new_s).s2ap_addr)
  && (result == RSI_SUCCESS ==> response == CurrentRec(new_s).s2ap_response)
  && ((!(result == RSI_ERROR_INPUT && is_aligned_to_granule(old_s, base)) &&
       !(result == RSI_ERROR_INPUT && is_aligned_to_granule(old_s, top)) &&
       !(result == RSI_ERROR_INPUT && top <= base) &&
       !(result == RSI_ERROR_INPUT && is_within_protected_ipa_space(old_s, CurrentRealm(old_s), base, top)) &&
       !(result == RSI_ERROR_INPUT && perm_index >= RMM_NUM_PERM_OVERLAY_INDICES) &&
       !(result == RSI_ERROR_INPUT && CookieIsValid(old_s, cookie)))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> CurrentRealm(new_s).overlay_locked[perm_index as int] == CurrentRealm(old_s).overlay_locked[perm_index as int])
  && (result != RSI_SUCCESS
    ==> new_base == CurrentRec(old_s).s2ap_addr)
  && (result != RSI_SUCCESS
    ==> response == CurrentRec(old_s).s2ap_response)
  && (result != RSI_SUCCESS
    ==> new_cookie == CurrentRec(old_s).cookie)
  && (result == RSI_SUCCESS
    ==> CurrentRec(new_s).cookie == new_cookie)
}