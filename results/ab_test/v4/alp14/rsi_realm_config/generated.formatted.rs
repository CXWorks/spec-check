pub open spec fn RSI_REALM_CONFIG_spec(
    old_s: S,
    new_s: S,
    addr: Address,
    realm: RmmRealm,
    cfg: RsiRealmConfig,
    walk: RmmRttWalkResult,
) -> bool {
    ((!AddrIsGranuleAligned(addr) ==> old_s.outcome == RSI_ERROR_INPUT) && (!AddrIsProtected(
        old_s,
        addr,
        realm,
    ) ==> old_s.outcome == RSI_ERROR_INPUT) && (walk.rtte.ripas == EMPTY ==> old_s.outcome
        == RSI_ERROR_INPUT)) && ((AddrIsGranuleAligned(addr) && AddrIsProtected(old_s, addr, realm)
        && walk.rtte.ripas != EMPTY) ==> (old_s.outcome == RSI_SUCCESS && cfg.ipa_width
        == realm.ipa_width && cfg.hash_algo == realm.hash_algo && cfg.num_aux_planes
        == realm.num_aux_planes && cfg.ats_plane == realm.ats_plane))
}