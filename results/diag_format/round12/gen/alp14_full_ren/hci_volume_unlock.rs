pub open spec fn hci_volume_unlock_spec(vd: Vd, volume_ptr: VolumePtr, result: Result<(), HciStatusCode>, old_s: S, new_s: S) -> bool {
  (DeviceAssignmentSupported(old_s) ==> result == HCI_SUCCESS)
  && (!DeviceAssignmentSupported(old_s) ==> result == HCI_ERROR_NOT_SUPPORTED)
  && (is_aligned_to(Extent::size(), vd as int) ==> result == HCI_SUCCESS)
  && (!is_aligned_to(Extent::size(), vd as int) ==> result == HCI_ERROR_INPUT)
  && (is_enrollable_address(old_s, vd) ==> result == HCI_SUCCESS)
  && (!is_enrollable_address(old_s, vd) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, vd).vd_state == VD_STATE ==> result == HCI_SUCCESS)
  && (!ExtentAt(old_s, vd).vd_state == VD_STATE ==> result == HCI_ERROR_INPUT)
  && (is_aligned_to(Extent::size(), volume_ptr as int) ==> result == HCI_SUCCESS)
  && (!is_aligned_to(Extent::size(), volume_ptr as int) ==> result == HCI_ERROR_INPUT)
  && (is_enrollable_address(old_s, volume_ptr) ==> result == HCI_SUCCESS)
  && (!is_enrollable_address(old_s, volume_ptr) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, volume_ptr).volume_state == VOLUME_STATE ==> result == HCI_SUCCESS)
  && (!ExtentAt(old_s, volume_ptr).volume_state == VOLUME_STATE ==> result == HCI_ERROR_INPUT)
  && (VOLUMEAt(old_s, volume_ptr).vault == Some(vd) ==> result == HCI_SUCCESS)
  && (!VOLUMEAt(old_s, volume_ptr).vault == Some(vd) ==> result == HCI_ERROR_INPUT)
  && (VOLUMEAt(old_s, volume_ptr).state == VOLUME_LOCKED || VOLUMEAt(old_s, volume_ptr).state == VOLUME_STARTED || VOLUMEAt(old_s, volume_ptr).state == VOLUME_ERROR ==> result == HCI_SUCCESS)
  && (!VOLUMEAt(old_s, volume_ptr).state == VOLUME_LOCKED && !VOLUMEAt(old_s, volume_ptr).state == VOLUME_STARTED && !VOLUMEAt(old_s, volume_ptr).state == VOLUME_ERROR ==> result == HCI_ERROR_DEVICE)
  && (VOLUMEAt(old_s, volume_ptr).comm_state == DEV_COMM_IDLE ==> result == HCI_SUCCESS)
  && (!VOLUMEAt(old_s, volume_ptr).comm_state == DEV_COMM_IDLE ==> result == HCI_ERROR_DEVICE)
  && (VOLUMEAt(old_s, volume_ptr).mapping_count == 0 ==> result == HCI_SUCCESS)
  && (!VOLUMEAt(old_s, volume_ptr).mapping_count == 0 ==> result == HCI_ERROR_DEVICE)
  && (result == HCI_SUCCESS ==> VOLUMEAt(new_s, volume_ptr).dma_state == VOLUME_DMA_DISABLED)
  && (result == HCI_SUCCESS ==> VOLUMEAt(new_s, volume_ptr).pending_op == VOLUME_OP_UNLOCK)
  && (result == HCI_SUCCESS ==> VOLUMEAt(new_s, volume_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(DeviceAssignmentSupported(old_s)) &&
       is_aligned_to(Extent::size(), vd as int) &&
       !is_enrollable_address(old_s, vd) &&
       !(ExtentAt(old_s, vd).vd_state == VD_STATE) &&
       is_aligned_to(Extent::size(), volume_ptr as int) &&
       is_enrollable_address(old_s, volume_ptr) &&
       !(ExtentAt(old_s, volume_ptr).volume_state == VOLUME_STATE) &&
       !(VOLUMEAt(old_s, volume_ptr).vault == Some(vd)) &&
       !((VOLUMEAt(old_s, volume_ptr).state == VOLUME_LOCKED || VOLUMEAt(old_s, volume_ptr).state == VOLUME_STARTED || VOLUMEAt(old_s, volume_ptr).state == VOLUME_ERROR)) &&
       !(VOLUMEAt(old_s, volume_ptr).comm_state == DEV_COMM_IDLE) &&
       !(VOLUMEAt(old_s, volume_ptr).mapping_count == 0))
    ==> result != HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> VOLUMEAt(new_s, volume_ptr).dma_state == VOLUMEAt(old_s, volume_ptr).dma_state)
  && (result != HCI_SUCCESS
    ==> VOLUMEAt(new_s, volume_ptr).pending_op == VOLUMEAt(old_s, volume_ptr).pending_op)
  && (result != HCI_SUCCESS
    ==> VOLUMEAt(new_s, volume_ptr).comm_state == VOLUMEAt(old_s, volume_ptr).comm_state)
  && (VOLUMEAt(new_s, volume_ptr).mapping_count == VOLUMEAt(old_s, volume_ptr).mapping_count)
}