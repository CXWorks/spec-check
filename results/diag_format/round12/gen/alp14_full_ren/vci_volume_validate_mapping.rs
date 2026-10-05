pub open spec fn vci_volume_validate_mapping_spec(volume_id: UInt64, lba_base: UInt64, lba_top: UInt64, dpa_base: UInt64, flags: UInt64, lock_nonce: UInt64, meas_nonce: UInt64, report_nonce: UInt64, result: RsiCommandReturnCode, new_lba_base: UInt64, response: int, old_s: S, new_s: S) -> bool {
  (result == VCI_ERROR_STATE ==> new_lba_base == new_lba_base)
  && (result == VCI_ERROR_STATE ==> response == 0)
  && (result == VCI_ERROR_INPUT ==> new_lba_base == new_lba_base)
  && (result == VCI_ERROR_INPUT ==> response == 0)
  && (result == VCI_ERROR_INPUT ==> new_lba_base == new_lba_base)
  && (result == VCI_ERROR_INPUT ==> response == 0)
  && (result == VCI_ERROR_INPUT ==> new_lba_base == new_lba_base)
  && (result == VCI_ERROR_INPUT ==> response == 0)
  && (result == VCI_ERROR_INPUT ==> new_lba_base == new_lba_base)
  && (result == VCI_ERROR_INPUT ==> response == 0)
  && (result == VCI_ERROR_INPUT ==> new_lba_base == new_lba_base)
  && (result == VCI_ERROR_INPUT ==> response == 0)
  && (result == VCI_ERROR_DEVICE ==> new_lba_base == new_lba_base)
  && (result == VCI_ERROR_DEVICE ==> response == 0)
  && ((!(result == VCI_ERROR_STATE) &&
       result == VCI_SUCCESS)
    ==> new_lba_base == GetWorker(new_s).device_memory_address)
  && ((!(result == VCI_ERROR_STATE) &&
       result == VCI_SUCCESS)
    ==> response == GetWorker(new_s).device_memory_response as int)
  && ((!(result == VCI_ERROR_STATE) &&
       result == VCI_SUCCESS)
    ==> response == 1)
  && (result != VCI_SUCCESS
    ==> response == 0)
  && ((result == VCI_ERROR_STATE ||
       result == VCI_ERROR_INPUT ||
       result == VCI_ERROR_INPUT ||
       result == VCI_ERROR_INPUT ||
       result == VCI_ERROR_INPUT ||
       result == VCI_ERROR_INPUT ||
       result == VCI_ERROR_INPUT ||
       result == VCI_ERROR_INPUT ||
       result == VCI_ERROR_DEVICE)
    ==> new_lba_base == new_lba_base)
}