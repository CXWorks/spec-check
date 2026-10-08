pub open spec fn rsi_mem_set_perm_index_spec(base: Address, top: Address, perm_index: UInt64, cookie: Bits64, result: RsiCommandReturnCode, new_base: Address, response: RsiResponse, new_cookie: Bits64, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, base) ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (top <= base ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (!AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)) ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (perm_index >= RMM_NUM_PERM_OVERLAY_INDICES ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (!IsValidCookie(old_s, cookie) ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (result == RSI_SUCCESS ==> result == RSI_SUCCESS)
  && (result == RSI_SUCCESS ==> OverlayLockState(new_s, CurrentRealm(new_s), perm_index as int) == MEM_PERM_LOCKED)
  && (result == RSI_SUCCESS ==> new_base == CurrentRec(new_s).s2ap_addr)
  && (result == RSI_SUCCESS ==> response == RecS2APResponseToRsi(new_s, CurrentRec(new_s)))
  && (result == RSI_SUCCESS ==> IsNewlyGeneratedCookie(new_cookie))
  && ((AddrIsGranuleAligned(old_s, base) &&
       AddrIsGranuleAligned(old_s, top) &&
       !(top <= base) &&
       AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)) &&
       !(perm_index >= RMM_NUM_PERM_OVERLAY_INDICES) &&
       IsValidCookie(old_s, cookie))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> OverlayLockState(new_s, CurrentRealm(new_s), perm_index as int) == OverlayLockState(old_s, CurrentRealm(old_s), perm_index as int))
  && (result != RSI_SUCCESS
    ==> new_base == CurrentRec(new_s).s2ap_addr)
  && (result != RSI_SUCCESS
    ==> response == RecS2APResponseToRsi(new_s, CurrentRec(new_s)))
  && (result != RSI_SUCCESS
    ==> new_cookie == new_cookie)
}