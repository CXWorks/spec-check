pub open spec fn rsi_realm_config_spec(result: RsiCommandReturnCode, addr: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
    && ((AddrIsGranuleAligned(old_s, addr)
        && AddrIsProtected(old_s, addr, CurrentRealm(old_s)))
        ==> (result == RSI_SUCCESS
            && RealmConfig(new_s, addr).ipa_width == CurrentRealm(old_s).ipa_width
            && Equal(CurrentRealm(old_s).hash_algo, RealmConfig(new_s, addr).hash_algo)))
}