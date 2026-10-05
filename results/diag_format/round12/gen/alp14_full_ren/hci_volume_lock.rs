pub open spec fn hci_volume_lock_spec(vd: Address, volume_ptr: Address, result: HciCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == HCI_ERROR_NOT_SUPPORTED ==> VOLUMEAt(new_s, volume_ptr).pending_operation == VOLUME_OP_NONE)
  && (result == HCI_ERROR_NOT_SUPPORTED ==> VOLUMEAt(new_s, volume_ptr).comm_state == DEV_COMM_IDLE)
  && (result == HCI_ERROR_INPUT ==> VOLUMEAt(new_s, volume_ptr).pending_operation == VOLUME_OP_NONE)
  && (result == HCI_ERROR_INPUT ==> VOLUMEAt(new_s, volume_ptr).comm_state == DEV_COMM_IDLE)
  && (result == HCI_ERROR_INPUT ==> VOLUMEAt(new_s, volume_ptr).pending_operation == VOLUME_OP_NONE)
  && (result == HCI_ERROR_INPUT ==> VOLUMEAt(new_s, volume_ptr).comm_state == DEV_COMM_IDLE)
  && (result == HCI_ERROR_INPUT ==> VOLUMEAt(new_s, volume_ptr).pending_operation == VOLUME_OP_NONE)
  && (result == HCI_ERROR_INPUT ==> VOLUMEAt(new_s, volume_ptr).comm_state == DEV_COMM_IDLE)
  && (result == HCI_ERROR_INPUT ==> VOLUMEAt(new_s, volume_ptr).pending_operation == VOLUME_OP_NONE)
  && (result == HCI_ERROR_INPUT ==> VOLUMEAt(new_s, volume_ptr).comm_state == DEV_COMM_IDLE)
  && (result == HCI_ERROR_DEVICE ==> VOLUMEAt(new_s, volume_ptr).pending_operation == VOLUME_OP_NONE)
  && (result == HCI_ERROR_DEVICE ==> VOLUMEAt(new_s, volume_ptr).comm_state == DEV_COMM_IDLE)
  && (result == HCI_ERROR_DEVICE ==> VOLUMEAt(new_s, volume_ptr).pending_operation == VOLUME_OP_NONE)
  && (result == HCI_ERROR_DEVICE ==> VOLUMEAt(new_s, volume_ptr).comm_state == DEV_COMM_IDLE)
  && (result == HCI_SUCCESS ==> VOLUMEAt(new_s, volume_ptr).pending_operation == VOLUME_OP_LOCK)
  && (result == HCI_SUCCESS ==> VOLUMEAt(new_s, volume_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result == HCI_SUCCESS)
    ==> VOLUMEAt(new_s, volume_ptr).pending_operation == VOLUME_OP_LOCK)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result == HCI_SUCCESS)
    ==> VOLUMEAt(new_s, volume_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_SUCCESS)
    ==> VOLUMEAt(new_s, volume_ptr).pending_operation == VOLUMEAt(old_s, volume_ptr).pending_operation)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result != HCI_SUCCESS)
    ==> VOLUMEAt(new_s, volume_ptr).comm_state == VOLUMEAt(old_s, volume_ptr).comm_state)
  && (result != HCI_SUCCESS
    ==> VOLUMEAt(new_s, volume_ptr).pending_operation == VOLUMEAt(old_s, volume_ptr).pending_operation)
  && (result != HCI_SUCCESS
    ==> VOLUMEAt(new_s, volume_ptr).comm_state == VOLUMEAt(old_s, volume_ptr).comm_state)
}