pub open spec fn hci_drive_stop_spec(drive_ptr: PhysicalAddress, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_ERROR_NOT_SUPPORTED ==> DriveAt(new_s, drive_ptr).state == DRIVE_STOPPED)
  && (result == RSI_ERROR_INPUT ==> DriveAt(new_s, drive_ptr).state == DRIVE_STOPPED)
  && (result == RSI_ERROR_INPUT ==> DriveAt(new_s, drive_ptr).state == DRIVE_STOPPED)
  && (result == RSI_ERROR_INPUT ==> DriveAt(new_s, drive_ptr).state == DRIVE_STOPPED)
  && (result == RSI_ERROR_DEVICE ==> DriveAt(new_s, drive_ptr).state == DRIVE_STOPPED)
  && (result == RSI_ERROR_DEVICE ==> DriveAt(new_s, drive_ptr).state == DRIVE_STOPPED)
  && ((!(result == RSI_ERROR_NOT_SUPPORTED) &&
       result == RSI_SUCCESS)
    ==> DriveAt(new_s, drive_ptr).state == DRIVE_STOPPING)
  && ((!(result == RSI_ERROR_NOT_SUPPORTED) &&
       result == RSI_SUCCESS)
    ==> DriveAt(new_s, drive_ptr).comm_state == DEV_COMM_PENDING)
  && ((result != RSI_SUCCESS)
    ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && ((result != RSI_SUCCESS)
    ==> DriveAt(new_s, drive_ptr).comm_state == DriveAt(old_s, drive_ptr).comm_state)
  && ((result == RSI_SUCCESS)
    ==> DriveAt(new_s, drive_ptr).state == DRIVE_STOPPING)
  && ((result == RSI_SUCCESS)
    ==> DriveAt(new_s, drive_ptr).comm_state == DEV_COMM_PENDING)
}