pub open spec fn rsi_realm_config_spec(addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsProtected(old_s, addr, old_s.CurrentRealm()) ==> result == RSI_ERROR_INPUT)
    && (old_s.RttWalk(old_s.CurrentRealm(), addr, 3, 0).rtte.ripas == RmmRipas::EMPTY ==> result == RSI_ERROR_INPUT)
    && (result == RSI_SUCCESS ==> cfg.ipa_width == old_s.CurrentRealm().ipa_width)
    && (result == RSI_SUCCESS ==> HashAlgorithmEqual(cfg.hash_algo, old_s.CurrentRealm().hash_algo))
    && (result == RSI_SUCCESS ==> cfg.num_aux_planes == old_s.CurrentRealm().num_aux_planes)
    && (result == RSI_SUCCESS ==> cfg.ats_plane == old_s.CurrentRealm().ats_plane)
}

pub open spec fn HashAlgorithmEqual(a: RsiHashAlgorithm, b: RmmHashAlgorithm) -> bool {
    match (a, b) {
        (RsiHashAlgorithm::RSI_HASH_SHA_256, RmmHashAlgorithm::HASH_SHA_256) => true,
        (RsiHashAlgorithm::RSI_HASH_SHA_512, RmmHashAlgorithm::HASH_SHA_512) => true,
        _ => false,
    }
}

pub open spec fn cfg: RsiRealmConfig {
    RsiRealmConfigAt(addr)
}