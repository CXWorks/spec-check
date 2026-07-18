pub open spec fn rmi_vdev_get_measurements_spec(rd: Address, vdev_ptr: Address, params_ptr: Address, flags: RmiVdevGetMeasurementsFlags, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_ptr).realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((VdevAt(old_s, vdev_ptr).vdev_state != VDEV_LOCKED && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_STARTED) ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_ptr).comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RmiVdevMeasureParamsAt(old_s, params_ptr).indices[0] == '1' || RmiVdevMeasureParamsAt(old_s, params_ptr).indices[255] == '1') ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).op == VDEV_OP_GET_MEAS)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, vdev_ptr) &&
       PaIsDelegable(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != VDEV) &&
       !(VdevAt(old_s, vdev_ptr).realm != rd) &&
       !((VdevAt(old_s, vdev_ptr).vdev_state != VDEV_LOCKED && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_STARTED)) &&
       !(VdevAt(old_s, vdev_ptr).comm_state != DEV_COMM_IDLE) &&
       AddrIsGranuleAligned(old_s, params_ptr) &&
       GranuleAccessPermitted(old_s, params_ptr, PAS_NS) &&
       !((RmiVdevMeasureParamsAt(old_s, params_ptr).indices[0] == '1' || RmiVdevMeasureParamsAt(old_s, params_ptr).indices[255] == '1')))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).op == VdevAt(old_s, vdev_ptr).op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
}