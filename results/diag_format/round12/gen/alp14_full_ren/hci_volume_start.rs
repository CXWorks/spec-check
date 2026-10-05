pub open spec fn hci_volume_start_spec(vd: Address, volume_ptr: Address, result: Result<(), HciStatusCode>, old_s: S, new_s: S) -> bool {
  (result == HCI_ERROR_NOT_SUPPORTED ==> VOLUME_OP(new_s, volume_ptr) == VOLUME_OP_IDLE)
  && (result == HCI_ERROR_NOT_SUPPORTED ==> VOLUME_COMM_STATE(new_s, volume_ptr) == DEV_COMM_IDLE)
  && ((!(vd % EXTENT_SIZE == 0) ||
       !IsEnrollablePhysicalAddress(old_s, vd) ||
       !ExtentInVdState(old_s, vd) ||
       !(volume_ptr % EXTENT_SIZE == 0) ||
       !IsEnrollablePhysicalAddress(old_s, volume_ptr) ||
       !ExtentInVolumeState(old_s, volume_ptr) ||
       !VOLUMEBelongsToVault(old_s, volume_ptr, vd))
    ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_DEVICE ==> VOLUME_OP(new_s, volume_ptr) == VOLUME_OP_IDLE)
  && (result == HCI_ERROR_DEVICE ==> VOLUME_COMM_STATE(new_s, volume_ptr) == DEV_COMM_IDLE)
  && (result == HCI_ERROR_DEVICE ==> result == HCI_ERROR_DEVICE)
  && (result == HCI_ERROR_DEVICE ==> result == HCI_ERROR_DEVICE)
  && (result.is_Ok() ==> VOLUME_OP(new_s, volume_ptr) == VOLUME_OP_START)
  && (result.is_Ok() ==> VOLUME_COMM_STATE(new_s, volume_ptr) == DEV_COMM_PENDING)
  && ((result == HCI_SUCCESS ||
       result == HCI_ERROR_NOT_SUPPORTED ||
       result == HCI_ERROR_INPUT ||
       result == HCI_ERROR_DEVICE)
    ==> VOLUME_OP(new_s, volume_ptr) == VOLUME_OP(new_s, volume_ptr))
  && ((result == HCI_SUCCESS ||
       result == HCI_ERROR_NOT_SUPPORTED ||
       result == HCI_ERROR_INPUT ||
       result == HCI_ERROR_DEVICE)
    ==> VOLUME_COMM_STATE(new_s, volume_ptr) == VOLUME_COMM_STATE(new_s, volume_ptr))
  && (result.is_Err()
    ==> VOLUME_OP(new_s, volume_ptr) == VOLUME_OP_IDLE)
  && (result.is_Err()
    ==> VOLUME_COMM_STATE(new_s, volume_ptr) == DEV_COMM_IDLE)
}