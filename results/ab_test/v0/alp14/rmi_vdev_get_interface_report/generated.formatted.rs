pub open spec fn RMI_VDEV_GET_INTERFACE_REPORT_spec(s: S, rd: Address, vdev_ptr: Address) -> bool {
    let realm = RealmAt(s, rd);
    let vdev = VdevAt(s, vdev_ptr);

    (
    // Failure condition: da_supp
    (ImplFeatures(s).feat_da != FEATURE_TRUE ==> ResultEqual(
        RMI_ERROR_NOT_SUPPORTED,
        RMI_ERROR_NOT_SUPPORTED,
    )) &&
    // Failure condition: rd_align
    (!AddrIsGranuleAligned(s, rd) ==> ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT))
        &&
    // Failure condition: rd_bound
    (!PaIsDelegable(s, rd) ==> ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT))
        &&
    // Failure condition: rd_state
    (GranuleAt(s, rd).state != RD ==> ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT))
        &&
    // Failure condition: vdev_align
    (!AddrIsGranuleAligned(s, vdev_ptr) ==> ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT))
        &&
    // Failure condition: vdev_bound
    (!PaIsDelegable(s, vdev_ptr) ==> ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT))
        &&
    // Failure condition: vdev_gran_state
    (GranuleAt(s, vdev_ptr).state != VDEV ==> ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT))
        &&
    // Failure condition: vdev_realm
    (vdev.realm != rd ==> ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT))
        &&
    // Failure condition: vdev_state
    ((vdev.vdev_state != VDEV_LOCKED && vdev.vdev_state != VDEV_STARTED) ==> ResultEqual(
        RMI_ERROR_DEVICE,
        RMI_ERROR_DEVICE,
    )) &&
    // Failure condition: comm_state
    (vdev.comm_state != DEV_COMM_IDLE ==> ResultEqual(RMI_ERROR_DEVICE, RMI_ERROR_DEVICE))
        &&
    // Success condition: op
    ((ImplFeatures(s).feat_da == FEATURE_TRUE && AddrIsGranuleAligned(s, rd) && PaIsDelegable(s, rd)
        && GranuleAt(s, rd).state == RD && AddrIsGranuleAligned(s, vdev_ptr) && PaIsDelegable(
        s,
        vdev_ptr,
    ) && GranuleAt(s, vdev_ptr).state == VDEV && vdev.realm == rd && (vdev.vdev_state == VDEV_LOCKED
        || vdev.vdev_state == VDEV_STARTED) && vdev.comm_state == DEV_COMM_IDLE) ==> vdev.op
        == VDEV_OP_GET_REPORT) &&
    // Success condition: comm_state
    ((ImplFeatures(s).feat_da == FEATURE_TRUE && AddrIsGranuleAligned(s, rd) && PaIsDelegable(s, rd)
        && GranuleAt(s, rd).state == RD && AddrIsGranuleAligned(s, vdev_ptr) && PaIsDelegable(
        s,
        vdev_ptr,
    ) && GranuleAt(s, vdev_ptr).state == VDEV && vdev.realm == rd && (vdev.vdev_state == VDEV_LOCKED
        || vdev.vdev_state == VDEV_STARTED) && vdev.comm_state == DEV_COMM_IDLE) ==> vdev.comm_state
        == DEV_COMM_PENDING))
}