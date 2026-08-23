pub open spec fn rsi_vdev_get_info_spec(vdev_id: Bits64, addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
    && (CurrentRealm(old_s).feat_da == FEATURE_TRUE
        && VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id)
        ==> result == RSI_ERROR_INPUT)
    && (CurrentRealm(old_s).feat_da == FEATURE_TRUE
        && !AddrIsAligned(old_s, addr, 512)
        ==> result == RSI_ERROR_INPUT)
    && (CurrentRealm(old_s).feat_da == FEATURE_TRUE
        && !AddrIsProtected(old_s, addr, CurrentRealm(old_s))
        ==> result == RSI_ERROR_INPUT)
    && (CurrentRealm(old_s).feat_da == FEATURE_TRUE
        && RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas == EMPTY
        ==> result == RSI_ERROR_INPUT)
    && (CurrentRealm(old_s).feat_da == FEATURE_TRUE
        && !VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id)
        && AddrIsAligned(old_s, addr, 512)
        && AddrIsProtected(old_s, addr, CurrentRealm(old_s))
        && RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas != EMPTY
        ==> result == RSI_SUCCESS
            && Equal(RsiVdevInfoAt(new_s, addr).hash_algo, PdevAt(old_s, VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).pdev).hash_algo)
            && Equal(RsiVdevInfoAt(new_s, addr).flags.p2p_enabled, PdevAt(old_s, VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).pdev).p2p_enabled)
            && Equal(RsiVdevInfoAt(new_s, addr).flags.p2p_bound, VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).p2p_bound)
            && RsiVdevInfoAt(new_s, addr).p2p_peer == VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).p2p_peer
            && VdevAttestInfoEqual(
                   RsiVdevInfoAt(new_s, addr).lock_nonce as int,
                   RsiVdevInfoAt(new_s, addr).meas_nonce as int,
                   RsiVdevInfoAt(new_s, addr).report_nonce as int,
                   VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).attest_info)
            && RsiVdevInfoAt(new_s, addr).vca_digest == PdevAt(old_s, VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).pdev).vca_digest
            && RsiVdevInfoAt(new_s, addr).meas_digest == VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).meas_digest
            && RsiVdevInfoAt(new_s, addr).report_digest == VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).report_digest
            && Equal(RsiVdevInfoAt(new_s, addr).state, VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state))
}