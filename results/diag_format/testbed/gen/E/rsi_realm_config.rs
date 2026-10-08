pub open spec fn rsi_realm_config_spec(addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, CurrentRealm(old_s), addr) ==> result == RSI_ERROR_INPUT)
  && (GranuleAt(old_s, CurrentRealm(old_s), addr).state == EMPTY ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS)
  && ((AddrIsGranuleAligned(old_s, addr) &&
       AddrIsProtected(old_s, CurrentRealm(old_s), addr) &&
       !(GranuleAt(old_s, CurrentRealm(old_s), addr).state == EMPTY))
    ==> result == RSI_SUCCESS)
}