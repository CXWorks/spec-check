pub open spec fn rsi_realm_config_spec(addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(addr) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsProtected(addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
    && (RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
    && ((AddrIsGranuleAligned(addr)
        && AddrIsProtected(addr, CurrentRealm(old_s))
        && RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas != EMPTY)
        ==> (result == RSI_SUCCESS
            && RsiRealmConfigAt(new_s, addr).ipa_width == CurrentRealm(new_s).ipa_width
            && RsiRealmConfigAt(new_s, addr).hash_algo == CurrentRealm(new_s).hash_algo
            && RsiRealmConfigAt(new_s, addr).num_aux_planes == CurrentRealm(new_s).num_aux_planes
            && RsiRealmConfigAt(new_s, addr).ats_plane == CurrentRealm(new_s).ats_plane))
}