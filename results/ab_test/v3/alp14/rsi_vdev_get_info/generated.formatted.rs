pub open spec fn RSI_VDEV_GET_INFO_spec(
    old_s: S,
    new_s: S,
    vdev_id: u64,
    addr: Address,
    result: RsiCommandReturnCode,
) -> bool {
    let realm = CurrentRealm();
    let vdev = VdevFromVdevId(old_s, realm, vdev_id);
    let pdev = PdevAt(old_s, vdev.pdev);
    let cfg = RsiVdevInfoAt(old_s, addr);
    let walk = RttWalk(old_s, realm, addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);

    // Failure condition: da_en (highest priority)
    (realm.feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
        &&
    // Failure conditions: vdev_id, addr_align, addr_bound, addr_empty (lower priority)
    (realm.feat_da == FEATURE_TRUE ==> (
    // vdev_id
    (VdevIdIsFree(old_s, realm, vdev_id) ==> result == RSI_ERROR_INPUT)
        &&
    // addr_align
    (!AddrIsAligned(addr, 512) ==> result == RSI_ERROR_INPUT)
        &&
    // addr_bound
    (!AddrIsProtected(addr, realm) ==> result == RSI_ERROR_INPUT)
        &&
    // addr_empty
    (walk.rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
        &&
    // Success conditions
    ((!VdevIdIsFree(old_s, realm, vdev_id) && AddrIsAligned(addr, 512) && AddrIsProtected(
        addr,
        realm,
    ) && walk.rtte.ripas != EMPTY) ==> (result == RSI_SUCCESS && Equal(
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
        && cfg.report_digest == vdev.report_digest && Equal(cfg.state, vdev.vdev_state)))))
}