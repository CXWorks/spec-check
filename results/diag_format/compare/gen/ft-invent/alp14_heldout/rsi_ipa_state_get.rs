pub open spec fn rsi_ipa_state_get_spec(base: Address, top: Address, result: RsiCommandReturnCode, out_top: Address, ripas: RsiRipas, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, top) ==> result == RSI_ERROR_INPUT)
  && ((top) <= (base) ==> result == RSI_ERROR_INPUT)
  && (!AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> out_top > base)
  && (result == RSI_SUCCESS ==> out_top <= top)
  && ((!(AddrIsGranuleAligned(old_s, base)) &&
       AddrIsGranuleAligned(old_s, top) &&
       !((top) <= (base)) &&
       AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)))
    ==> result == RSI_SUCCESS)
}