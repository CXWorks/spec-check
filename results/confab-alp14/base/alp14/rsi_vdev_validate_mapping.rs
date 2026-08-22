pub open spec fn rsi_vdev_validate_mapping_spec(result: RsiCommandReturnCode, new_ipa_base: Address, response: RsiResponse, vdev_id: u64, ipa_base: Address, ipa_top: Address, pa_base: Address, flags: RsiDevMemFlags, lock_nonce: u64, meas_nonce: u64, report_nonce: u64, old_s: S, new_s: S) -> bool {
    (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
    && (VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id) ==> result == RSI_ERROR_INPUT)
    && ((VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state != VDEV_LOCKED
            && VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state != VDEV_STARTED)
        ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(old_s, ipa_base) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(old_s, ipa_top) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(old_s, pa_base) ==> result == RSI_ERROR_INPUT)
    && ((ipa_top as int) <= (ipa_base as int) ==> result == RSI_ERROR_INPUT)
    && (!AddrRangeIsProtected(old_s, ipa_base, ipa_top, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
    && (!VdevAttestInfoEqual(lock_nonce as int, meas_nonce as int, report_nonce as int, VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).attest_info)
        ==> result == RSI_ERROR_DEVICE)
    && ((CurrentRealm(old_s).feat_da == FEATURE_TRUE
            && !VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id)
            && (VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state == VDEV_LOCKED
                || VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state == VDEV_STARTED)
            && AddrIsGranuleAligned(old_s, ipa_base)
            && AddrIsGranuleAligned(old_s, ipa_top)
            && AddrIsGranuleAligned(old_s, pa_base)
            && (ipa_top as int) > (ipa_base as int)
            && AddrRangeIsProtected(old_s, ipa_base, ipa_top, CurrentRealm(old_s))
            && VdevAttestInfoEqual(lock_nonce as int, meas_nonce as int, report_nonce as int, VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).attest_info))
        ==> (result == RSI_SUCCESS
            && new_ipa_base == CurrentRec(old_s).dev_mem_addr
            && response == RecDevMemResponseToRsi(old_s, CurrentRec(old_s))))
    && new_s == old_s
}