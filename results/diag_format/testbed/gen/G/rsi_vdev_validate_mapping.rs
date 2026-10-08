pub open spec fn rsi_vdev_validate_mapping_spec(vdev_id: Bits64, ipa_base: Address, ipa_top: Address, pa_base: Address, flags: RsiDevMemFlags, lock_nonce: UInt64, meas_nonce: UInt64, report_nonce: UInt64, result: RsiCommandReturnCode, new_ipa_base: Address, response: RsiResponse, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da == RMM_FEATURE_FALSE ==> result == RSI_ERROR_STATE)
  && (VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state != VDEV_LOCKED && VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state != VDEV_STARTED ==> result == RSI_ERROR_INPUT)
  && ((ipa_base) % RMM_GRANULE_SIZE != 0 ==> result == RSI_ERROR_INPUT)
  && ((ipa_top) % RMM_GRANULE_SIZE != 0 ==> result == RSI_ERROR_INPUT)
  && ((pa_base) % RMM_GRANULE_SIZE != 0 ==> result == RSI_ERROR_INPUT)
  && (ipa_top <= ipa_base ==> result == RSI_ERROR_INPUT)
  && (!AddrIsWithin(old_s, ipa_base, CurrentRealm(old_s).rtt_base[0], CurrentRealm(old_s).rtt_base[3]) &&
       !AddrIsWithin(old_s, ipa_base, CurrentRealm(old_s).rtt_base[1], CurrentRealm(old_s).rtt_base[3]) &&
       !AddrIsWithin(old_s, ipa_base, CurrentRealm(old_s).rtt_base[2], CurrentRealm(old_s).rtt_base[3]) &&
       !AddrIsWithin(old_s, ipa_base, CurrentRealm(old_s).rtt_base[3], CurrentRealm(old_s).rtt_base[3]))
    ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> new_ipa_base == CurrentRec(new_s).dev_mem_addr)
  && (result == RSI_SUCCESS ==> response == RecDevMemResponseToRsi(new_s, CurrentRec(new_s)))
  && ((!(CurrentRealm(old_s).feat_da == RMM_FEATURE_FALSE) &&
       !(VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state != VDEV_LOCKED && VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state != VDEV_STARTED) &&
       !((ipa_base) % RMM_GRANULE_SIZE != 0) &&
       !((ipa_top) % RMM_GRANULE_SIZE != 0) &&
       !((pa_base) % RMM_GRANULE_SIZE != 0) &&
       !(ipa_top <= ipa_base) &&
       (AddrIsWithin(old_s, ipa_base, CurrentRealm(old_s).rtt_base[0], CurrentRealm(old_s).rtt_base[3]) ||
        AddrIsWithin(old_s, ipa_base, CurrentRealm(old_s).rtt_base[1], CurrentRealm(old_s).rtt_base[3]) ||
        AddrIsWithin(old_s, ipa_base, CurrentRealm(old_s).rtt_base[2], CurrentRealm(old_s).rtt_base[3]) ||
        AddrIsWithin(old_s, ipa_base, CurrentRealm(old_s).rtt_base[3], CurrentRealm(old_s).rtt_base[3])))
    ==> result == RSI_SUCCESS)
}