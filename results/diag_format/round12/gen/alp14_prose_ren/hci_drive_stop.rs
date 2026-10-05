pub open spec fn hci_drive_stop_spec(drive_ptr: Address, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  (result == HCI_ERROR_NOT_SUPPORTED ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_NOT_SUPPORTED ==> Driveat(new_s, drive_ptr).comm_state == Driveat(old_s, drive_ptr).comm_state)
  && ((!(drive_ptr % size_of::<Extent>() == 0) ||
       !(IsEnrollablePhysicalAddress(drive_ptr)) ||
       !(Driveat(old_s, drive_ptr).state == DRIVE))
    ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_DEVICE ==> result == HCI_ERROR_DEVICE)
  && (result == HCI_ERROR_DEVICE ==> result == HCI_ERROR_DEVICE)
  && (result == HCI_SUCCESS ==> Driveat(new_s, drive_ptr).state == DRIVE_STOPPING)
  && (result == HCI_SUCCESS ==> Driveat(new_s, drive_ptr).comm_state == DEV_COMM_PENDING)
  && ((result == HCI_SUCCESS || result == HCI_ERROR_INPUT || result == HCI_ERROR_DEVICE)
    ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result != HCI_SUCCESS
    ==> Driveat(new_s, drive_ptr).comm_state == Driveat(old_s, drive_ptr).comm_state)
}