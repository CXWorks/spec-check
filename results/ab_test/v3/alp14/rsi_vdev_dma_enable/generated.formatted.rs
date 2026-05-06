pub open spec fn RSI_VDEV_DMA_ENABLE_spec(
    old_s: S,
    new_s: S,
    vdev_id: u64,
    flags: RsiVdevDmaFlags,
    lock_nonce: u64,
    meas_nonce: u64,
    report_nonce: u64,
    result: RsiCommandReturnCode,
) -> bool {
    let realm = CurrentRealm(old_s);
    let vdev = VdevFromVdevId(old_s, realm, vdev_id);

    // Failure condition: da_en - realm.feat_da != FEATURE_TRUE
    (realm.feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
        &&
    // Failure condition: vdev_id - VdevIdIsFree(realm, vdev_id)
    (VdevIdIsFree(old_s, realm, vdev_id) ==> result == RSI_ERROR_INPUT)
        &&
    // Failure condition: attest_info - nonces don't match
    (!VdevAttestInfoEqual(
        lock_nonce as int,
        meas_nonce as int,
        report_nonce as int,
        vdev.attest_info,
    ) ==> result == RSI_ERROR_DEVICE) &&
    // Success condition: dma_state
    (result == RSI_SUCCESS ==> realm.feat_da == FEATURE_TRUE && !VdevIdIsFree(old_s, realm, vdev_id)
        && VdevAttestInfoEqual(
        lock_nonce as int,
        meas_nonce as int,
        report_nonce as int,
        vdev.attest_info,
    ) && VdevFromVdevId(new_s, realm, vdev_id).dma_state == VDEV_DMA_ENABLED)
        &&
    // No footprint - realm state unchanged
    (new_s == old_s || result != RSI_SUCCESS)
}