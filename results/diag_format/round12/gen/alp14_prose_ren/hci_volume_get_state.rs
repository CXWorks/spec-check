pub open spec fn hci_volume_get_state_spec(volume_ptr: Address, result: Hcicommandreturncode, state: Hcivolumestate, old_s: S, new_s: S) -> bool {
  (DeviceAssignmentSupported(old_s) ==> result == HCI_ERROR_NOT_SUPPORTED)
  && (!DeviceAssignmentSupported(old_s)
    ==> result == HCI_SUCCESS)
  && (result == HCI_SUCCESS
    ==> Volumeat(new_s, volume_ptr).state == state)
  && ((!(volume_ptr % size_of::<Extent>() == 0) ||
       !IsPhysicalAddressEnrollable(old_s, volume_ptr) ||
       !(Volumeat(old_s, volume_ptr).state == VOLUME))
    ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_INPUT
    ==> result == HCI_ERROR_INPUT)
  && ((DeviceAssignmentSupported(old_s) &&
       (volume_ptr % size_of::<Extent>() == 0) &&
       IsPhysicalAddressEnrollable(old_s, volume_ptr) &&
       !(Volumeat(old_s, volume_ptr).state == VOLUME))
    ==> result == HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> Volumeat(new_s, volume_ptr).state == Volumeat(old_s, volume_ptr).state)
}