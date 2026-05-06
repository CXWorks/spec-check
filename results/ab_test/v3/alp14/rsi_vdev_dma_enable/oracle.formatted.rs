pub open spec fn rsi_vdev_dma_enable_spec(
    vdev_id: Bits64,
    flags: RsiVdevDmaFlags,
    lock_nonce: UInt64,
    meas_nonce: UInt64,
    report_nonce: UInt64,
    result: RsiCommandReturnCode,
    old_s: S,
    new_s: S,
) -> bool {
    (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE) && (VdevIdIsFree(
        old_s,
        CurrentRealm(old_s),
        vdev_id,
    ) ==> result == RSI_ERROR_INPUT) && (!VdevAttestInfoEqual(
        lock_nonce,
        meas_nonce,
        report_nonce,
        VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).attest_info,
    ) ==> result == RSI_ERROR_DEVICE) && (result == RSI_SUCCESS ==> VdevFromVdevId(
        new_s,
        CurrentRealm(new_s),
        vdev_id,
    ).dma_state == VDEV_DMA_ENABLED) && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) && !(
    VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id)) && VdevAttestInfoEqual(
        lock_nonce,
        meas_nonce,
        report_nonce,
        VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).attest_info,
    )) ==> result == RSI_SUCCESS) && (result != RSI_SUCCESS ==> VdevFromVdevId(
        new_s,
        CurrentRealm(new_s),
        vdev_id,
    ).dma_state == VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).dma_state)
}