pub open spec fn rsi_mem_set_perm_index_spec(base: Address, top: Address, perm_index: UInt64, handle: Bits64, result: RsiCommandReturnCode, new_base: Address, response: RsiResponse, new_handle: Bits64, old_s: S, new_s: S) -> bool {
  (!AddrIsRsiGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsRsiGranuleAligned(old_s, top) ==> result == RSI_ERROR_INPUT)
  && ((top) <= (base) ==> result == RSI_ERROR_INPUT)
  && (!AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RIPAS_EMPTY ==> result == RSI_ERROR_INPUT)
  && (perm_index >= RMM_NUM_PERM_OVERLAY_INDICES ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> CurrentRealm(new_s).overlay_locked[perm_index as int] == MEM_PERM_LOCKED)
  && (result == RSI_SUCCESS ==> new_base == CurrentRec(new_s).s2ap_addr)
  && (result == RSI_SUCCESS ==> response == RecS2APResponseToRsi(new_s, CurrentRec(new_s)))
  && ((AddrIsRsiGranuleAligned(old_s, base) &&
       AddrIsRsiGranuleAligned(old_s, top) &&
       !((top) <= (base)) &&
       AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)) &&
       !(RttWalk(old_s, CurrentRealm(old_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RIPAS_EMPTY) &&
       !(perm_index >= RMM_NUM_PERM_OVERLAY_INDICES))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> CurrentRealm(new_s).overlay_locked[perm_index as int] == CurrentRealm(old_s).overlay_locked[perm_index as int])
  && (result != RSI_SUCCESS
    ==> new_base == CurrentRec(old_s).s2ap_addr)
  && (result != RSI_SUCCESS
    ==> response == RecS2APResponseToRsi(old_s, CurrentRec(old_s)))
}