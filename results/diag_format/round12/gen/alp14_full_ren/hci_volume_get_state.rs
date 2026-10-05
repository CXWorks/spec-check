pub open spec fn hci_volume_get_state_spec(volume_ptr: UInt64, result: Result<(), HciStatusCode>, state: UInt8, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_device_assignment == FEATURE_FALSE ==> ResultEqual(result, HCI_ERROR_NOT_SUPPORTED))
  && ((volume_ptr % size_of::<Extent>() != 0) ==> ResultEqual(result, HCI_ERROR_INPUT))
  && (result == HCI_ERROR_INPUT ==> ResultEqual(result, HCI_ERROR_INPUT))
  && (result == HCI_ERROR_INPUT ==> ResultEqual(result, HCI_ERROR_INPUT))
  && (result == HCI_ERROR_INPUT ==> ResultEqual(result, HCI_ERROR_INPUT))
  && (result.is_Ok() ==> state == VOLUME_STATE(new_s, volume_ptr))
  && ((!(ImplFeatures(old_s).feat_device_assignment == FEATURE_FALSE) &&
       !(volume_ptr % size_of::<Extent>() != 0) &&
       !(result == HCI_ERROR_INPUT) &&
       !(result == HCI_ERROR_INPUT))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> state == 0)
  && (result == HCI_ERROR_INPUT
    ==> state == 0)
  && (result == HCI_ERROR_INPUT
    ==> state == 0)
  && (result == HCI_ERROR_INPUT
    ==> state == 0)
}