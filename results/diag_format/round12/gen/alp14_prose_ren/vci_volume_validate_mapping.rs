pub open spec fn vci_volume_validate_mapping_spec(volume_id: Bits64, lba_base: Address, lba_top: Address, dpa_base: Address, flags: Vcidevmemflags, lock_nonce: UInt64, meas_nonce: UInt64, report_nonce: UInt64, result: Vcicommandreturncode, new_lba_base: Address, response: Vciresponse, old_s: S, new_s: S) -> bool {
  (DeviceAssignmentEnabled(old_s) ==> result == VCI_SUCCESS)
  (Volumefromvolumeid(old_s, Currentvault(old_s), volume_id).in_use ==> result == VCI_SUCCESS)
  (Volumefromvolumeid(old_s, Currentvault(old_s), volume_id).state == VOLUME_LOCKED || Volumefromvolumeid(old_s, Currentvault(old_s), volume_id).state == VOLUME_STARTED ==> result == VCI_SUCCESS)
  (lba_base % ExtentSize(old_s) == 0 ==> result == VCI_SUCCESS)
  (lba_top % ExtentSize(old_s) == 0 ==> result == VCI_SUCCESS)
  (dpa_base % ExtentSize(old_s) == 0 ==> result == VCI_SUCCESS)
  (lba_top > lba_base ==> result == VCI_SUCCESS)
  (LBAInRange(old_s, lba_base, lba_top, Currentvault(old_s)) ==> result == VCI_SUCCESS)
  (lock_nonce == lock_nonce && meas_nonce == meas_nonce && report_nonce == report_nonce ==> result == VCI_SUCCESS)
  ((!(DeviceAssignmentEnabled(old_s)) ||
       !(Volumefromvolumeid(old_s, Currentvault(old_s), volume_id).in_use))
    ==> result == VCI_ERROR_STATE)
  (!(Volumefromvolumeid(old_s, Currentvault(old_s), volume_id).state == VOLUME_LOCKED || Volumefromvolumeid(old_s, Currentvault(old_s), volume_id).state == VOLUME_STARTED)
    ==> result == VCI_ERROR_INPUT)
  (lba_base % ExtentSize(old_s) != 0
    ==> result == VCI_ERROR_INPUT)
  (lba_top % ExtentSize(old_s) != 0
    ==> result == VCI_ERROR_INPUT)
  (dpa_base % ExtentSize(old_s) != 0
    ==> result == VCI_ERROR_INPUT)
  (lba_top <= lba_base
    ==> result == VCI_ERROR_INPUT)
  (!LBAInRange(old_s, lba_base, lba_top, Currentvault(old_s))
    ==> result == VCI_ERROR_INPUT)
  ((lock_nonce != lock_nonce || meas_nonce != meas_nonce || report_nonce != report_nonce)
    ==> result == VCI_ERROR_DEVICE)
  (result == VCI_SUCCESS ==> new_lba_base == Currentworker(new_s).device_memory_address)
  (result == VCI_SUCCESS ==> response == Currentworker(new_s).device_memory_response)
  ((!(DeviceAssignmentEnabled(old_s)) &&
       Volumefromvolumeid(old_s, Currentvault(old_s), volume_id).in_use)
    ==> result == VCI_SUCCESS)
  ((Volumefromvolumeid(old_s, Currentvault(old_s), volume_id).state == VOLUME_LOCKED || Volumefromvolumeid(old_s, Currentvault(old_s), volume_id).state == VOLUME_STARTED)
    ==> result == VCI_SUCCESS)
  (lba_base % ExtentSize(old_s) == 0 &&
   lba_top % ExtentSize(old_s) == 0 &&
   dpa_base % ExtentSize(old_s) == 0 &&
   lba_top > lba_base &&
   LBAInRange(old_s, lba_base, lba_top, Currentvault(old_s)) &&
   lock_nonce == lock_nonce &&
   meas_nonce == meas_nonce &&
   report_nonce == report_nonce)
    ==> result == VCI_SUCCESS)
  (result != VCI_SUCCESS
    ==> new_lba_base == new_lba_base)
  (result != VCI_SUCCESS
    ==> response == response)
}