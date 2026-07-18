pub open spec fn rsi_mem_set_perm_index_spec(base: Address, top: Address, perm_index: UInt64, cookie: Bits64, result: RsiCommandReturnCode, new_s: S) -> bool {
  (!AddrIsGranuleAligned(new_s, base) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(new_s, top) ==> result == RSI_ERROR_INPUT)
  && ((top) <= (base) ==> result == RSI_ERROR_INPUT)
  && (!AddrRangeIsProtected(new_s, base, top, CurrentRealm(new_s)) ==> result == RSI_ERROR_INPUT)
  && (perm_index >= RMM_NUM_PERM_OVERLAY_INDICES ==> result == RSI_ERROR_INPUT)
  && (CookieIsInvalid(new_s, cookie) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> CurrentRealm(new_s).overlay_locked[perm_index as int] == MEM_PERM_LOCKED)
  && (result == RSI_SUCCESS ==> new_base == CurrentRec(new_s).s2ap_addr)
  && (result == RSI_SUCCESS ==> RecS2APResponseToRsi(new_s, CurrentRec(new_s)) == response)
  && (result == RSI_SUCCESS ==> New cookie is generated)
  && ((AddrIsGranuleAligned(old_s, base) &&
       AddrIsGranuleAligned(old_s, top) &&
       !(UInt(top) <= UInt(base)) &&
       AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)) &&
       !(perm_index >= RMM_NUM_PERM_OVERLAY_INDICES) &&
       !(CookieIsInvalid(old_s, cookie)))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> CurrentRealm(new_s).overlay_locked[perm_index as int] == CurrentRealm(old_s).overlay_locked[perm_index as int])
}