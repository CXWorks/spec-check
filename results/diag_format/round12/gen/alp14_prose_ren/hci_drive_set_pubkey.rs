pub open spec fn hci_drive_set_pubkey_spec(drive_ptr: Address, params_ptr: Address, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  (result == HCI_ERROR_NOT_SUPPORTED ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_NOT_SUPPORTED ==> Driveat(new_s, drive_ptr).comm_state == Driveat(old_s, drive_ptr).comm_state)
  && (result == HCI_ERROR_INPUT ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_INPUT ==> Driveat(new_s, drive_ptr).comm_state == Driveat(old_s, drive_ptr).comm_state)
  && (result == HCI_ERROR_INPUT ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_INPUT ==> Driveat(new_s, drive_ptr).comm_state == Driveat(old_s, drive_ptr).comm_state)
  && (result == HCI_ERROR_INPUT ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_INPUT ==> Driveat(new_s, drive_ptr).comm_state == Driveat(old_s, drive_ptr).comm_state)
  && (result == HCI_ERROR_INPUT ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_INPUT ==> Driveat(new_s, drive_ptr).comm_state == Driveat(old_s, drive_ptr).comm_state)
  && (result == HCI_ERROR_INPUT ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_INPUT ==> Driveat(new_s, drive_ptr).comm_state == Driveat(old_s, drive_ptr).comm_state)
  && (result == HCI_ERROR_INPUT ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_INPUT ==> Driveat(new_s, drive_ptr).comm_state == Driveat(old_s, drive_ptr).comm_state)
  && (result == HCI_ERROR_DEVICE ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_DEVICE ==> Driveat(new_s, drive_ptr).comm_state == Driveat(old_s, drive_ptr).comm_state)
  && (result == HCI_SUCCESS ==> Driveat(new_s, drive_ptr).state == DRIVE_HAS_KEY)
  && (result == HCI_SUCCESS ==> Driveat(new_s, drive_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(DeviceAssignmentSupported(old_s)) &&
       !(drive_ptr % size_of::<Extent>() == 0) &&
       !(is_enrollable_physical_address(old_s, drive_ptr)) &&
       !(Extentat(old_s, drive_ptr).state == DRIVE) &&
       !(params_ptr % size_of::<Extent>() == 0) &&
       !(Extentat(old_s, params_ptr).accessible_in_non_secure_physical_address_space(old_s)) &&
       !(Hcipublickeyparamsat(old_s, params_ptr).key_length > 1024) &&
       !(Hcipublickeyparamsat(old_s, params_ptr).metadata_length > 1024) &&
       !(is_key_invalid(old_s, Hcipublickeyparamsat(old_s, params_ptr).key_length, Hcipublickeyparamsat(old_s, params_ptr).metadata_length)) &&
       !(is_metadata_invalid(old_s, Hcipublickeyparamsat(old_s, params_ptr).metadata_length)) &&
       !(Driveat(old_s, drive_ptr).state == DRIVE_NEEDS_KEY))
    ==> result == HCI_ERROR_NOT_SUPPORTED)
  && (result == HCI_SUCCESS &&
       !(DeviceAssignmentSupported(old_s)) &&
       !(drive_ptr % size_of::<Extent>() == 0) &&
       !(is_enrollable_physical_address(old_s, drive_ptr)) &&
       !(Extentat(old_s, drive_ptr).state == DRIVE) &&
       !(params_ptr % size_of::<Extent>() == 0) &&
       !(Extentat(old_s, params_ptr).accessible_in_non_secure_physical_address_space(old_s)) &&
       !(Hcipublickeyparamsat(old_s, params_ptr).key_length > 1024) &&
       !(Hcipublickeyparamsat(old_s, params_ptr).metadata_length > 1024) &&
       !(is_key_invalid(old_s, Hcipublickeyparamsat(old_s, params_ptr).key_length, Hcipublickeyparamsat(old_s, params_ptr).metadata_length)) &&
       !(is_metadata_invalid(old_s, Hcipublickeyparamsat(old_s, params_ptr).metadata_length)) &&
       !(Driveat(old_s, drive_ptr).state == DRIVE_NEEDS_KEY))
    ==> result == HCI_ERROR_NOT_SUPPORTED)
  && (result != HCI_SUCCESS &&
       Driveat(old_s, drive_ptr).state == DRIVE_NEEDS_KEY)
    ==> Driveat(new_s, drive_ptr).state == DRIVE_NEEDS_KEY)
  && (result != HCI_SUCCESS &&
       Driveat(old_s, drive_ptr).comm_state == DEV_COMM_PENDING)
    ==> Driveat(new_s, drive_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result == HCI_SUCCESS)
    ==> Driveat(new_s, drive_ptr).state == DRIVE_HAS_KEY)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result == HCI_SUCCESS)
    ==> Driveat(new_s, drive_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_SUCCESS)
    ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_SUCCESS)
    ==> Driveat(new_s, drive_ptr).comm_state == Driveat(old_s, drive_ptr).comm_state)
  && (result == HCI_SUCCESS
    ==> Hcipublickeyparamsat(new_s, params_ptr).key_length == Hcipublickeyparamsat(old_s, params_ptr).key_length)
  && (result == HCI_SUCCESS
    ==> Hcipublickeyparamsat(new_s, params_ptr).metadata_length == Hcipublickeyparamsat(old_s, params_ptr).metadata_length)
}