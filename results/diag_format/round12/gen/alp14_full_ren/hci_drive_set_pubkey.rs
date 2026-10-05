pub open spec fn hci_drive_set_pubkey_spec(drive_ptr: PhysicalAddress, params_ptr: PhysicalAddress, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_ERROR_NOT_SUPPORTED ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result == RSI_ERROR_INPUT ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result == RSI_ERROR_INPUT ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result == RSI_ERROR_INPUT ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result == RSI_ERROR_INPUT ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result == RSI_ERROR_INPUT ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result == RSI_ERROR_INPUT ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result == RSI_ERROR_INPUT ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result == RSI_ERROR_INPUT ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result == RSI_ERROR_INPUT ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result == RSI_ERROR_INPUT ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result == RSI_ERROR_DEVICE ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result == RSI_SUCCESS ==> DriveAt(new_s, drive_ptr).state == DRIVE_HAS_KEY)
  && (result == RSI_SUCCESS ==> DriveAt(new_s, drive_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(DriveAt(old_s, drive_ptr).state == DRIVE) &&
       !(DriveAt(old_s, drive_ptr).state == DRIVE_NEEDS_KEY))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result != RSI_SUCCESS
    ==> DriveAt(new_s, drive_ptr).comm_state == DriveAt(old_s, drive_ptr).comm_state)
}