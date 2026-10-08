pub open spec fn rsi_vdev_get_info_spec(vdev_id: Bits64, addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    let vdev = VdevFromVdevId(old_s, realm, vdev_id);
    let pdev = PdevAt(old_s, vdev.pdev);
    let cfg = RsiVdevInfoAt(old_s, addr);
    let walk = RttWalk(old_s, realm, addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int);

    (!((ImplFeatures(old_s).feat_da == RMM_FEATURE_TRUE) && (ImplFeatures(old_s).feat_ats == RMM_FEATURE_TRUE)) ==> result == RSI_ERROR_STATE)
    && ((vdev_id != VdevFromVdevId(old_s, realm, vdev_id).vdev_id) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsAligned(old_s, addr, 512) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsProtected(old_s, addr, realm) ==> result == RSI_ERROR_INPUT)
    && (RmmRipas(old_s, addr) == RMM_RIPAS_EMPTY ==> result == RSI_ERROR_INPUT)
    && (result == RSI_SUCCESS ==> (cfg.hash_algo == pdev.hash_algo) && (cfg.p2p_enabled == PdevFlags(old_s, pdev).p2p) && (cfg.p2p_bound == VdevAt(old_s, addr).p2p_bound) && (cfg.p2p_peer == VdevAt(old_s, addr).p2p_peer) && (VdevAttestInfoEqual1(VdevAttestInfoEqual1(old_s, vdev.vdev_attest_info_1, vdev.vdev_attest_info_2), cfg) == true) && (cfg.state == VdevAt(old_s, addr).vdev_state)))
}