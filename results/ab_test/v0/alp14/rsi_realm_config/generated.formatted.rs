pub open spec fn RSI_REALM_CONFIG_spec(s: S, addr: Address) -> bool {
    let realm = s.CurrentRealm();
    let cfg = RsiRealmConfigAt(s, addr);
    let walk = RttWalk(s, realm, addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);

    (AddrIsGranuleAligned(addr) && AddrIsProtected(s, addr, realm) && walk.rtte.ripas != EMPTY)
        ==> (cfg.ipa_width == realm.ipa_width && cfg.hash_algo == realm.hash_algo
        && cfg.num_aux_planes == realm.num_aux_planes && cfg.ats_plane == realm.ats_plane)
}