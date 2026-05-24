pub open spec fn rsi_realm_config_spec(result: RsiCommandReturnCode, addr: Address, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    let cfg = RsiRealmConfigAt(old_s, addr);
    let walk = RttWalk(old_s, realm, addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    (!AddrIsGranuleAligned(addr) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsProtected(old_s, addr, realm) ==> result == RSI_ERROR_INPUT)
    && (walk.rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
    && ((AddrIsGranuleAligned(addr) && AddrIsProtected(old_s, addr, realm) && walk.rtte.ripas != EMPTY) ==>
        (result.is_Ok()
         && cfg.ipa_width == realm.ipa_width
         && Equal(cfg.hash_algo, realm.hash_algo)
         && cfg.num_aux_planes == realm.num_aux_planes
         && cfg.ats_plane == realm.ats_plane))
}