pub open spec fn rsi_vdev_validate_mapping_spec(vdev_id: Bits64, ipa_base: Address, ipa_top: Address, pa_base: Address, flags: RsiDevMemFlags, lock_nonce: UInt64, meas_nonce: UInt64, report_nonce: UInt64, result: RsiCommandReturnCode, new_ipa_base: Address, response: RsiResponse, old_s: S, new_s: S) -> bool {
  (!CurrentRealm(old_s).feat_da ==> result == RSI_ERROR_STATE)
  && (!VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state == VDEV_LOCKED && !VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state == VDEV_STARTED ==> result == RSI_ERROR_INPUT)
  && (VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state != VDEV_LOCKED && VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state != VDEV_STARTED ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, CurrentRealm(old_s), ipa_base) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, CurrentRealm(old_s), ipa_top) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, CurrentRealm(old_s), pa_base) ==> result == RSI_ERROR_INPUT)
  && (ipa_top <= ipa_base ==> result == RSI_ERROR_INPUT)
  && (!(ipa_base > CurrentRealm(old_s).rtt_base[0] && ipa_top < CurrentRealm(old_s).rtt_base[0] + pow2(10)) ==> result == RSI_ERROR_INPUT)
  && (VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).attest_info.lock_nonce != lock_nonce || VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).attest_info.meas_nonce != meas_nonce || VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).attest_info.report_nonce != report_nonce ==> result == RSI_ERROR_DEVICE)
  && (result == RSI_SUCCESS ==> response == RecDevMemResponseToRsi(new_s, CurrentRec(new_s)))
  && (result == RSI_SUCCESS ==> new_ipa_base == RecDevMemAddr(new_s, CurrentRec(new_s)))
  && ((!(CurrentRealm(old_s).feat_da) &&
       !(VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state == VDEV_LOCKED && !VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state == VDEV_STARTED) &&
       !(VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state != VDEV_LOCKED && VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state != VDEV_STARTED) &&
       AddrIsGranuleAligned(old_s, CurrentRealm(old_s), ipa_base) &&
       AddrIsGranuleAligned(old_s, CurrentRealm(old_s), ipa_top) &&
       AddrIsGranuleAligned(old_s, CurrentRealm(old_s), pa_base) &&
       !(ipa_top <= ipa_base) &&
       (ipa_base > CurrentRealm(old_s).rtt_base[0] && ipa_top < CurrentRealm(old_s).rtt_base[0] + pow2(10)))
    ==> result == RSI_SUCCESS)
}