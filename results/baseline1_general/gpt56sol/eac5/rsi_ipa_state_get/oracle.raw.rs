pub open spec fn rsi_ipa_state_get_spec(addr: Address, result: RsiCommandReturnCode, ripas: RsiRipas, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && ((AddrIsGranuleAligned(old_s, addr) &&
       AddrIsProtected(old_s, addr, CurrentRealm(old_s)))
    ==> result == RSI_SUCCESS)
}