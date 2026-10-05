pub open spec fn hci_volume_destroy_spec(vd: Address, drive_ptr: Address, volume_ptr: Address, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  (result == HCI_ERROR_NOT_SUPPORTED ==> Vaultat(new_s, vd).state == Vaultat(old_s, vd).state)
  && (result == HCI_ERROR_NOT_SUPPORTED ==> Volumeat(new_s, volume_ptr).state == Volumeat(old_s, volume_ptr).state)
  && (result == HCI_ERROR_NOT_SUPPORTED ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_INPUT ==> Vaultat(new_s, vd).state == Vaultat(old_s, vd).state)
  && (result == HCI_ERROR_INPUT ==> Volumeat(new_s, volume_ptr).state == Volumeat(old_s, volume_ptr).state)
  && (result == HCI_ERROR_INPUT ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_INPUT ==> Vaultat(new_s, vd).state == Vaultat(old_s, vd).state)
  && (result == HCI_ERROR_INPUT ==> Volumeat(new_s, volume_ptr).state == Volumeat(old_s, volume_ptr).state)
  && (result == HCI_ERROR_INPUT ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_INPUT ==> Vaultat(new_s, vd).state == Vaultat(old_s, vd).state)
  && (result == HCI_ERROR_INPUT ==> Volumeat(new_s, volume_ptr).state == Volumeat(old_s, volume_ptr).state)
  && (result == HCI_ERROR_INPUT ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_INPUT ==> Vaultat(new_s, vd).state == Vaultat(old_s, vd).state)
  && (result == HCI_ERROR_INPUT ==> Volumeat(new_s, volume_ptr).state == Volumeat(old_s, volume_ptr).state)
  && (result == HCI_ERROR_INPUT ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_INPUT ==> Vaultat(new_s, vd).state == Vaultat(old_s, vd).state)
  && (result == HCI_ERROR_INPUT ==> Volumeat(new_s, volume_ptr).state == Volumeat(old_s, volume_ptr).state)
  && (result == HCI_ERROR_INPUT ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_DEVICE ==> Vaultat(new_s, vd).state == Vaultat(old_s, vd).state)
  && (result == HCI_ERROR_DEVICE ==> Volumeat(new_s, volume_ptr).state == Volumeat(old_s, volume_ptr).state)
  && (result == HCI_ERROR_DEVICE ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_DEVICE ==> Vaultat(new_s, vd).state == Vaultat(old_s, vd).state)
  && (result == HCI_ERROR_DEVICE ==> Volumeat(new_s, volume_ptr).state == Volumeat(old_s, volume_ptr).state)
  && (result == HCI_ERROR_DEVICE ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_DEVICE ==> Vaultat(new_s, vd).state == Vaultat(old_s, vd).state)
  && (result == HCI_ERROR_DEVICE ==> Volumeat(new_s, volume_ptr).state == Volumeat(old_s, volume_ptr).state)
  && (result == HCI_ERROR_DEVICE ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_DEVICE ==> Vaultat(new_s, vd).state == Vaultat(old_s, vd).state)
  && (result == HCI_ERROR_DEVICE ==> Volumeat(new_s, volume_ptr).state == Volumeat(old_s, volume_ptr).state)
  && (result == HCI_ERROR_DEVICE ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_DEVICE ==> Vaultat(new_s, vd).state == Vaultat(old_s, vd).state)
  && (result == HCI_ERROR_DEVICE ==> Volumeat(new_s, volume_ptr).state == Volumeat(old_s, volume_ptr).state)
  && (result == HCI_ERROR_DEVICE ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_ERROR_DEVICE ==> Vaultat(new_s, vd).state == Vaultat(old_s, vd).state)
  && (result == HCI_ERROR_DEVICE ==> Volumeat(new_s, volume_ptr).state == Volumeat(old_s, volume_ptr).state)
  && (result == HCI_ERROR_DEVICE ==> Driveat(new_s, drive_ptr).state == Driveat(old_s, drive_ptr).state)
  && (result == HCI_SUCCESS ==> Volumeat(new_s, volume_ptr).state == ENROLLED)
  && (result == HCI_SUCCESS ==> Vaultat(new_s, vd).num_volumes == Vaultat(old_s, vd).num_volumes - 1)
  && (result == HCI_SUCCESS ==> Driveat(new_s, drive_ptr).num_volumes == Driveat(old_s, drive_ptr).num_volumes - 1)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_DEVICE) &&
       !(result == HCI_ERROR_DEVICE) &&
       !(result == HCI_ERROR_DEVICE) &&
       !(result == HCI_ERROR_DEVICE))
    ==> Volumeat(new_s, volume_ptr).state == ENROLLED)
  && (result != HCI_SUCCESS
    ==> Volumeat(new_s, volume_ptr).state == Volumeat(old_s, volume_ptr).state)
  && (result != HCI_SUCCESS
    ==> Vaultat(new_s, vd).num_volumes == Vaultat(old_s, vd).num_volumes)
  && (result != HCI_SUCCESS
    ==> Driveat(new_s, drive_ptr).num_volumes == Driveat(old_s, drive_ptr).num_volumes)
}