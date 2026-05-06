pub open spec fn rsi_vdev_dma_enable_spec(
    result: RsiCommandReturnCode,
    old_s: S,
    new_s: S,
    vdev_id: u64,
    flags: RsiVdevDmaFlags,
    lock_nonce: u64,
    meas_nonce: u64,
    report_nonce: u64,
) -> bool {
    let realm = CurrentRealm(old_s);
    let vdev = VdevFromVdevId(old_s, realm, vdev_id);

    // Failure condition: da_en - realm.feat_da != FEATURE_TRUE
    (realm.feat_da != FEATURE_TRUE ==> result
        == RSI_ERROR_STATE)
    // Failure condition: vdev_id - VdevIdIsFree
     && (VdevIdIsFree(old_s, realm, vdev_id) ==> result
        == RSI_ERROR_INPUT)
    // Failure condition: attest_info - nonce mismatch
     && (!VdevAttestInfoEqual(
        lock_nonce as int,
        meas_nonce as int,
        report_nonce as int,
        vdev.attest_info,
    ) ==> result == RSI_ERROR_DEVICE)
    // Success condition: dma_state
     && (realm.feat_da == FEATURE_TRUE && !VdevIdIsFree(old_s, realm, vdev_id)
        && VdevAttestInfoEqual(
        lock_nonce as int,
        meas_nonce as int,
        report_nonce as int,
        vdev.attest_info,
    ) ==> (result == RSI_SUCCESS && VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).dma_state
        == VDEV_DMA_ENABLED))
}