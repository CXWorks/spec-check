pub open spec fn rsi_vdev_get_info_spec(result: RsiCommandReturnCode, vdev_id: u64, addr: Address, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    let vdev = VdevFromVdevId(old_s, realm, vdev_id);
    let pdev = PdevAt(old_s, vdev.pdev);
    let cfg = RsiVdevInfoAt(new_s, addr);
    let walk = RttWalk(old_s, realm, addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    (realm.feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
    && (VdevIdIsFree(old_s, realm, vdev_id) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsAligned(old_s, addr, 512) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsProtected(old_s, addr, realm) ==> result == RSI_ERROR_INPUT)
    && (walk.rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
    && ((realm.feat_da == FEATURE_TRUE
        && !VdevIdIsFree(old_s, realm, vdev_id)
        && AddrIsAligned(old_s, addr, 512)
        && AddrIsProtected(old_s, addr, realm)
        && walk.rtte.ripas != EMPTY)
        ==> (result == RSI_SUCCESS
            && cfg.hash_algo == pdev.hash_algo
            && cfg.flags.p2p_enabled == pdev.p2p_enabled
            && cfg.flags.p2p_bound == vdev.p2p_bound
            && cfg.p2p_peer == vdev.p2p_peer
            && VdevAttestInfoEqual(cfg.lock_nonce as int, cfg.meas_nonce as int, cfg.report_nonce as int, vdev.attest_info)
            && cfg.vca_digest == pdev.vca_digest
            && cfg.meas_digest == vdev.meas_digest
            && cfg.report_digest == vdev.report_digest
            && cfg.state == vdev.vdev_state))
}