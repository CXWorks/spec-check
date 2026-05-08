pub open spec fn RSI_VDEV_GET_INFO_spec(
    s: S,
    vdev_id: u64,
    addr: Address,
) -> (result: RsiCommandReturnCode, cfg: RsiVdevInfo)
{
    let realm = CurrentRealm();
    let vdev = VdevFromVdevId(s, realm, vdev_id);
    let pdev = PdevAt(s, vdev.pdev);
    let cfg = RsiVdevInfoAt(s, addr);
    let walk = RttWalk(s, realm, addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    
    // Failure conditions (ordered)
    if realm.feat_da != FEATURE_TRUE {
        (RSI_ERROR_STATE, cfg)
    } else if VdevIdIsFree(s, realm, vdev_id) {
        (RSI_ERROR_INPUT, cfg)
    } else if !AddrIsAligned(addr, 512) {
        (RSI_ERROR_INPUT, cfg)
    } else if !AddrIsProtected(addr, realm) {
        (RSI_ERROR_INPUT, cfg)
    } else if walk.rtte.ripas == EMPTY {
        (RSI_ERROR_INPUT, cfg)
    } else {
        // Success: all postconditions hold
        let result_cfg = RsiVdevInfo {
            hash_algo: pdev.hash_algo,
            flags: RmiVdevFlags {
                p2p_enabled: pdev.p2p_enabled,
                p2p_bound: vdev.p2p_bound,
                ..cfg.flags
            },
            p2p_peer: vdev.p2p_peer,
            lock_nonce: vdev.attest_info.lock_nonce,
            meas_nonce: vdev.attest_info.meas_nonce,
            report_nonce: vdev.attest_info.report_nonce,
            vca_digest: pdev.vca_digest,
            meas_digest: vdev.meas_digest,
            report_digest: vdev.report_digest,
            state: vdev.vdev_state,
            ..cfg
        };
        (RSI_SUCCESS, result_cfg)
    }
}