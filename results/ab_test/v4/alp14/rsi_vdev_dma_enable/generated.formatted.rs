pub open spec fn RSI_VDEV_DMA_ENABLE_spec(
    old_s: S,
    new_s: S,
    vdev_id: Bits64,
    flags: RsiVdevDmaFlags,
    lock_nonce: u64,
    meas_nonce: u64,
    report_nonce: u64,
    result: RsiCommandReturnCode,
) -> bool {
    let realm = CurrentRealm(old_s);
    let vdev = VdevFromVdevId(old_s, realm, vdev_id);

    (realm.feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE) && (VdevIdIsFree(
        old_s,
        realm,
        vdev_id,
    ) ==> result == RSI_ERROR_INPUT) && (!VdevAttestInfoEqual(
        lock_nonce as int,
        meas_nonce as int,
        report_nonce as int,
        vdev.attest_info,
    ) ==> result == RSI_ERROR_DEVICE) && ((realm.feat_da == FEATURE_TRUE && !VdevIdIsFree(
        old_s,
        realm,
        vdev_id,
    ) && VdevAttestInfoEqual(
        lock_nonce as int,
        meas_nonce as int,
        report_nonce as int,
        vdev.attest_info,
    )) ==> (result == RSI_SUCCESS && vdev.dma_state == VDEV_DMA_ENABLED))
}