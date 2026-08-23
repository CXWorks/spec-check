pub open spec fn rsi_realm_config_spec(addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
    && (RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
    && ((AddrIsGranuleAligned(old_s, addr)
         && AddrIsProtected(old_s, addr, CurrentRealm(old_s))
         && RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas != EMPTY)
        ==> (result == RSI_SUCCESS
             && RsiRealmConfigAt(new_s, addr).ipa_width == CurrentRealm(new_s).ipa_width
             && Equal(CurrentRealm(new_s).hash_algo, RsiRealmConfigAt(new_s, addr).hash_algo)
             && RsiRealmConfigAt(new_s, addr).num_aux_planes == CurrentRealm(new_s).num_aux_planes
             && RsiRealmConfigAt(new_s, addr).ats_plane == CurrentRealm(new_s).ats_plane))
}