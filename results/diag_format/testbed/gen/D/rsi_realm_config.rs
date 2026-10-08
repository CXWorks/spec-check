pub open spec fn rsi_realm_config_spec(addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsProtected(old_s, addr) ==> result == RSI_ERROR_INPUT)
    && (GranuleAt(old_s, addr).state == RMM_RTT_ENTRY_UNASSIGNED ==> result == RSI_ERROR_INPUT)
    && (result == RSI_SUCCESS ==> (cfg.ipa_width == old_s.CurrentRealm().ipa_width && cfg.hash_algo == old_s.CurrentRealm().hash_algo && cfg.num_aux_planes == old_s.CurrentRealm().num_aux_planes && cfg.ats_plane == old_s.CurrentRealm().ats_plane))
}