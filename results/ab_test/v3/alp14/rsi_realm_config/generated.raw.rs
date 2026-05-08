```verus
pub open spec fn RSI_REALM_CONFIG_spec(old_s: S, new_s: S, addr: Address) -> bool {
    let realm = CurrentRealm(old_s);
    let walk = RttWalk(old_s, realm, addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let cfg = RsiRealmConfigAt(old_s, addr);
    
    (
        // Failure condition: addr_align
        (!AddrIsGranuleAligned(addr) ==> old_s.result == RSI_ERROR_INPUT) &&
        
        // Failure condition: addr_bound
        (!AddrIsProtected(addr, realm) ==> old_s.result == RSI_ERROR_INPUT) &&
        
        // Failure condition: addr_empty
        (walk.rtte.ripas == EMPTY ==> old_s.result == RSI_ERROR_INPUT) &&
        
        // Success conditions (when no failure occurs)
        ((AddrIsGranuleAligned(addr) && AddrIsProtected(addr, realm) && walk.rtte.ripas != EMPTY) ==>
            (cfg.ipa_width == realm.ipa_width &&
             Equal(cfg.hash_algo, realm.hash_algo) &&
             cfg.num_aux_planes == realm.num_aux_planes &&
             cfg.ats_plane == realm.ats_plane &&
             new_s == old_s))
    )
}
```