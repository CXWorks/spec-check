pub open spec fn rsi_vdev_validate_mapping_spec(vdev_id: UInt64, ipa_base: UInt64, ipa_top: UInt64, pa_base: UInt64, flags: UInt64, lock_nonce: UInt64, meas_nonce: UInt64, report_nonce: UInt64, result: RsiCommandReturnCode, new_ipa_base: UInt64, response: UInt, old_s: S, new_s: S) -> bool {
  (result == RSI_ERROR_STATE ==> new_ipa_base == VdevAt(old_s, vdev_id).ipa_base)
  && (result == RSI_ERROR_STATE ==> response == 0)
  && (result == RSI_ERROR_INPUT ==> new_ipa_base == VdevAt(old_s, vdev_id).ipa_base)
  && (result == RSI_ERROR_INPUT ==> response == 0)
  && (result == RSI_ERROR_INPUT ==> new_ipa_base == VdevAt(old_s, vdev_id).ipa_base)
  && (result == RSI_ERROR_INPUT ==> response == 0)
  && (result == RSI_ERROR_INPUT ==> new_ipa_base == VdevAt(old_s, vdev_id).ipa_base)
  && (result == RSI_ERROR_INPUT ==> response == 0)
  && (result == RSI_ERROR_INPUT ==> new_ipa_base == VdevAt(old_s, vdev_id).ipa_base)
  && (result == RSI_ERROR_INPUT ==> response == 0)
  && (result == RSI_ERROR_DEVICE ==> new_ipa_base == VdevAt(old_s, vdev_id).ipa_base)
  && (result == RSI_ERROR_DEVICE ==> response == 0)
  && ((!(DeviceAssignmentEnabled(old_s)) ==> result == RSI_ERROR_STATE)
    && (VdevAt(old_s, vdev_id).in_use == false ==> result == RSI_ERROR_INPUT)
    && (VdevAt(old_s, vdev_id).state != VDEV_LOCKED && VdevAt(old_s, vdev_id).state != VDEV_STARTED ==> result == RSI_ERROR_INPUT)
    && (ipa_base % GRANULE_SIZE == 0 ==> result != RSI_ERROR_INPUT)
    && (ipa_top % GRANULE_SIZE == 0 ==> result != RSI_ERROR_INPUT)
    && (pa_base % GRANULE_SIZE == 0 ==> result != RSI_ERROR_INPUT)
    && (ipa_top <= ipa_base ==> result == RSI_ERROR_INPUT)
    && (!(ipa_base..=ipa_top) subset_of (RealmAt(old_s, VdevAt(old_s, vdev_id).realm).protected_ipa_space) ==> result == RSI_ERROR_INPUT)
    && (VdevAt(old_s, vdev_id).lock_nonce != lock_nonce || VdevAt(old_s, vdev_id).meas_nonce != meas_nonce || VdevAt(old_s, vdev_id).report_nonce != report_nonce ==> result == RSI_ERROR_DEVICE))
    ==> result == RSI_SUCCESS)
  && (result == RSI_SUCCESS ==> new_ipa_base == VdevAt(new_s, vdev_id).ipa_base)
  && (result == RSI_SUCCESS ==> response == VdevAt(new_s, vdev_id).response as int)
  && ((!(DeviceAssignmentEnabled(old_s)) &&
       VdevAt(old_s, vdev_id).in_use &&
       !(VdevAt(old_s, vdev_id).state != VDEV_LOCKED && VdevAt(old_s, vdev_id).state != VDEV_STARTED) &&
       (ipa_base % GRANULE_SIZE != 0) &&
       (ipa_top % GRANULE_SIZE != 0) &&
       (ipa_base % GRANULE_SIZE != 0) &&
       !(ipa_top <= ipa_base) &&
       (-(ipa_base..=ipa_top) subset_of (RealmAt(old_s, VdevAt(old_s, vdev_id).realm).protected_ipa_space)) &&
       !(VdevAt(old_s, vdev_id).lock_nonce != lock_nonce || VdevAt(old_s, vdev_id).meas_nonce != meas_nonce || VdevAt(old_s, vdev_id).report_nonce != report_nonce))
    ==> result != RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> VdevAt(new_s, vdev_id).ipa_base == VdevAt(old_s, vdev_id).ipa_base)
  && (result != RSI_SUCCESS
    ==> VdevAt(new_s, vdev_id).ipa_top == VdevAt(old_s, vdev_id).ipa_top)
  && (result != RSI_SUCCESS
    ==> VdevAt(new_s, vdev_id).pa_base == VdevAt(old_s, vdev_id).pa_base)
  && (result != RSI_SUCCESS
    ==> VdevAt(new_s, vdev_id).response == VdevAt(old_s, vdev_id).response)
  && (result == RSI_SUCCESS
    ==> VdevAt(new_s, vdev_id).response == VdevAt(old_s, vdev_id).response)
  && (VdevAt(new_s, vdev_id).ipa_base == VdevAt(old_s, vdev_id).ipa_base &&
       VdevAt(new_s, vdev_id).ipa_top == VdevAt(old_s, vdev_id).ipa_top &&
       VdevAt(new_s, vdev_id).pa_base == VdevAt(old_s, vdev_id).pa_base &&
       VdevAt(new_s, vdev_id).response == VdevAt(old_s, vdev_id).response)
  ==> result == RSI_SUCCESS)
  && (result == RSI_SUCCESS
    ==> VdevAt(new_s, vdev_id).ipa_base == new_ipa_base)
  && (result == RSI_SUCCESS
    ==> VdevAt(new_s, vdev_id).response == response)
}