pub open spec fn rsi_vsmmu_get_info_spec(addr: Address, result: RsiCommandReturnCode, top: Address, old_s: S, new_s: S) -> bool {
  ((addr) % GRANULE_SIZE != 0 ==> result == RSI_ERROR_INPUT)
  && (!IsProtectedIpa(old_s, addr) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS && RttWalk(old_s, addr, 0 as int).state != ASSIGNED_VSMMU ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS && (RttWalk(old_s, addr, 0 as int).vsmmu_base != addr) ==> result == RSI_ERROR_INPUT)
  && ((!(addr) % GRANULE_SIZE != 0 &&
       IsProtectedIpa(old_s, addr) &&
       !(result == RSI_SUCCESS && RttWalk(old_s, addr, 0 as int).state != ASSIGNED_VSMMU) &&
       !(result == RSI_SUCCESS && (RttWalk(old_s, addr, 0 as int).vsmmu_base != addr)))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> top == 0)
}