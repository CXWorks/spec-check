pub open spec fn hci_volume_lock_spec(vd: Address, volume_ptr: Address, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  (result == HCI_ERROR_NOT_SUPPORTED ==> Vaultat(new_s, vd).volume_state == VOLUME_UNLOCKED)
  && (result == HCI_ERROR_NOT_SUPPORTED ==> Vaultat(new_s, vd).comm_state == DEV_COMM_IDLE)
  && (result == HCI_ERROR_INPUT ==> Vaultat(new_s, vd).volume_state == VOLUME_UNLOCKED)
  && (result == HCI_ERROR_INPUT ==> Vaultat(new_s, vd).comm_state == DEV_COMM_IDLE)
  && (result == HCI_ERROR_INPUT ==> Vaultat(new_s, vd).volume_state == VOLUME_UNLOCKED)
  && (result == HCI_ERROR_INPUT ==> Vaultat(new_s, vd).comm_state == DEV_COMM_IDLE)
  && (result == HCI_ERROR_INPUT ==> Vaultat(new_s, vd).volume_state == VOLUME_UNLOCKED)
  && (result == HCI_ERROR_INPUT ==> Vaultat(new_s, vd).comm_state == DEV_COMM_IDLE)
  && (result == HCI_ERROR_INPUT ==> Vaultat(new_s, vd).volume_state == VOLUME_UNLOCKED)
  && (result == HCI_ERROR_INPUT ==> Vaultat(new_s, vd).comm_state == DEV_COMM_IDLE)
  && (result == HCI_ERROR_DEVICE ==> Vaultat(new_s, vd).volume_state == VOLUME_UNLOCKED)
  && (result == HCI_ERROR_DEVICE ==> Vaultat(new_s, vd).comm_state == DEV_COMM_IDLE)
  && (result == HCI_ERROR_DEVICE ==> Vaultat(new_s, vd).volume_state == VOLUME_UNLOCKED)
  && (result == HCI_ERROR_DEVICE ==> Vaultat(new_s, vd).comm_state == DEV_COMM_IDLE)
  && (result == HCI_SUCCESS ==> Vaultat(new_s, vd).volume_state == VOLUME_LOCKED)
  && (result == HCI_SUCCESS ==> Vaultat(new_s, vd).comm_state == DEV_COMM_PENDING)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result == HCI_SUCCESS)
    ==> Vaultat(new_s, vd).volume_state == VOLUME_LOCKED)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result == HCI_SUCCESS)
    ==> Vaultat(new_s, vd).comm_state == DEV_COMM_PENDING)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_SUCCESS)
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_SUCCESS)
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_NOT_SUPPORTED
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_NOT_SUPPORTED
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && ((!(result == HCI_ERROR_INPUT) &&
       result == HCI_SUCCESS)
    ==> Vaultat(new_s, vd).volume_state == VOLUME_LOCKED)
  && ((!(result == HCI_ERROR_INPUT) &&
       result == HCI_SUCCESS)
    ==> Vaultat(new_s, vd).comm_state == DEV_COMM_PENDING)
  && ((!(result == HCI_ERROR_INPUT) &&
       result != HCI_SUCCESS)
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && ((!(result == HCI_ERROR_INPUT) &&
       result != HCI_SUCCESS)
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_INPUT
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_INPUT
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state == Vaultat(old_s, vd).comm_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).volume_state == Vaultat(old_s, vd).volume_state)
  && (result != HCI_SUCCESS && result != HCI_ERROR_DEVICE
    ==> Vaultat(new_s, vd).comm_state ==