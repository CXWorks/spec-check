pub open spec fn RMI_VDEV_START_spec(old_s: S, rd: Address, vdev_ptr: Address, new_s: S, result: RmiCommandReturnCode) -> bool {
    let realm = RealmAt(old_s, rd);
    let vdev = VdevAt(old_s, vdev_ptr);
    let granule_rd = GranuleAt(old_s, rd);
    let granule_vdev = GranuleAt(old_s, vdev_ptr);
    
    // Failure condition: da_supp
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) &&
    
    // Failure condition: rd_align
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: rd_bound
    (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: rd_state
    (granule_rd.state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: vdev_align
    (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: vdev_bound
    (!PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: vdev_gran_state
    (granule_vdev.state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: vdev_realm
    (vdev.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: vdev_state
    (vdev.vdev_state != VDEV_LOCKED ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    
    // Failure condition: comm_state
    (vdev.comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    
    // Success conditions
    (ImplFeatures(old_s).feat_da == FEATURE_TRUE &&
     AddrIsGranuleAligned(rd) &&
     PaIsDelegable(rd) &&
     granule_rd.state == RD &&
     AddrIsGranuleAligned(vdev_ptr) &&
     PaIsDelegable(vdev_ptr) &&
     granule_vdev.state == VDEV &&
     vdev.realm == rd &&
     vdev.vdev_state == VDEV_LOCKED &&
     vdev.comm_state == DEV_COMM_IDLE
     ==>
     (result.is_Ok() &&
      VdevAt(new_s, vdev_ptr).op == VDEV_OP_START &&
      VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING))
}