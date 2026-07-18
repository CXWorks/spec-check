pub open spec fn rmi_pdev_communicate_spec(pdev_ptr: Address, data_ptr: Address, result: Result<RmiCommandReturnCode, _>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, data_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, data_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.req_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.req_addr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.resp_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.resp_addr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RmiDevCommDataAt(old_s, data_ptr).enter.resp_len > RMM_GRANULE_SIZE ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((PdevAt(old_s, pdev_ptr).comm_state == DEV_COMM_IDLE || PdevAt(old_s, pdev_ptr).comm_state == DEV_COMM_ERROR) ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).comm_state == DeviceCommunicate(new_s, PdevAt(new_s, pdev_ptr), RmiDevCommDataAt(new_s, data_ptr)))
  && (result.is_Ok() && (DeviceCommunicate(new_s, PdevAt(new_s, pdev_ptr), RmiDevCommDataAt(new_s, data_ptr)) == DEV_COMM_ERROR && PdevAt(new_s, pdev_ptr).state != PDEV_STOPPING) ==> PdevAt(new_s, pdev_ptr).state == PDEV_ERROR)
  && (result.is_Ok() && (DeviceCommunicate(new_s, PdevAt(new_s, pdev_ptr), RmiDevCommDataAt(new_s, data_ptr)) == DEV_COMM_IDLE && PdevAt(new_s, pdev_ptr).state_pre == PDEV_NEW) ==> PdevAt(new_s, pdev_ptr).state == PDEV_NEEDS_KEY)
  && (result.is_Ok() && (DeviceCommunicate(new_s, PdevAt(new_s, pdev_ptr), RmiDevCommDataAt(new_s, data_ptr)) == DEV_COMM_IDLE && PdevAt(new_s, pdev_ptr).state_pre == PDEV_HAS_KEY) ==> PdevAt(new_s, pdev_ptr).state == PDEV_READY)
  && (result.is_Ok() && (DeviceCommunicate(new_s, PdevAt(new_s, pdev_ptr), RmiDevCommDataAt(new_s, data_ptr)) == DEV_COMM_IDLE && PdevAt(new_s, pdev_ptr).state_pre == PDEV_READY) ==> PdevAt(new_s, pdev_ptr).state == PDEV_READY)
  && (result.is_Ok() && (DeviceCommunicate(new_s, PdevAt(new_s, pdev_ptr), RmiDevCommDataAt(new_s, data_ptr)) != DEV_COMM_ACTIVE && PdevAt(new_s, pdev_ptr).state_pre == PDEV_STOPPING) ==> PdevAt(new_s, pdev_ptr).state == PDEV_STOPPED)
  && (result.is_Ok() && (DeviceCommunicate(new_s, PdevAt(new_s, pdev_ptr), RmiDevCommDataAt(new_s, data_ptr)) == DEV_COMM_IDLE && PdevAt(new_s, pdev_ptr).state_pre == PDEV_COMMUNICATING) ==> PdevAt(new_s, pdev_ptr).state == PDEV_READY)
  && (result.is_Ok() && (DeviceCommunicate(new_s, PdevAt(new_s, pdev_ptr), RmiDevCommDataAt(new_s, data_ptr)) == DEV_COMM_IDLE && PdevAt(new_s, pdev_ptr).state_pre == PDEV_IDE_RESETTING) ==> PdevAt(new_s, pdev_ptr).state == PDEV_READY)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV) &&
       AddrIsGranuleAligned(old_s, data_ptr) &&
       GranuleAccessPermitted(old_s, data_ptr, PAS_NS) &&
       AddrIsGranuleAligned(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.req_addr) &&
       GranuleAccessPermitted(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.req_addr, PAS_NS) &&
       AddrIsGranuleAligned(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.resp_addr) &&
       GranuleAccessPermitted(old_s, RmiDevCommDataAt(old_s, data_ptr).enter.resp_addr, PAS_NS) &&
       !(RmiDevCommDataAt(old_s, data_ptr).enter.resp_len > RMM_GRANULE_SIZE) &&
       !((PdevAt(old_s, pdev_ptr).comm_state == DEV_COMM_IDLE || PdevAt(old_s, pdev_ptr).comm_state == DEV_COMM_ERROR)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
}