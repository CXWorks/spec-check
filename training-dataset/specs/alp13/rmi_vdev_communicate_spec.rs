pub open spec fn rmi_vdev_communicate_spec(pdev_ptr: Address, vdev_ptr: Address, data_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, data_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, data_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.req_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.req_addr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.resp_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.resp_addr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RmiDevCommDataAt(old_s, data_ptr).enter.resp_len > RMM_GRANULE_SIZE ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_ptr).pdev != pdev_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DeviceCommunicate2(new_s, VdevAt(new_s, vdev_ptr), RmiDevCommDataAt(new_s, data_ptr)))
  && (result.is_Ok() && DeviceCommunicate2(old_s, VdevAt(old_s, vdev_ptr), RmiDevCommDataAt(old_s, data_ptr)) == DEV_COMM_ERROR ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_ERROR)
  && (result.is_Ok() && (DeviceCommunicate2(old_s, VdevAt(old_s, vdev_ptr), RmiDevCommDataAt(old_s, data_ptr)) == DEV_COMM_IDLE && VdevAt(old_s, vdev_ptr).op == VDEV_OP_UNLOCK) ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_UNLOCKED)
  && (result.is_Ok() && (DeviceCommunicate2(old_s, VdevAt(old_s, vdev_ptr), RmiDevCommDataAt(old_s, data_ptr)) == DEV_COMM_IDLE && VdevAt(old_s, vdev_ptr).op == VDEV_OP_LOCK) ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_LOCKED)
  && (result.is_Ok() && (DeviceCommunicate2(old_s, VdevAt(old_s, vdev_ptr), RmiDevCommDataAt(old_s, data_ptr)) == DEV_COMM_IDLE && VdevAt(old_s, vdev_ptr).op == VDEV_OP_LOCK) ==> VdevAt(new_s, vdev_ptr).lock_nonce == VdevGenerateNonce(new_s, VdevAt(new_s, vdev_ptr)))
  && (result.is_Ok() && (DeviceCommunicate2(old_s, VdevAt(old_s, vdev_ptr), RmiDevCommDataAt(old_s, data_ptr)) == DEV_COMM_IDLE && VdevAt(old_s, vdev_ptr).op == VDEV_OP_START) ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_STARTED)
  && (result.is_Ok() && (DeviceCommunicate2(old_s, VdevAt(old_s, vdev_ptr), RmiDevCommDataAt(old_s, data_ptr)) == DEV_COMM_IDLE && VdevAt(old_s, vdev_ptr).op == VDEV_OP_GET_MEAS) ==> VdevAt(new_s, vdev_ptr).meas_nonce == VdevGenerateNonce(new_s, VdevAt(new_s, vdev_ptr)))
  && (result.is_Ok() && (DeviceCommunicate2(old_s, VdevAt(old_s, vdev_ptr), RmiDevCommDataAt(old_s, data_ptr)) == DEV_COMM_IDLE && VdevAt(old_s, vdev_ptr).op == VDEV_OP_GET_REPORT) ==> VdevAt(new_s, vdev_ptr).report_nonce == VdevGenerateNonce(new_s, VdevAt(new_s, vdev_ptr)))
  && (result.is_Ok() && DeviceCommunicate2(old_s, VdevAt(old_s, vdev_ptr), RmiDevCommDataAt(old_s, data_ptr)) == DEV_COMM_IDLE ==> VdevAt(new_s, vdev_ptr).op == VDEV_OP_NONE)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV) &&
       AddrIsGranuleAligned(old_s, vdev_ptr) &&
       PaIsDelegable(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != VDEV) &&
       AddrIsGranuleAligned(old_s, data_ptr) &&
       GranuleAccessPermitted(old_s, data_ptr, PAS_NS) &&
       AddrIsGranuleAligned(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.req_addr) &&
       GranuleAccessPermitted(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.req_addr, PAS_NS) &&
       AddrIsGranuleAligned(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.resp_addr) &&
       GranuleAccessPermitted(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.resp_addr, PAS_NS) &&
       !(RmiDevCommDataAt(old_s, data_ptr).enter.resp_len > RMM_GRANULE_SIZE) &&
       !(VdevAt(old_s, vdev_ptr).pdev != pdev_ptr) &&
       !(VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).lock_nonce == VdevAt(old_s, vdev_ptr).lock_nonce)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).meas_nonce == VdevAt(old_s, vdev_ptr).meas_nonce)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).report_nonce == VdevAt(old_s, vdev_ptr).report_nonce)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).op == VdevAt(old_s, vdev_ptr).op)
  && (!(result.is_Ok() && (DeviceCommunicate2(old_s, VdevAt(old_s, vdev_ptr), RmiDevCommDataAt(old_s, data_ptr)) == DEV_COMM_ERROR)) ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
}
