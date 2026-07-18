pub open spec fn rsi_vdev_validate_mapping_spec(vdev_id: Bits64, ipa_base: Address, ipa_top: Address, pa_base: Address, flags: RsiDevMemFlags, lock_nonce: UInt64, meas_nonce: UInt64, report_nonce: UInt64, result: RsiCommandReturnCode, new_ipa_base: Address, response: RsiResponse, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id) ==> result == RSI_ERROR_INPUT)
  && ((VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state != VDEV_LOCKED && VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state != VDEV_STARTED) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, ipa_base) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, ipa_top) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, pa_base) ==> result == RSI_ERROR_INPUT)
  && ((ipa_top) <= (ipa_base) ==> result == RSI_ERROR_INPUT)
  && (!AddrRangeIsProtected(old_s, ipa_base, ipa_top, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (!VdevAttestInfoEqual(old_s, lock_nonce, meas_nonce, report_nonce, VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).attest_info) ==> result == RSI_ERROR_DEVICE)
  && (result == RSI_SUCCESS ==> new_ipa_base == CurrentRec(old_s).dev_mem_addr)
  && (result == RSI_SUCCESS ==> response == RecDevMemResponseToRsi(old_s, CurrentRec(old_s)))
  && ((!CurrentRealm(old_s).feat_da != FEATURE_TRUE && !(VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id) && (VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state != VDEV_LOCKED && VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state != VDEV_STARTED) && !AddrIsGranuleAligned(old_s, ipa_base) && !AddrIsGranuleAligned(old_s, ipa_top) && !AddrIsGranuleAligned(old_s, pa_base) && ((ipa_top) <= (ipa_base)) && !AddrRangeIsProtected(old_s, ipa_base, ipa_top, CurrentRealm(old_s)) && !VdevAttestInfoEqual(old_s, lock_nonce, meas_nonce, report_nonce, VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).attest_info)) ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> new_ipa_base == new_ipa_base)
  && (result != RSI_SUCCESS
    ==> response == response)
}