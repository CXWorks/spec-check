pub open spec fn rsi_mem_set_perm_index_spec(base: Address, top: Address, perm_index: UInt64, cookie: Bits64, result: RsiCommandReturnCode, new_base: Address, response: RsiResponse, new_cookie: Bits64, old_s: S, new_s: S) -> bool {
  (AddrIsAligned(old_s, base, RMM_GRANULE_SIZE_ORDER as int) ==> result == RSI_SUCCESS)
  && (AddrIsAligned(old_s, top, RMM_GRANULE_SIZE_ORDER as int) ==> result == RSI_SUCCESS)
  && (top > base ==> result == RSI_SUCCESS)
  && (AddrIsWithin(old_s, base, CurrentRealm(old_s).rtt_base[0], CurrentRealm(old_s).rtt_base[0] + pow2(20)) ==> result == RSI_SUCCESS)
  && (perm_index >= RMM_NUM_PERM_OVERLAY_INDICES ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> CurrentRealm(new_s).overlay_locked[perm_index as int] == MEM_PERM_LOCKED)
  && (result == RSI_SUCCESS ==> new_base == CurrentRec(new_s).s2ap_addr)
  && (result == RSI_SUCCESS ==> response == RecS2APResponseToRsi(new_s, CurrentRec(new_s)))
  && ((!(AddrIsAligned(old_s, base, RMM_GRANULE_SIZE_ORDER as int)) ||
       !(AddrIsAligned(old_s, top, RMM_GRANULE_SIZE_ORDER as int)) ||
       !(top > base) ||
       !(AddrIsWithin(old_s, base, CurrentRealm(old_s).rtt_base[0], CurrentRealm(old_s).rtt_base[0] + pow2(20))) ||
       !(perm_index >= RMM_NUM_PERM_OVERLAY_INDICES))
    ==> result == RSI_ERROR_INPUT)
  && (result != RSI_SUCCESS
    ==> CurrentRealm(new_s).overlay_locked[perm_index as int] == CurrentRealm(old_s).overlay_locked[perm_index as int])
  && (result != RSI_SUCCESS
    ==> new_base == CurrentRec(old_s).s2ap_addr)
  && (result != RSI_SUCCESS
    ==> response == RecS2APResponseToRsi(old_s, CurrentRec(old_s)))
  && (result != RSI_SUCCESS
    ==> new_cookie == cookie)
}