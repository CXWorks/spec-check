pub open spec fn rsi_vdev_get_info_spec(vdev_id: Bits64, addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    let vdev = VdevFromVdevId(old_s, realm, vdev_id);
    let pdev = PdevAt(old_s, vdev.pdev);
    let cfg = RsiVdevInfoAt(old_s, addr);
    let walk = RttWalk(old_s, realm, addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int);
    (
        (realm.feat_da == RMM_FEATURE_FALSE ==> ResultEqual(result, RSI_ERROR_STATE))
        && (VdevIdIsFree(old_s, realm, vdev_id) ==> ResultEqual(result, RSI_ERROR_INPUT))
        && (!AddrIsAligned(old_s, addr, 512) ==> ResultEqual(result, RSI_ERROR_INPUT))
        && (!AddrIsProtected(old_s, addr, realm) ==> ResultEqual(result, RSI_ERROR_INPUT))
        && (walk.rtte.ripas == EMPTY ==> ResultEqual(result, RSI_ERROR_INPUT))
        && (result == RSI_SUCCESS ==> cfg.hash_algo == HashAlgorithmToRsi(pdev.hash_algo))
        && (result == RSI_SUCCESS ==> cfg.flags.p2p_enabled == FeatureToRsi(pdev.p2p_enabled))
        && (result == RSI_SUCCESS ==> cfg.flags.p2p_bound == FeatureToRsi(vdev.p2p_bound))
        && (result == RSI_SUCCESS ==> cfg.p2p_peer == vdev.p2p_peer)
        && (result == RSI_SUCCESS ==> VdevAttestInfoEqual(cfg.lock_nonce, cfg.meas_nonce, cfg.report_nonce, vdev.attest_info))
        && (result == RSI_SUCCESS ==> cfg.vca_digest == pdev.vca_digest)
        && (result == RSI_SUCCESS ==> cfg.meas_digest == vdev.meas_digest)
        && (result == RSI_SUCCESS ==> cfg.report_digest == vdev.report_digest)
        && (result == RSI_SUCCESS ==> cfg.state == VdevStateToRsi(vdev.vdev_state))
    )
}