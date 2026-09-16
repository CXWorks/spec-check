pub open spec fn rsi_ipa_state_get_spec(base: Address, top: Address, result: RsiCommandReturnCode, out_top: Address, ripas: RsiRipas, old_s: S, new_s: S) -> bool {
  (!AddrIsRsiGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsRsiGranuleAligned(old_s, top) ==> result == RSI_ERROR_INPUT)
  && ((top) <= (base) ==> result == RSI_ERROR_INPUT)
  && (!AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> out_top > base)
  && (result == RSI_SUCCESS ==> out_top <= top)
  && ((!(AddrIsRsiGranuleAligned(old_s, base)) &&
       AddrIsRsiGranuleAligned(old_s, top) &&
       !((top) <= (base)) &&
       AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)))
    ==> result == RSI_SUCCESS)
}