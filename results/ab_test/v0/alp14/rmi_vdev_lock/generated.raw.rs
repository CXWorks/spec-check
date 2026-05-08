```verus
pub open spec fn RMI_VDEV_LOCK_spec(s: S, rd: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>) -> bool {
  let realm = RealmAt(s, rd);
  let vdev = VdevAt(s, vdev_ptr);
  
  (
    // Failure condition: da_supp
    (ImplFeatures(s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) &&
    
    // Failure condition: rd_align
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: rd_bound
    (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: rd_state
    (GranuleAt(s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: vdev_align
    (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: vdev_bound
    (!PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: vdev_gran_state
    (GranuleAt(s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: vdev_realm
    (vdev.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: vdev_state
    (vdev.vdev_state != VDEV_UNLOCKED ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    
    // Failure condition: comm_state
    (vdev.comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    
    // Success conditions
    (
      (ImplFeatures(s).feat_da == FEATURE_TRUE &&
       AddrIsGranuleAligned(rd) &&
       PaIsDelegable(rd) &&
       GranuleAt(s, rd).state == RD &&
       AddrIsGranuleAligned(vdev_ptr) &&
       PaIsDelegable(vdev_ptr) &&
       GranuleAt(s, vdev_ptr).state == VDEV &&
       vdev.realm == rd &&
       vdev.vdev_state == VDEV_UNLOCKED &&
       vdev.comm_state == DEV_COMM_IDLE) ==>
      (result.is_Ok() && vdev.op == VDEV_OP_LOCK && vdev.comm_state == DEV_COMM_PENDING)
    )
  )
}
```