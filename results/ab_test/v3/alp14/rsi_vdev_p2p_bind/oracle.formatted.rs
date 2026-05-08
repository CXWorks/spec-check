pub open spec fn rsi_vdev_p2p_bind_spec(
    vdev_id_1: Bits64,
    lock_nonce_1: UInt64,
    meas_nonce_1: UInt64,
    report_nonce_1: UInt64,
    vdev_id_2: Bits64,
    lock_nonce_2: UInt64,
    meas_nonce_2: UInt64,
    report_nonce_2: UInt64,
    result: RsiCommandReturnCode,
    old_s: S,
    new_s: S,
) -> bool {
    (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE) && (VdevIdIsFree(
        old_s,
        CurrentRealm(old_s),
        vdev_id_1,
    ) ==> result == RSI_ERROR_INPUT) && (VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id_2)
        ==> result == RSI_ERROR_INPUT) && ((PdevAt(
        old_s,
        VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id_1).pdev,
    ).p2p_stream_valid == RMM_FALSE || PdevAt(
        old_s,
        VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id_2).pdev,
    ).p2p_stream_valid == RMM_FALSE) ==> result == RSI_ERROR_INPUT) && (PdevAt(
        old_s,
        VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id_1).pdev,
    ).p2p_stream != PdevAt(
        old_s,
        VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id_2).pdev,
    ).p2p_stream ==> result == RSI_ERROR_INPUT) && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE)
        && !(VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id_1)) && !(VdevIdIsFree(
        old_s,
        CurrentRealm(old_s),
        vdev_id_2,
    )) && !((PdevAt(
        old_s,
        VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id_1).pdev,
    ).p2p_stream_valid == RMM_FALSE || PdevAt(
        old_s,
        VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id_2).pdev,
    ).p2p_stream_valid == RMM_FALSE)) && !(PdevAt(
        old_s,
        VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id_1).pdev,
    ).p2p_stream != PdevAt(
        old_s,
        VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id_2).pdev,
    ).p2p_stream)) ==> result == RSI_SUCCESS)
}