pub open spec fn hci_volume_get_interface_report_spec(vd: Address, volume_ptr: Address, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  (result == HCI_ERROR_NOT_SUPPORTED ==> Vaultat(new_s, vd).state == VAULT_UNLOCKED)
  && (result == HCI_ERROR_NOT_SUPPORTED ==> Volumeat(new_s, volume_ptr).state == VOLUME_UNLOCKED)
  && (result == HCI_ERROR_INPUT ==> Vaultat(new_s, vd).state == VAULT_UNLOCKED)
  && (result == HCI_ERROR_INPUT ==> Volumeat(new_s, volume_ptr).state == VOLUME_UNLOCKED)
  && (result == HCI_ERROR_INPUT ==> Vaultat(new_s, vd).state == VAULT_UNLOCKED)
  && (result == HCI_ERROR_INPUT ==> Volumeat(new_s, volume_ptr).state == VOLUME_UNLOCKED)
  && (result == HCI_ERROR_INPUT ==> Vaultat(new_s, vd).state == VAULT_UNLOCKED)
  && (result == HCI_ERROR_INPUT ==> Volumeat(new_s, volume_ptr).state == VOLUME_UNLOCKED)
  && (result == HCI_ERROR_INPUT ==> Vaultat(new_s, vd).state == VAULT_UNLOCKED)
  && (result == HCI_ERROR_INPUT ==> Volumeat(new_s, volume_ptr).state == VOLUME_UNLOCKED)
  && (result == HCI_ERROR_DEVICE ==> Vaultat(new_s, vd).state == VAULT_UNLOCKED)
  && (result == HCI_ERROR_DEVICE ==> Volumeat(new_s, volume_ptr).state == VOLUME_UNLOCKED)
  && (result == HCI_ERROR_DEVICE ==> Vaultat(new_s, vd).state == VAULT_UNLOCKED)
  && (result == HCI_ERROR_DEVICE ==> Volumeat(new_s, volume_ptr).state == VOLUME_UNLOCKED)
  && (result == HCI_SUCCESS ==> Vaultat(new_s, vd).state == VAULT_UNLOCKED)
  && (result == HCI_SUCCESS ==> Volumeat(new_s, volume_ptr).state == VOLUME_UNLOCKED)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result == HCI_SUCCESS)
    ==> Volumeat(new_s, volume_ptr).op == VOLUME_OP_GET_REPORT)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result == HCI_SUCCESS)
    ==> Volumeat(new_s, volume_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result == HCI_SUCCESS)
    ==> Volumeat(new_s, volume_ptr).comm_state == DEV_COMM_PENDING)
  && ((result != HCI_ERROR_NOT_SUPPORTED &&
       result != HCI_ERROR_INPUT &&
       result != HCI_ERROR_DEVICE)
    ==> Volumeat(new_s, volume_ptr).comm_state == DEV_COMM_IDLE)
  && (result != HCI_SUCCESS
    ==> Volumeat(new_s, volume_ptr).op == Volumeat(old_s, volume_ptr).op)
  && (result != HCI_SUCCESS
    ==> Volumeat(new_s, volume_ptr).comm_state == Volumeat(old_s, volume_ptr).comm_state)
  && (result != HCI_SUCCESS
    ==> Volumeat(new_s, volume_ptr).comm_state == Volumeat(old_s, volume_ptr).comm_state)
}