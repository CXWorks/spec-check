pub open spec fn rsi_ipa_state_get_spec(base: Address, top: Address, result: RsiCommandReturnCode, out_top: Address, ripas: RsiRipas, old_s: S, new_s: S) -> bool {
  ((!(AddrIsAligned(old_s, base, RMM_GRANULE_SIZE_ORDER as int)) ||
       !(AddrIsAligned(old_s, top, RMM_GRANULE_SIZE_ORDER as int)) ||
       !(top > base) ||
       CurrentRealm(old_s).state != REALM_ACTIVE)
    ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS
    ==> out_top > base)
  && (result == RSI_SUCCESS
    ==> out_top <= top)
  && ((!(AddrIsAligned(old_s, base, RMM_GRANULE_SIZE_ORDER as int)) ||
       !(AddrIsAligned(old_s, top, RMM_GRANULE_SIZE_ORDER as int)) ||
       !(top > base) ||
       CurrentRealm(old_s).state != REALM_ACTIVE)
    ==> result == RSI_ERROR_INPUT)
}