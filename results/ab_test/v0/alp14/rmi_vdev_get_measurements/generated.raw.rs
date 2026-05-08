```verus
pub open spec fn RMI_VDEV_GET_MEASUREMENTS_spec(s: S, rd: Address, vdev_ptr: Address, params_ptr: Address, result: RmiCommandReturnCode) -> bool {
  let realm = RealmAt(s, rd);
  let vdev = VdevAt(s, vdev_ptr);
  let params = RmiVdevMeasureParamsAt(s, params_ptr);
  
  (
    // Failure conditions
    (!ImplFeatures(s).feat_da_enabled() ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) &&
    (!AddrIsGranuleAligned(s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!AddrIsGranuleAligned(s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (vdev.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((vdev.vdev_state != VDEV_LOCKED && vdev.vdev_state != VDEV_STARTED) ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    (vdev.comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    (!AddrIsGranuleAligned(s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!GranuleAccessPermitted(s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((params.indices[0] == 1 || params.indices[255] == 1) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Success conditions (when no failures apply)
    (result.is_Ok() ==> (
      vdev.op == VDEV_OP_GET_MEAS &&
      vdev.comm_state == DEV_COMM_PENDING
    ))
  )
}
```