pub open spec fn rsi_realm_config_spec(addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> CurrentRealm(new_s).ipa_width == RealmConfig(new_s, addr).ipa_width)
  && (result == RSI_SUCCESS ==> Equal(RealmConfig(new_s, addr).hash_algo, CurrentRealm(new_s).hash_algo))
  && ((AddrIsGranuleAligned(old_s, addr) &&
       AddrIsProtected(old_s, addr, CurrentRealm(old_s)))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> CurrentRealm(new_s).ipa_width == CurrentRealm(old_s).ipa_width)
  && (result != RSI_SUCCESS
    ==> CurrentRealm(new_s).hash_algo == CurrentRealm(old_s).hash_algo)
}