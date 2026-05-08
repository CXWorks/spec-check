pub open spec fn RSI_VDEV_P2P_BIND_spec(
    s: S,
    vdev_id_1: u64,
    lock_nonce_1: u64,
    meas_nonce_1: u64,
    report_nonce_1: u64,
    vdev_id_2: u64,
    lock_nonce_2: u64,
    meas_nonce_2: u64,
    report_nonce_2: u64,
    result: RsiCommandReturnCode,
) -> bool {
    let realm = CurrentRealm(s);
    let vdev_1 = VdevFromVdevId(s, realm, vdev_id_1);
    let pdev_1 = PdevAt(s, vdev_1.pdev);
    let vdev_2 = VdevFromVdevId(s, realm, vdev_id_2);
    let pdev_2 = PdevAt(s, vdev_2.pdev);

    // da_en check: must be first in failure ordering
    if realm.feat_da != FEATURE_TRUE {
        result == RSI_ERROR_STATE
    } else if VdevIdIsFree(s, realm, vdev_id_1) {
        result == RSI_ERROR_INPUT
    } else if VdevIdIsFree(s, realm, vdev_id_2) {
        result == RSI_ERROR_INPUT
    } else if pdev_1.p2p_stream_valid == RMM_FALSE || pdev_2.p2p_stream_valid == RMM_FALSE {
        result == RSI_ERROR_INPUT
    } else if pdev_1.p2p_stream != pdev_2.p2p_stream {
        result == RSI_ERROR_INPUT
    } else {
        // Success case - no changes to state
        true
    }
}