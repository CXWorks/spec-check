pub open spec fn hci_volume_get_interface_report_spec(vd: Vd, volume_ptr: VolumePtr, result: Result<(), HciStatusCode>, old_s: S, new_s: S) -> bool {
  (DeviceAssignmentSupported(old_s) ==> result == HCI_ERROR_NOT_SUPPORTED)
  && ((!(VdCanBeEnrolled(old_s, vd)) || !(ExtentAtVdIsInVdState(old_s, vd)) || !(VolumePtrCanBeEnrolled(old_s, volume_ptr)) || !(ExtentAtVolumePtrIsInVolumeState(old_s, volume_ptr))) ==> result == HCI_ERROR_INPUT)
  && (VdCanBeEnrolled(old_s, vd) && ExtentAtVdIsInVdState(old_s, vd) && VolumePtrCanBeEnrolled(old_s, volume_ptr) && ExtentAtVolumePtrIsInVolumeState(old_s, volume_ptr) ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_INPUT ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_DEVICE ==> result == HCI_ERROR_DEVICE)
  && (result == HCI_ERROR_DEVICE ==> result == HCI_ERROR_DEVICE)
  && (result.is_Ok() ==> VolumePendingOperation(new_s, volume_ptr) == VOLUME_OP_GET_REPORT)
  && (result.is_Ok() ==> VolumeCommunicationState(new_s, volume_ptr) == DEV_COMM_PENDING)
  && ((!(DeviceAssignmentSupported(old_s)) &&
       VdCanBeEnrolled(old_s, vd) &&
       ExtentAtVdIsInVdState(old_s, vd) &&
       VolumePtrCanBeEnrolled(old_s, volume_ptr) &&
       ExtentAtVolumePtrIsInVolumeState(old_s, volume_ptr))
    ==> result == HCI_ERROR_INPUT)
  && (result.is_Err()
    ==> VolumePendingOperation(new_s, volume_ptr) == VolumePendingOperation(old_s, volume_ptr))
  && (result.is_Err()
    ==> VolumeCommunicationState(new_s, volume_ptr) == VolumeCommunicationState(old_s, volume_ptr))
}