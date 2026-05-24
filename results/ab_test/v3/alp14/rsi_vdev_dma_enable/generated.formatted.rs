pub open spec fn rsi_vdev_dma_enable_spec(result: RsiCommandReturnCode, vdev_id: u64, lock_nonce: u64, meas_nonce: u64, report_nonce: u64, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    let vdev = VdevFromVdevId(old_s, realm, vdev_id);
    
    (!old_s.realm.feat_da ==> result == RSI_ERROR_STATE)
    && (VdevIdIsFree(old_s, realm, vdev_id) ==> result == RSI_ERROR_INPUT)
    && (!VdevAttestInfoEqual(lock_nonce, meas_nonce, report_nonce, vdev.attest_info) ==> result == RSI_ERROR_DEVICE)
    && ((old_s.realm.feat_da && !VdevIdIsFree(old_s, realm, vdev_id) && VdevAttestInfoEqual(lock_nonce, meas_nonce, report_nonce, vdev.attest_info)) ==> (result.is_Ok() && new_s.vdev.dma_state == VDEV_DMA_ENABLED))
}