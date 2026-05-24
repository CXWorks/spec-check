pub open spec fn rsi_vdev_p2p_bind_spec(result: RsiCommandReturnCode, old_s: S, new_s: S, vdev_id_1: u64, lock_nonce_1: u64, meas_nonce_1: u64, report_nonce_1: u64, vdev_id_2: u64, lock_nonce_2: u64, meas_nonce_2: u64, report_nonce_2: u64) -> bool {
    let realm = CurrentRealm(old_s);
    let vdev_1 = VdevFromVdevId(old_s, realm, vdev_id_1);
    let pdev_1 = PdevAt(old_s, vdev_1.pdev);
    let vdev_2 = VdevFromVdevId(old_s, realm, vdev_id_2);
    let pdev_2 = PdevAt(old_s, vdev_2.pdev);
    (realm.feat_da != FEATURE_TRUE ==> ResultEqual(result, RSI_ERROR_STATE))
    && (VdevIdIsFree(old_s, realm, vdev_id_1) ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (VdevIdIsFree(old_s, realm, vdev_id_2) ==> ResultEqual(result, RSI_ERROR_INPUT))
    && ((pdev_1.p2p_stream_valid == RMM_FALSE || pdev_2.p2p_stream_valid == RMM_FALSE) ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (pdev_1.p2p_stream != pdev_2.p2p_stream ==> ResultEqual(result, RSI_ERROR_INPUT))
    && ((realm.feat_da == FEATURE_TRUE
        && !VdevIdIsFree(old_s, realm, vdev_id_1)
        && !VdevIdIsFree(old_s, realm, vdev_id_2)
        && pdev_1.p2p_stream_valid == RMM_TRUE
        && pdev_2.p2p_stream_valid == RMM_TRUE
        && pdev_1.p2p_stream == pdev_2.p2p_stream)
        ==> result.is_Ok())
}