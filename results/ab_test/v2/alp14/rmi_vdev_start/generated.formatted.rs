pub open spec fn rmi_vdev_start_spec(
    result: RmiCommandReturnCode,
    old_s: S,
    new_s: S,
    rd: Address,
    vdev_ptr: Address,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let vdev = VdevAt(old_s, vdev_ptr);

    // Failure condition: da_supp
    &&(!ImplFeatures(old_s).feat_da.is_FEATURE_TRUE() ==> result
        == RMI_ERROR_NOT_SUPPORTED)
    // Failure condition: rd_align
     && (!AddrIsGranuleAligned(old_s, rd) ==> result
        == RMI_ERROR_INPUT)
    // Failure condition: rd_bound
     && (!PaIsDelegable(old_s, rd) ==> result
        == RMI_ERROR_INPUT)
    // Failure condition: rd_state
     && (GranuleAt(old_s, rd).state != RD ==> result
        == RMI_ERROR_INPUT)
    // Failure condition: vdev_align
     && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> result
        == RMI_ERROR_INPUT)
    // Failure condition: vdev_bound
     && (!PaIsDelegable(old_s, vdev_ptr) ==> result
        == RMI_ERROR_INPUT)
    // Failure condition: vdev_gran_state
     && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> result
        == RMI_ERROR_INPUT)
    // Failure condition: vdev_realm
     && (vdev.realm != rd ==> result
        == RMI_ERROR_INPUT)
    // Failure condition: vdev_state
     && (vdev.vdev_state != VDEV_LOCKED ==> result
        == RMI_ERROR_DEVICE)
    // Failure condition: comm_state
     && (vdev.comm_state != DEV_COMM_IDLE ==> result
        == RMI_ERROR_DEVICE)
    // Success conditions
     && ((ImplFeatures(old_s).feat_da.is_FEATURE_TRUE() && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state == RD && AddrIsGranuleAligned(
        old_s,
        vdev_ptr,
    ) && PaIsDelegable(old_s, vdev_ptr) && GranuleAt(old_s, vdev_ptr).state == VDEV && vdev.realm
        == rd && vdev.vdev_state == VDEV_LOCKED && vdev.comm_state == DEV_COMM_IDLE) ==> (result
        == RMI_SUCCESS && VdevAt(new_s, vdev_ptr).op == VDEV_OP_START && VdevAt(
        new_s,
        vdev_ptr,
    ).comm_state == DEV_COMM_PENDING))
}