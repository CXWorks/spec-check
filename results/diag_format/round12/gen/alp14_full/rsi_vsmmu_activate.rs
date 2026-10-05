pub open spec fn rsi_vsmmu_activate_spec(base: UInt64, top: UInt64, result: RsiCommandReturnCode, new_base: UInt64, old_s: S, new_s: S) -> bool {
  ((base % GRANULE_SIZE) != 0 ==> result == RSI_ERROR_INPUT)
  && ((top % GRANULE_SIZE) != 0 ==> result == RSI_ERROR_INPUT)
  && (top <= base ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> (RIPAS(new_s, base..new_base) == DEV))
  && (result == RSI_SUCCESS && base == VSMMU_register_region_base(old_s) && new_base != top ==> VSMMU_state(new_s) == VSMMU_ACTIVATING)
  && (result == RSI_SUCCESS && new_base == top ==> VSMMU_state(new_s) == VSMMU_ACTIVE)
  && ((!( (base % GRANULE_SIZE) != 0) &&
       !( (top % GRANULE_SIZE) != 0) &&
       !(top <= base))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> RIPAS(new_s, base..new_base) == RIPAS(old_s, base..new_base))
  && (result != RSI_SUCCESS
    ==> VSMMU_state(new_s) == VSMMU_state(old_s))
  && (!(result == RSI_SUCCESS && (base == VSMMU_register_region_base(old_s) && new_base != top)) ==> VSMMU_state(new_s) == VSMMU_state(old_s))
}