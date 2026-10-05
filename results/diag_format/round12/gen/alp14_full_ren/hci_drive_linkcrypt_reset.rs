pub open spec fn hci_drive_linkcrypt_reset_spec(drive_ptr: Address, result: HciCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == HCI_ERROR_NOT_SUPPORTED ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result == HCI_ERROR_NOT_SUPPORTED ==> DriveAt(new_s, drive_ptr).comm_state == DriveAt(old_s, drive_ptr).comm_state)
  && (result == HCI_ERROR_INPUT ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result == HCI_ERROR_INPUT ==> DriveAt(new_s, drive_ptr).comm_state == DriveAt(old_s, drive_ptr).comm_state)
  && (result == HCI_ERROR_INPUT ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result == HCI_ERROR_INPUT ==> DriveAt(new_s, drive_ptr).comm_state == DriveAt(old_s, drive_ptr).comm_state)
  && (result == HCI_ERROR_INPUT ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result == HCI_ERROR_INPUT ==> DriveAt(new_s, drive_ptr).comm_state == DriveAt(old_s, drive_ptr).comm_state)
  && (result == HCI_ERROR_DEVICE ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result == HCI_ERROR_DEVICE ==> DriveAt(new_s, drive_ptr).comm_state == DriveAt(old_s, drive_ptr).comm_state)
  && (result == HCI_ERROR_DEVICE ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result == HCI_ERROR_DEVICE ==> DriveAt(new_s, drive_ptr).comm_state == DriveAt(old_s, drive_ptr).comm_state)
  && (result == HCI_SUCCESS ==> DriveAt(new_s, drive_ptr).state == DRIVE_LINKCRYPT_RESETTING)
  && (result == HCI_SUCCESS ==> DriveAt(new_s, drive_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(DeviceAssignmentSupported(old_s)) &&
       !(drive_ptr % size_of::<Extent>() == 0) &&
       !IsEnrollableAddress(old_s, drive_ptr) &&
       !(ExtentAt(old_s, drive_ptr).state == DRIVE))
    ==> result == HCI_ERROR_INPUT)
  && (result != HCI_SUCCESS && result != HCI_ERROR_NOT_SUPPORTED && result != HCI_ERROR_INPUT && result != HCI_ERROR_DEVICE ==> DriveAt(new_s, drive_ptr).state == DriveAt(old_s, drive_ptr).state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_NOT_SUPPORTED && result != HCI_ERROR_INPUT && result != HCI_ERROR_DEVICE ==> DriveAt(new_s, drive_ptr).comm_state == DriveAt(old_s, drive_ptr).comm_state)
}