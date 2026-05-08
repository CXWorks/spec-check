pub open spec fn RSI_VDEV_GET_INFO_spec(
    old_s: S,
    new_s: S,
    vdev_id: u64,
    addr: Address,
    result: RsiCommandReturnCode,
) -> bool {
    let realm = CurrentRealm(old_s);
    let vdev = VdevFromVdevId(old_s, realm, vdev_id);
    let pdev = PdevAt(old_s, vdev.pdev);
    let cfg = RsiVdevInfoAt(old_s, addr);
    let walk = RttWalk(old_s, realm, addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int);

    (realm.feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE) && (VdevIdIsFree(
        old_s,
        realm,
        vdev_id,
    ) ==> result == RSI_ERROR_INPUT) && (!AddrIsAligned(addr, 512) ==> result == RSI_ERROR_INPUT)
        && (!AddrIsProtected(old_s, addr, realm) ==> result == RSI_ERROR_INPUT) && (walk.rtte.ripas
        == EMPTY ==> result == RSI_ERROR_INPUT) && (result == RSI_SUCCESS ==> (Equal(
        cfg.hash_algo,
        pdev.hash_algo,
    ) && Equal(cfg.flags.p2p_enabled, pdev.p2p_enabled) && Equal(
        cfg.flags.p2p_bound,
        vdev.p2p_bound,
    ) && cfg.p2p_peer == vdev.p2p_peer && VdevAttestInfoEqual(
        cfg.lock_nonce,
        cfg.meas_nonce,
        cfg.report_nonce,
        vdev.attest_info,
    ) && cfg.vca_digest == pdev.vca_digest && cfg.meas_digest == vdev.meas_digest
        && cfg.report_digest == vdev.report_digest && Equal(cfg.state, vdev.vdev_state))) && old_s
        == new_s
}