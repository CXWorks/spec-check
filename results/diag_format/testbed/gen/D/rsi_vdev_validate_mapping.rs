pub open spec fn rsi_vdev_validate_mapping_spec(vdev_id: Bits64, ipa_base: Address, ipa_top: Address, pa_base: Address, flags: RsiDevMemFlags, lock_nonce: UInt64, meas_nonce: UInt64, report_nonce: UInt64, result: RsiCommandReturnCode, new_ipa_base: Address, response: RsiResponse, old_s: S, new_s: S) -> bool {
    (!ImplFeatures(old_s).feat_da ==> ResultEqual(result, RSI_ERROR_STATE))
    && (!VdevAt(old_s, vdev_id).vdev_state == VDEV_LOCKED || !VdevAt(old_s, vdev_id).vdev_state == VDEV_STARTED ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, ipa_base) ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, ipa_top) ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, pa_base) ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (ipa_top <= ipa_base ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (!AddrRangeIsProtected(old_s, ipa_base, ipa_top, old_s.CurrentRealm()) ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (!VdevAt(old_s, vdev_id).attest_info.lock_nonce == lock_nonce || !VdevAt(old_s, vdev_id).attest_info.meas_nonce == meas_nonce || !VdevAt(old_s, vdev_id).attest_info.report_nonce == report_nonce ==> ResultEqual(result, RSI_ERROR_DEVICE))
    && (result == RSI_SUCCESS ==> (new_ipa_base == old_s.CurrentRec().dev_mem_addr && response == RecDevMemResponseToRsi(old_s.CurrentRec())))
}