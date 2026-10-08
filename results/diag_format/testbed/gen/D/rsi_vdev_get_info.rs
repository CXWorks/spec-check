pub open spec fn rsi_vdev_get_info_spec(vdev_id: Bits64, addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    let vdev = VdevFromVdevId(old_s, realm, vdev_id);
    let pdev = PdevAt(old_s, vdev.pdev);
    let cfg = RsiVdevInfoAt(old_s, addr);
    let walk = RttWalk(old_s, realm, addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int);
    (!PdevFlags(old_s, pdev).p2p_enabled ==> result == RSI_ERROR_STATE)
    && (vdev.is_none() ==> result == RSI_ERROR_INPUT)
    && (!AddrIsAligned(old_s, addr, 512) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsProtected(old_s, addr, realm) ==> result == RSI_ERROR_INPUT)
    && (GranuleAt(old_s, addr).state == RmmGranuleState::EMPTY ==> result == RSI_ERROR_INPUT)
    && (result == RSI_SUCCESS ==> cfg.hash_algo == PdevHashAlgorithm(old_s, pdev))
    && (result == RSI_SUCCESS ==> cfg.p2p_enabled == PdevFlags(old_s, pdev).p2p_enabled)
    && (result == RSI_SUCCESS ==> cfg.p2p_bound == VdevAt(old_s, addr).p2p_bound)
    && (result == RSI_SUCCESS ==> cfg.p2p_peer == VdevAt(old_s, addr).p2p_peer)
    && (result == RSI_SUCCESS ==> cfg.lock_nonce == VdevAt(old_s, addr).lock_nonce)
    && (result == RSI_SUCCESS ==> cfg.meas_nonce == VdevAt(old_s, addr).meas_nonce)
    && (result == RSI_SUCCESS ==> cfg.report_nonce == VdevAt(old_s, addr).report_nonce)
    && (result == RSI_SUCCESS ==> cfg.vca_digest == PdevVcaDigest(old_s, pdev))
    && (result == RSI_SUCCESS ==> cfg.meas_digest == VdevMeasDigest(old_s, vdev))
    && (result == RSI_SUCCESS ==> cfg.report_digest == VdevReportDigest(old_s, vdev))
    && (result == RSI_SUCCESS ==> cfg.state == VdevState(old_s, vdev))
}