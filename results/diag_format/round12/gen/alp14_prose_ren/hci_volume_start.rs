pub open spec fn hci_volume_start_spec(vd: Address, volume_ptr: Address, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  (result == HCI_ERROR_NOT_SUPPORTED ==> Vaultat(new_s, vd) == Vaultat(old_s, vd))
  && (result == HCI_ERROR_NOT_SUPPORTED ==> Volumeat(new_s, volume_ptr) == Volumeat(old_s, volume_ptr))
  && ((!(vd % EXTENT_SIZE == 0) ||
       !(is_enrollable_physical_address(vd) || true) ||
       !(Extentat(new_s, vd).state == VD) ||
       !(volume_ptr % EXTENT_SIZE == 0) ||
       !(is_enrollable_physical_address(volume_ptr) || true) ||
       !(Extentat(new_s, volume_ptr).state == VOLUME) ||
       !(Volumeat(new_s, volume_ptr).vault == Vaultat(new_s, vd)))
    ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_DEVICE ==> result == HCI_ERROR_DEVICE)
  && (result == HCI_ERROR_DEVICE ==> result == HCI_ERROR_DEVICE)
  && (result == HCI_SUCCESS ==> Volumeat(new_s, volume_ptr).op == VOLUME_OP_START)
  && (result == HCI_SUCCESS ==> Volumeat(new_s, volume_ptr).comm_state == DEV_COMM_PENDING)
  && ((result == HCI_SUCCESS &&
       Volumeat(new_s, volume_ptr).op == VOLUME_OP_START &&
       Volumeat(new_s, volume_ptr).comm_state == DEV_COMM_PENDING)
    ==> Volumeat(new_s, volume_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       (vd % EXTENT_SIZE == 0) &&
       is_enrollable_physical_address(vd) &&
       Extentat(new_s, vd).state == VD &&
       (volume_ptr % EXTENT_SIZE == 0) &&
       is_enrollable_physical_address(volume_ptr) &&
       Extentat(new_s, volume_ptr).state == VOLUME &&
       Volumeat(new_s, volume_ptr).vault == Vaultat(new_s, vd))
    ==> result == HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> Volumeat(new_s, volume_ptr).op == Volumeat(old_s, volume_ptr).op)
  && (result != HCI_SUCCESS
    ==> Volumeat(new_s, volume_ptr).comm_state == Volumeat(old_s, volume_ptr).comm_state)
  && (result != HCI_SUCCESS
    ==> Vaultat(new_s, vd) == Vaultat(old_s, vd))
  && (result != HCI_SUCCESS
    ==> Volumeat(new_s, volume_ptr) == Volumeat(old_s, volume_ptr))
}