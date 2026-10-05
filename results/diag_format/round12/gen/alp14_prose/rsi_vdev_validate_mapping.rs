pub open spec fn rsi_vdev_validate_mapping_spec(vdev_id: Bits64, ipa_base: Address, ipa_top: Address, pa_base: Address, flags: RsiDevMemFlags, lock_nonce: UInt64, meas_nonce: UInt64, report_nonce: UInt64, result: RsiCommandReturnCode, new_ipa_base: Address, response: RsiResponse, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).device_assignment == FEATURE_FALSE ==> result == RSI_ERROR_STATE)
  && (VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).in_use == false ==> result == RSI_ERROR_INPUT)
  && (VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).state != VDEV_LOCKED && VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).state != VDEV_STARTED ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, ipa_base, GRANULE_SIZE as int) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, ipa_top, GRANULE_SIZE as int) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, pa_base, GRANULE_SIZE as int) ==> result == RSI_ERROR_INPUT)
  && (ipa_top <= ipa_base ==> result == RSI_ERROR_INPUT)
  && (!IsProtectedIpaRange(old_s, CurrentRealm(old_s), ipa_base, ipa_top) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_ERROR_DEVICE ==> result == RSI_ERROR_DEVICE)
  && (result == RSI_SUCCESS ==> new_ipa_base == CurrentRec(new_s).device_memory_address)
  && (result == RSI_SUCCESS ==> response == CurrentRec(new_s).device_memory_response as RsiResponse)
  && ((!(CurrentRealm(old_s).device_assignment == FEATURE_FALSE) &&
       !(VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).in_use == false) &&
       !(VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).state != VDEV_LOCKED && VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).state != VDEV_STARTED) &&
       AddrIsGranuleAligned(old_s, ipa_base, GRANULE_SIZE as int) &&
       AddrIsGranuleAligned(old_s, ipa_top, GRANULE_SIZE as int) &&
       AddrIsGranuleAligned(old_s, pa_base, GRANULE_SIZE as int) &&
       !(ipa_top <= ipa_base) &&
       IsProtectedIpaRange(old_s, CurrentRealm(old_s), ipa_base, ipa_top))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> new_ipa_base == CurrentRec(new_s).device_memory_address)
  && (result != RSI_SUCCESS
    ==> response == CurrentRec(new_s).device_memory_response as RsiResponse)
}